//! Window ownership and routing. One manager owns process/service lifetime.
use crate::{
    App, Message as ManagerMessage,
    applications::{self, Frame, runtime::Command},
    launcher::{self, Action, Focus, Launcher},
};
use iced::{Element, Event, Subscription, Task, keyboard, window};
use sei_apps::consumer::Status;
use sei_windows::launcher::Placement;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub enum Message {
    ManagerWindowCreated,
    Manager(ManagerMessage),
    Event(window::Id, Event, iced::event::Status),
    Launcher(window::Id, Action),
    Geometry(u64, Result<Placement, String>),
    Created(window::Id),
    Placed(window::Id, Result<(), String>),
    ObservedFocus(window::Id, u64, bool),
    SearchFocus(window::Id, u64, bool),
}
pub struct Desktop {
    pub manager: App,
    pub manager_id: Option<window::Id>,
    pub popup: Option<Launcher>,
    pub shared: Arc<Frame>,
    ticket: u64,
    opening: Option<(u64, Instant)>,
    closing: bool,
    closing_popup: Option<window::Id>,
    pub error: Option<String>,
    pub startup_deadline: Option<Instant>,
}
impl Desktop {
    pub fn new(manager: App) -> Self {
        Self {
            manager,
            manager_id: None,
            popup: None,
            shared: Arc::new(Frame::default()),
            ticket: 0,
            opening: None,
            closing: false,
            closing_popup: None,
            error: None,
            startup_deadline: None,
        }
    }
    pub fn title(&self, id: window::Id) -> String {
        if self.popup.as_ref().is_some_and(|p| p.id == id) {
            "Sei-Desk Launcher".into()
        } else {
            "Sei-Desk".into()
        }
    }
    pub fn theme(&self, id: window::Id) -> iced::Theme {
        if self.popup.as_ref().is_some_and(|p| p.id == id) {
            launcher::theme()
        } else {
            crate::view::theme(&self.manager)
        }
    }
    pub fn view(&self, id: window::Id) -> Element<'_, Message> {
        if let Some(p) = self.popup.as_ref().filter(|p| p.id == id) {
            return launcher::view(p, applications::busy(&self.shared.launch));
        }
        if self.manager_id.is_some_and(|manager| manager != id) {
            return iced::widget::Space::new().into();
        }
        let view = crate::view::content(&self.manager).map(Message::Manager);
        if let Some(error) = &self.error {
            iced::widget::column![
                iced::widget::text(format!("Launcher: {error}")).size(14),
                view
            ]
            .into()
        } else {
            view
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            self.manager.subscription().map(Message::Manager),
            iced::event::listen_with(|event, status, id| Some(Message::Event(id, event, status))),
        ])
    }
    fn dismiss(&mut self) -> Task<Message> {
        self.opening = None;
        self.popup.take().map_or_else(Task::none, |p| {
            self.closing_popup = Some(p.id);
            window::close(p.id)
        })
    }
    fn geometry(&mut self, require_foreground: bool) -> Task<Message> {
        let Some(id) = self.manager_id else {
            self.error = Some("Management window is unavailable.".into());
            return Task::none();
        };
        let ticket = if require_foreground {
            let Some(next) = self.ticket.checked_add(1) else {
                self.error = Some("Reopen Sei-Desk to open a launcher.".into());
                return Task::none();
            };
            self.ticket = next;
            self.opening = Some((next, Instant::now() + Duration::from_secs(3)));
            next
        } else {
            if self.opening.is_some() {
                return Task::none();
            }
            let Some(p) = &mut self.popup else {
                return Task::none();
            };
            if p.checking.is_some() {
                return Task::none();
            }
            let Some(next) = self.ticket.checked_add(1) else {
                self.error = Some("Display inspection IDs exhausted. Reopen Sei-Desk.".into());
                return self.dismiss();
            };
            self.ticket = next;
            p.checking = Some(next);
            next
        };
        window::run(id, move |w| {
            native(w, |hwnd| {
                sei_windows::launcher::inspect(hwnd, require_foreground)
            })
        })
        .map(move |result| Message::Geometry(ticket, result))
    }
    fn open(&mut self) -> Task<Message> {
        if self.closing
            || self.manager.composing
            || self.opening.is_some()
            || self.closing_popup.is_some()
        {
            return Task::none();
        }
        // Also validate current manager foreground/availability before reusing.
        self.geometry(true)
    }
    fn service_frame(&mut self, frame: Arc<Frame>) -> Task<Message> {
        if frame.revision <= self.shared.revision {
            return Task::none();
        }
        self.shared = frame.clone();
        let mut manager_frame = (*frame).clone();
        if frame.refresh_origin != 0 {
            manager_frame.refresh = self.manager.apps.frame.refresh.clone();
        }
        if frame.launch_origin != 0 {
            manager_frame.launch = self.manager.apps.frame.launch.clone();
            manager_frame.launch_name = self.manager.apps.frame.launch_name.clone();
        }
        if frame.error_origin != 0 {
            manager_frame.error = None;
            manager_frame.stale = false;
        }
        let manager_task = self
            .manager
            .update(ManagerMessage::Applications(Arc::new(manager_frame)))
            .map(Message::Manager);
        let Some(p) = &mut self.popup else {
            return manager_task;
        };
        let popup_frame = scoped(&frame, p.token);
        let changed = p.model.frame.catalog.as_ref().map(|c| c.generation)
            != popup_frame.catalog.as_ref().map(|c| c.generation);
        p.model.apply_frame(Arc::new(popup_frame));
        if let Some(seq) = p.pending_launch
            && frame.ack >= seq
        {
            if frame.error_origin == p.token && frame.error.is_some() {
                p.pending_launch = None;
            } else if frame.launch_origin == p.token {
                match frame.launch {
                    Status::Succeeded { .. } => return Task::batch([manager_task, self.dismiss()]),
                    Status::Failed { .. } | Status::Cancelled(_) => p.pending_launch = None,
                    _ => (),
                }
            }
        }
        if p.focus == Focus::Retry && p.pending_launch.is_none() && p.retry_key().is_none() {
            p.focus = Focus::Results;
        }
        if changed {
            Task::batch([manager_task, p.reveal()])
        } else {
            manager_task
        }
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        if self.closing {
            return Task::none();
        }
        match message {
            Message::ManagerWindowCreated => {
                self.startup_deadline = None;
                Task::none()
            }
            Message::Manager(ManagerMessage::OpenLauncher) => self.open(),
            Message::Manager(ManagerMessage::Applications(frame)) => self.service_frame(frame),
            Message::Manager(ManagerMessage::Pins(state)) => {
                if let Some(p) = &mut self.popup {
                    p.model.apply_pins(state.clone());
                }
                self.manager
                    .update(ManagerMessage::Pins(state))
                    .map(Message::Manager)
            }
            Message::Manager(ManagerMessage::Close) => {
                self.closing = true;
                self.opening = None;
                self.popup = None;
                self.manager
                    .update(ManagerMessage::Close)
                    .map(Message::Manager)
            }
            Message::Manager(message) => {
                let pulse = matches!(message, ManagerMessage::Monitor(crate::Update::Pulse(_)));
                let task = self.manager.update(message).map(Message::Manager);
                if !pulse {
                    return task;
                }
                let now = Instant::now();
                if self
                    .startup_deadline
                    .is_some_and(|deadline| now >= deadline)
                {
                    eprintln!("Sei-Desk management window creation was not confirmed; exiting.");
                    return self.update(Message::Manager(ManagerMessage::Close));
                }
                if self.opening.is_some_and(|(_, deadline)| now >= deadline) {
                    self.opening = None;
                    self.error =
                        Some("Window inspection timed out; try again from the manager.".into());
                }
                if self
                    .popup
                    .as_ref()
                    .is_some_and(|p| p.deadline.is_some_and(|d| now >= d))
                {
                    self.error=Some("Launcher creation or focus was not confirmed. The manager remains available.".into());
                    return Task::batch([task, self.dismiss()]);
                }
                Task::batch([task, self.geometry(false)])
            }
            Message::Geometry(ticket, result) => {
                let initial = self.opening.is_some_and(|(t, _)| t == ticket);
                if !initial
                    && !self
                        .popup
                        .as_ref()
                        .is_some_and(|p| p.checking == Some(ticket))
                {
                    return Task::none();
                }
                if initial {
                    self.opening = None;
                }
                let placement = match result {
                    Ok(p) => p,
                    Err(error) => {
                        self.error = Some(error);
                        return self.dismiss();
                    }
                };
                self.error = None;
                if let Some(p) = &mut self.popup {
                    p.checking = None;
                    if initial {
                        p.deadline = Some(Instant::now() + Duration::from_secs(3));
                        let changed = p.placement != placement;
                        p.placement = placement;
                        return Task::batch([
                            window::gain_focus(p.id),
                            if changed {
                                place(p.id, placement)
                            } else {
                                Task::none()
                            },
                        ]);
                    }
                    if p.placement == placement {
                        return Task::none();
                    }
                    p.placement = placement;
                    return place(p.id, placement);
                }
                if !initial {
                    return Task::none();
                }
                let (width, height) = placement.logical_size();
                let (id, task) = window::open(window::Settings {
                    size: iced::Size::new(width, height),
                    visible: false,
                    decorations: false,
                    transparent: false,
                    resizable: false,
                    exit_on_close_request: false,
                    level: window::Level::Normal,
                    ..Default::default()
                });
                let mut model = applications::Model::default();
                model.apply_frame(Arc::new(scoped(&self.shared, ticket)));
                model.apply_pins(self.manager.apps.pins.clone());
                model.pinned_only = true;
                model.filter();
                self.popup = Some(Launcher {
                    id,
                    token: ticket,
                    model,
                    composing: false,
                    focused: false,
                    focus: Focus::Search,
                    focus_generation: 0,
                    shown: false,
                    deadline: Some(Instant::now() + Duration::from_secs(3)),
                    placement,
                    checking: None,
                    pending_launch: None,
                    launch_attempt: None,
                });
                task.map(Message::Created)
            }
            Message::Created(id) => {
                if let Some(p) = self.popup.as_ref().filter(|p| p.id == id) {
                    place(id, p.placement)
                } else {
                    window::close(id)
                }
            }
            Message::Placed(id, result) => {
                let Some(p) = self.popup.as_mut().filter(|p| p.id == id) else {
                    return Task::none();
                };
                if let Err(error) = result {
                    self.error = Some(error);
                    return self.dismiss();
                }
                if p.shown {
                    return Task::none();
                }
                p.shown = true;
                let focus = p.focus_task();
                let task = Task::batch([
                    window::set_mode(id, window::Mode::Windowed),
                    window::gain_focus(id),
                    focus,
                ]);
                let refresh = if self.shared.catalog.is_none() {
                    let task = self.action(id, Action::Refresh);
                    if let Some(p) = &mut self.popup {
                        p.focus = Focus::Search;
                    }
                    task
                } else {
                    Task::none()
                };
                Task::batch([task, refresh])
            }
            Message::ObservedFocus(id, generation, focused) => {
                let Some(p) = self
                    .popup
                    .as_mut()
                    .filter(|p| p.id == id && p.focus_generation == generation)
                else {
                    return Task::none();
                };
                p.focused = focused;
                if focused {
                    p.deadline = None;
                } else {
                    p.deadline = Some(Instant::now() + Duration::from_secs(3));
                }
                Task::none()
            }
            Message::SearchFocus(id, generation, focused) => {
                if let Some(p) = self
                    .popup
                    .as_mut()
                    .filter(|p| p.id == id && p.focus_generation == generation)
                    && focused
                {
                    p.focus = Focus::Search;
                }
                Task::none()
            }
            Message::Launcher(id, action) => self.action(id, action),
            Message::Event(id, event, status) => {
                if self.closing_popup == Some(id)
                    && matches!(event, Event::Window(window::Event::Closed))
                {
                    self.closing_popup = None;
                    return Task::none();
                }
                if self.manager_id.is_none()
                    && matches!(event, Event::Window(window::Event::Opened { .. }))
                {
                    self.manager_id = Some(id);
                }
                if self.manager_id == Some(id) {
                    if matches!(event, Event::Window(window::Event::Unfocused))
                        && (self.opening.is_some() || self.popup.as_ref().is_some_and(|p| !p.shown))
                    {
                        return self.dismiss();
                    }
                    if matches!(event, Event::Window(window::Event::Closed)) {
                        return self.update(Message::Manager(ManagerMessage::Close));
                    }
                    if matches!(event, Event::Window(window::Event::Opened { .. })) {
                        self.startup_deadline = None;
                    }
                    if let Event::Keyboard(keyboard::Event::KeyPressed {
                        key: keyboard::Key::Character(ref key),
                        modifiers,
                        repeat: false,
                        ..
                    }) = event
                        && key.eq_ignore_ascii_case("l")
                        && modifiers.control()
                        && !modifiers.alt()
                        && !modifiers.logo()
                        && !modifiers.shift()
                    {
                        return self.open();
                    }
                    let placement_changed = matches!(
                        event,
                        Event::Window(window::Event::Moved(_) | window::Event::Rescaled(_))
                    );
                    let task = crate::local_event(event, status, id)
                        .map_or_else(Task::none, |message| self.update(Message::Manager(message)));
                    return if placement_changed {
                        Task::batch([task, self.geometry(false)])
                    } else {
                        task
                    };
                }
                let Some(p) = self.popup.as_mut().filter(|p| p.id == id) else {
                    return Task::none();
                };
                match event {
                    Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
                        let generation = p.focus_generation;
                        iced::widget::operation::is_focused(p.search_id())
                            .map(move |focused| Message::SearchFocus(id, generation, focused))
                    }
                    Event::Window(window::Event::CloseRequested | window::Event::Closed) => {
                        self.dismiss()
                    }
                    Event::Window(window::Event::Focused) => {
                        p.focus_generation = p.focus_generation.wrapping_add(1);
                        let generation = p.focus_generation;
                        p.focused = true;
                        p.deadline = None;
                        window::run(id, |w| {
                            native(w, |hwnd| Ok(sei_windows::launcher::foreground(hwnd)))
                        })
                        .map(move |result| {
                            Message::ObservedFocus(id, generation, result.unwrap_or(false))
                        })
                    }
                    Event::Window(window::Event::Unfocused) => {
                        p.focus_generation = p.focus_generation.wrapping_add(1);
                        p.focused = false;
                        Task::none()
                    }
                    Event::Window(window::Event::Rescaled(_) | window::Event::Moved(_)) => {
                        self.geometry(false)
                    }
                    Event::InputMethod(iced::advanced::input_method::Event::Preedit(text, _)) => {
                        p.composing = !text.is_empty();
                        Task::none()
                    }
                    Event::InputMethod(
                        iced::advanced::input_method::Event::Commit(_)
                        | iced::advanced::input_method::Event::Closed,
                    ) => {
                        p.composing = false;
                        Task::none()
                    }
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key,
                        modifiers,
                        repeat: false,
                        ..
                    }) if !modifiers.control()
                        && !modifiers.alt()
                        && !modifiers.logo()
                        && !p.composing =>
                    {
                        use keyboard::{Key, key::Named};
                        let action = match key {
                            Key::Named(Named::Escape) => Some(Action::Dismiss),
                            Key::Named(Named::Tab) => Some(Action::Traverse(modifiers.shift())),
                            Key::Named(Named::ArrowDown)
                                if status == iced::event::Status::Ignored =>
                            {
                                Some(Action::Move(1))
                            }
                            Key::Named(Named::ArrowUp)
                                if status == iced::event::Status::Ignored =>
                            {
                                Some(Action::Move(-1))
                            }
                            Key::Named(Named::Enter | Named::Space)
                                if status == iced::event::Status::Ignored =>
                            {
                                Some(Action::Activate)
                            }
                            _ => None,
                        };
                        action.map_or_else(Task::none, |action| self.action(id, action))
                    }
                    _ => Task::none(),
                }
            }
        }
    }
    fn action(&mut self, id: window::Id, action: Action) -> Task<Message> {
        let Some(p) = self.popup.as_mut().filter(|p| p.id == id) else {
            return Task::none();
        };
        if p.composing && !matches!(action, Action::Query(_) | Action::Dismiss) {
            return Task::none();
        }
        if !matches!(action, Action::Query(_) | Action::Move(_) | Action::Submit) {
            p.focus_generation = p.focus_generation.wrapping_add(1);
        }
        match action {
            Action::Dismiss => self.dismiss(),
            Action::Query(query) => {
                p.model.local_error = None;
                p.model.pinned_only = false;
                p.model.edit(query);
                p.focus = Focus::Search;
                p.reveal()
            }
            Action::Move(delta) => {
                p.model.move_selection(delta);
                if p.focus == Focus::Retry && p.retry_key().is_none() {
                    p.focus = Focus::Results;
                }
                p.reveal()
            }
            Action::Select(key) => {
                if p.model.filtered.iter().any(|&i| {
                    p.model
                        .frame
                        .catalog
                        .as_ref()
                        .is_some_and(|c| c.entries[i].id == key)
                }) {
                    p.model.selected = Some(key);
                    p.focus = Focus::Results;
                }
                p.reveal()
            }
            Action::Browse => {
                p.focus = Focus::Browse;
                p.model.pinned_only = !p.model.pinned_only;
                p.model.filter();
                p.reveal()
            }
            Action::Traverse(reverse) => {
                p.traverse(reverse);
                Task::batch([p.focus_task(), p.reveal()])
            }
            Action::Activate => {
                let action = match p.focus {
                    Focus::Dismiss => Action::Dismiss,
                    Focus::Refresh => Action::Refresh,
                    Focus::Browse => Action::Browse,
                    Focus::Retry => {
                        let Some(key) = p.retry_key() else {
                            return Task::none();
                        };
                        Action::Retry(key)
                    }
                    _ => Action::Submit,
                };
                self.action(id, action)
            }
            action @ (Action::Submit | Action::Retry(_)) => {
                let failed_key = p.retry_key();
                match &action {
                    Action::Retry(key) if failed_key.as_ref() != Some(key) => return Task::none(),
                    Action::Submit if failed_key.is_some() => return Task::none(),
                    _ => (),
                }
                if p.pending_launch.is_some() || applications::busy(&self.shared.launch) {
                    return Task::none();
                }
                let Some(key) = p.model.key() else {
                    p.model.local_error =
                        Some("Select a current application before launching.".into());
                    return Task::none();
                };
                match self
                    .manager
                    .apps_handle
                    .as_ref()
                    .ok_or_else(|| "Application service unavailable.".to_string())
                    .and_then(|h| h.send_for(p.token, Command::Launch(Some(key.clone()))))
                {
                    Ok(seq) => {
                        p.pending_launch = Some(seq);
                        p.launch_attempt = Some(key);
                        p.model.mark_launch(seq);
                        p.model.local_error = None;
                    }
                    Err(error) => p.model.local_error = Some(error),
                }
                Task::none()
            }
            Action::Refresh => {
                p.focus = Focus::Refresh;
                if p.model.refreshing() {
                    return Task::none();
                }
                match self
                    .manager
                    .apps_handle
                    .as_ref()
                    .ok_or_else(|| "Application service unavailable.".to_string())
                    .and_then(|h| h.send_for(p.token, Command::Refresh))
                {
                    Ok(seq) => {
                        p.model.mark_refresh(seq);
                        p.model.local_error = None;
                    }
                    Err(error) => p.model.local_error = Some(error),
                }
                Task::none()
            }
        }
    }
}
fn scoped(frame: &Frame, origin: u64) -> Frame {
    let mut frame = frame.clone();
    if frame.refresh_origin != 0 && frame.refresh_origin != origin {
        frame.refresh = Status::Idle;
    }
    if frame.launch_origin != origin {
        frame.launch = Status::Idle;
        frame.launch_name = None;
    }
    if frame.error_origin != origin {
        frame.error = None;
        frame.stale = false;
    }
    frame
}
fn native<T>(
    w: &dyn window::Window,
    f: impl FnOnce(usize) -> Result<T, String>,
) -> Result<T, String> {
    let handle = w
        .window_handle()
        .map_err(|_| "Native window handle unavailable.")?;
    match handle.as_raw() {
        window::raw_window_handle::RawWindowHandle::Win32(handle) => f(handle.hwnd.get() as usize),
        _ => Err("The launcher requires a validated Windows handle.".into()),
    }
}
fn place(id: window::Id, placement: Placement) -> Task<Message> {
    window::run(id, move |w| {
        native(w, |hwnd| sei_windows::launcher::place(hwnd, placement))
    })
    .map(move |result| Message::Placed(id, result))
}

#[cfg(test)]
pub(crate) mod tests;
