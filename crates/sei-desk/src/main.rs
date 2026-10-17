//! First runnable dashboard; the accepted validation harness remains available.
mod applications;
mod monitor;
mod traffic;
mod traffic_chart;
mod validation;
mod view;

use iced::{Element, Event, Subscription, Task, keyboard, window};
use monitor::{Feed, Monitor, Update};
use sei_windows::metrics::{Battery, Capacity, Snapshot};
use std::{process::ExitCode, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Overview,
    Applications,
}
#[derive(Debug, Clone)]
enum Message {
    Applications(std::sync::Arc<applications::Frame>),
    Pins(std::sync::Arc<applications::preferences::worker::State>),
    Reveal { generation: u64, offset: f32 },
    AppAction(applications::Action),
    Composition(bool),
    QuerySearchFocus,
    SearchFocused { generation: u64, focused: bool },
    Monitor(Update),
    Navigate(Page),
    Traverse(bool),
    Activate,
    Scroll(f32),
    Resized(f32),
    Close,
}
struct App {
    apps: applications::Model,
    apps_handle: Option<applications::runtime::Handle>,
    pins_handle: Option<applications::preferences::worker::Handle>,
    composing: bool,
    focus_generation: u64,
    feed: Feed,
    snapshot: Option<Snapshot>,
    cpu: Option<f32>,
    network: Option<sei_windows::network::Reading>,
    traffic: traffic::History,
    sampled_at: Option<Instant>,
    now: Instant,
    clock: String,
    page: Page,
    focus: Option<Page>,
    width: f32,
    error: Option<String>,
    diagnostics: bool,
}
impl App {
    fn boot(feed: Feed, diagnostics: bool) -> Self {
        Self {
            apps: applications::Model::default(),
            apps_handle: None,
            pins_handle: None,
            composing: false,
            focus_generation: 0,
            feed,
            snapshot: None,
            cpu: None,
            network: None,
            traffic: traffic::History::default(),
            sampled_at: None,
            now: Instant::now(),
            clock: "Collecting local time…".into(),
            page: Page::Overview,
            focus: None,
            width: 1100.0,
            error: None,
            diagnostics,
        }
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Reveal { generation, offset } => {
                if self.page == Page::Applications && generation == self.focus_generation {
                    return iced::widget::operation::scroll_to(
                        "dashboard-content",
                        iced::widget::operation::AbsoluteOffset {
                            x: None,
                            y: Some(offset),
                        },
                    );
                }
            }
            Message::Pins(state) => {
                if self.apps.apply_pins(state) && self.page == Page::Applications {
                    return iced::widget::operation::scroll_to(
                        "applications-results",
                        iced::widget::operation::AbsoluteOffset {
                            x: None,
                            y: Some(self.apps.offset(self.width)),
                        },
                    );
                }
            }
            Message::Applications(frame) => {
                if self.diagnostics {
                    eprintln!(
                        "applications_frame revision={} refresh={:?} launch={:?} entries={}",
                        frame.revision,
                        frame.refresh,
                        frame.launch,
                        frame.catalog.as_ref().map_or(0, |c| c.entries.len())
                    );
                }
                self.apps.apply_frame(frame);
            }
            Message::AppAction(action) => return self.apps_action(action),
            Message::Composition(active) => self.composing = active,
            Message::QuerySearchFocus if self.page == Page::Applications => {
                let generation = self.focus_generation;
                return iced::widget::operation::is_focused("applications-search").map(
                    move |focused| Message::SearchFocused {
                        generation,
                        focused,
                    },
                );
            }
            Message::QuerySearchFocus => (),
            Message::SearchFocused {
                generation,
                focused,
            } => {
                if focused && generation == self.focus_generation && self.page == Page::Applications
                {
                    self.apps.focus = Some(applications::Focus::Search);
                }
            }
            Message::Monitor(Update::Sample {
                snapshot,
                cpu,
                network,
                at,
            }) => {
                if self.sampled_at.is_some_and(|previous| at <= previous) {
                    return Task::none();
                }
                if self.diagnostics {
                    eprintln!(
                        "dashboard_sample cpu={cpu:?} memory={:?} disk={:?} battery={:?}",
                        snapshot.memory, snapshot.disk, snapshot.battery
                    );
                    eprintln!(
                        "network_sample status={:?} interfaces={} history={}",
                        network.status,
                        network.cohort.len(),
                        self.traffic.len()
                    );
                }
                self.traffic.observe(&network);
                self.network = Some(*network);
                self.snapshot = Some(snapshot);
                self.cpu = cpu;
                self.sampled_at = Some(at);
                self.now = Instant::now();
                self.error = None;
            }
            Message::Monitor(Update::Pulse(clock)) => {
                self.now = Instant::now();
                self.clock = clock;
                self.traffic.prune(self.now);
            }
            Message::Monitor(Update::Failed(error)) => self.error = Some(error),
            Message::Navigate(page) => {
                self.focus_generation = self.focus_generation.wrapping_add(1);
                self.page = page;
                self.focus = Some(page);
                self.apps.focus =
                    (page == Page::Applications).then_some(applications::Focus::Search);
                if page == Page::Applications && !self.apps.requested {
                    self.refresh_apps();
                }
                return Task::batch([
                    self.focus_task(),
                    iced::widget::operation::scroll_to(
                        "dashboard-content",
                        iced::widget::operation::AbsoluteOffset {
                            x: Some(0.0),
                            y: Some(0.0),
                        },
                    ),
                ]);
            }
            Message::Traverse(reverse) => {
                if self.composing {
                    return Task::none();
                }
                if self.page == Page::Applications {
                    let order = self.apps.focus_order();
                    let current = if let Some(target) = self.apps.focus {
                        order.iter().position(|f| *f == Some(target)).unwrap_or(2)
                    } else if self.focus == Some(Page::Overview) {
                        0
                    } else {
                        1
                    };
                    let next = if reverse {
                        (current + order.len() - 1) % order.len()
                    } else {
                        (current + 1) % order.len()
                    };
                    self.apps.focus = order[next];
                    if next < 2 {
                        self.focus = Some(if next == 0 {
                            Page::Overview
                        } else {
                            Page::Applications
                        });
                    }
                    self.focus_generation = self.focus_generation.wrapping_add(1);
                    return self.focus_task();
                }
                self.focus = Some(match (self.focus, reverse) {
                    (None, true) | (Some(Page::Overview), _) => Page::Applications,
                    _ => Page::Overview,
                })
            }
            Message::Activate => {
                if self.composing {
                    return Task::none();
                }
                if self.page == Page::Applications
                    && let Some(focus) = self.apps.focus
                {
                    use applications::{Action, Focus};
                    return self.apps_action(match focus {
                        Focus::Search | Focus::Results => Action::Submit,
                        Focus::Refresh => Action::Refresh,
                        Focus::Launch => Action::Launch,
                        Focus::Cancel => Action::Cancel,
                        Focus::Filter => Action::Filter,
                        Focus::Presentation => Action::Presentation,
                        Focus::Sort => Action::Sort,
                        Focus::Group => Action::Group,
                        Focus::Pin => Action::Pin,
                    });
                }
                if let Some(page) = self.focus {
                    return self.update(Message::Navigate(page));
                }
            }
            Message::Scroll(y) => {
                return iced::widget::operation::scroll_by(
                    "dashboard-content",
                    iced::widget::operation::AbsoluteOffset { x: 0.0, y },
                );
            }
            Message::Resized(width) => self.width = width,
            Message::Close => {
                if let Some(handle) = &self.apps_handle {
                    handle.stop();
                }
                return iced::exit();
            }
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        view::content(self)
    }
    fn focus_task(&self) -> Task<Message> {
        let focus = iced::widget::operation::focus(
            if self.page == Page::Applications
                && self.apps.focus == Some(applications::Focus::Search)
            {
                "applications-search"
            } else {
                "applications-local-focus"
            },
        );
        let target = match self.apps.focus {
            Some(applications::Focus::Search) => "applications-search",
            Some(applications::Focus::Results) => "applications-results-panel",
            Some(focus) => focus.id(),
            None => return focus,
        };
        if self.page != Page::Applications {
            return focus;
        }
        let generation = self.focus_generation;
        Task::batch([
            focus,
            iced::advanced::widget::operate(applications::reveal::query(target))
                .map(move |offset| Message::Reveal { generation, offset }),
        ])
    }
    fn send_apps(&mut self, command: applications::runtime::Command) -> Option<u64> {
        match self
            .apps_handle
            .as_ref()
            .ok_or_else(|| "Application service unavailable".to_string())
            .and_then(|handle| handle.send(command))
        {
            Ok(seq) => {
                self.apps.local_error = None;
                Some(seq)
            }
            Err(error) => {
                self.apps.local_error = Some(error);
                None
            }
        }
    }
    fn refresh_apps(&mut self) {
        if !self.apps.refreshing()
            && let Some(seq) = self.send_apps(applications::runtime::Command::Refresh)
        {
            self.apps.mark_refresh(seq);
        }
    }
    fn apps_action(&mut self, action: applications::Action) -> Task<Message> {
        use applications::{Action, Focus, runtime::Command};
        if self.page != Page::Applications {
            return Task::none();
        }
        if self.composing && !matches!(action, Action::Query(_)) {
            return Task::none();
        }
        if !matches!(action, Action::Query(_) | Action::Move(_) | Action::Submit) {
            self.focus_generation = self.focus_generation.wrapping_add(1);
        }
        match action {
            Action::Filter | Action::Presentation | Action::Sort | Action::Group => {
                use applications::Presentation;
                self.apps.focus = Some(match action {
                    Action::Filter => {
                        self.apps.pinned_only = !self.apps.pinned_only;
                        Focus::Filter
                    }
                    Action::Presentation => {
                        self.apps.presentation = if self.apps.presentation == Presentation::List {
                            Presentation::Grid
                        } else {
                            Presentation::List
                        };
                        Focus::Presentation
                    }
                    Action::Sort => {
                        self.apps.sort = self.apps.sort.next();
                        Focus::Sort
                    }
                    _ => {
                        self.apps.grouping = self.apps.grouping.next();
                        Focus::Group
                    }
                });
                self.apps.filter();
                self.send_apps(Command::Select(self.apps.key()));
                return Task::batch([
                    self.focus_task(),
                    iced::widget::operation::scroll_to(
                        "applications-results",
                        iced::widget::operation::AbsoluteOffset {
                            x: Some(0.0),
                            y: Some(self.apps.offset(self.width)),
                        },
                    ),
                ]);
            }
            Action::Pin => {
                self.apps.focus = Some(Focus::Pin);
                let result = self.apps.pin_change().and_then(|pins| {
                    self.pins_handle
                        .as_ref()
                        .ok_or_else(|| "Pins worker unavailable.".to_string())?
                        .save(pins)
                });
                match result {
                    Ok(()) => {
                        self.apps.pins_pending = true;
                        self.apps.local_error = None
                    }
                    Err(error) => self.apps.local_error = Some(error),
                }
                return self.focus_task();
            }
            Action::Missing(token) => {
                if self
                    .apps
                    .missing
                    .iter()
                    .any(|&i| self.apps.pins.pins[i].token == token)
                {
                    self.apps.missing_selected = Some(token);
                    self.apps.selected = None;
                    self.apps.focus = Some(Focus::Results);
                    self.send_apps(Command::Select(None));
                    return self.focus_task();
                }
            }
            Action::Query(query) => {
                self.apps.edit(query);
                self.apps.focus = Some(Focus::Search);
                self.send_apps(Command::Select(self.apps.key()));
            }
            Action::Move(direction) => {
                self.apps.move_selection(direction);
                self.send_apps(Command::Select(self.apps.key()));
                return iced::widget::operation::scroll_to(
                    "applications-results",
                    iced::widget::operation::AbsoluteOffset {
                        x: None,
                        y: Some(self.apps.offset(self.width)),
                    },
                );
            }
            Action::Select(id) => {
                if self
                    .apps
                    .frame
                    .catalog
                    .as_ref()
                    .is_some_and(|c| self.apps.filtered.iter().any(|&i| c.entries[i].id == id))
                {
                    self.apps.selected = Some(id);
                    self.apps.missing_selected = None;
                    self.apps.focus = Some(Focus::Results);
                    self.focus_generation = self.focus_generation.wrapping_add(1);
                    self.send_apps(Command::Select(self.apps.key()));
                    return self.focus_task();
                }
            }
            Action::Refresh => {
                self.apps.focus = Some(Focus::Refresh);
                self.refresh_apps();
                return self.focus_task();
            }
            Action::Cancel => {
                self.apps.focus = Some(Focus::Cancel);
                self.send_apps(Command::Cancel);
                return self.focus_task();
            }
            Action::Launch | Action::Submit => {
                if matches!(action, Action::Launch) {
                    self.apps.focus = Some(Focus::Launch);
                }
                if !self.apps.launching() {
                    if let Some(key) = self.apps.key() {
                        if let Some(seq) = self.send_apps(Command::Launch(Some(key))) {
                            self.apps.mark_launch(seq);
                        }
                    } else {
                        self.apps.local_error =
                            Some("Select a current application before launching.".into());
                    }
                }
                if self.apps.focus == Some(Focus::Launch) {
                    return self.focus_task();
                }
            }
            Action::Clear => {
                if !self.apps.query.is_empty() {
                    self.apps.edit(String::new());
                } else {
                    self.apps.selected = None;
                    self.apps.missing_selected = None;
                }
                self.apps.focus = Some(Focus::Search);
                self.send_apps(Command::Select(self.apps.key()));
                return self.focus_task();
            }
        }
        Task::none()
    }
    fn paused(&self) -> bool {
        self.sampled_at
            .is_some_and(|at| self.now.saturating_duration_since(at).as_secs() >= 5)
    }
    fn subscription(&self) -> Subscription<Message> {
        let mut subscriptions = vec![
            Subscription::run_with(self.feed.clone(), monitor::stream).map(Message::Monitor),
            iced::event::listen_with(local_event),
        ];
        if let Some(handle) = &self.apps_handle {
            subscriptions.push(
                Subscription::run_with(handle.feed(), applications::runtime::stream)
                    .map(Message::Applications),
            );
        }
        if let Some(handle) = &self.pins_handle {
            subscriptions.push(
                Subscription::run_with(handle.feed(), applications::preferences::worker::stream)
                    .map(Message::Pins),
            );
        }
        Subscription::batch(subscriptions)
    }
}
fn local_event(event: Event, status: iced::event::Status, _: window::Id) -> Option<Message> {
    use keyboard::{Key, key::Named};
    match event {
        Event::InputMethod(iced::advanced::input_method::Event::Preedit(content, _)) => {
            Some(Message::Composition(!content.is_empty()))
        }
        Event::InputMethod(
            iced::advanced::input_method::Event::Commit(_)
            | iced::advanced::input_method::Event::Closed,
        ) => Some(Message::Composition(false)),
        Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
            Some(Message::QuerySearchFocus)
        }
        Event::Window(window::Event::CloseRequested) => Some(Message::Close),
        Event::Window(window::Event::Resized(size)) => Some(Message::Resized(size.width)),
        Event::Window(window::Event::Opened { size, .. }) => Some(Message::Resized(size.width)),
        Event::Keyboard(keyboard::Event::KeyPressed {
            key,
            modifiers,
            repeat: false,
            ..
        }) if !modifiers.control()
            && !modifiers.alt()
            && !modifiers.logo()
            && (status == iced::event::Status::Ignored
                || matches!(key, Key::Named(Named::Tab | Named::Escape))) =>
        {
            match key {
                Key::Named(Named::Tab) => Some(Message::Traverse(modifiers.shift())),
                Key::Named(Named::Enter | Named::Space) => Some(Message::Activate),
                Key::Named(Named::PageDown) => Some(Message::Scroll(320.0)),
                Key::Named(Named::PageUp) => Some(Message::Scroll(-320.0)),
                Key::Named(Named::ArrowDown) => {
                    Some(Message::AppAction(applications::Action::Move(1)))
                }
                Key::Named(Named::ArrowUp) => {
                    Some(Message::AppAction(applications::Action::Move(-1)))
                }
                Key::Named(Named::Escape) => Some(Message::AppAction(applications::Action::Clear)),
                _ => None,
            }
        }
        _ => None,
    }
}
fn main() -> ExitCode {
    let mut args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|arg| arg == "--validation") {
        args.remove(0);
        return validation::run(args);
    }
    if args.iter().any(|arg| arg != "--diagnostics") {
        eprintln!(
            "Usage: sei-desk [--diagnostics] | --validation [--diagnostics] [--error-fixture]"
        );
        return ExitCode::from(2);
    }
    let diagnostics = args.iter().any(|arg| arg == "--diagnostics");
    let pins = applications::preferences::worker::Owner::start();
    let pins_handle = pins.as_ref().ok().map(|owner| owner.handle());
    let pins_error = pins
        .as_ref()
        .err()
        .map(|error| format!("Favorites worker unavailable: {error}"));
    let applications = applications::runtime::Owner::start();
    let apps_handle = applications.as_ref().ok().map(|owner| owner.handle());
    let apps_error = applications
        .as_ref()
        .err()
        .map(|error| format!("Application worker unavailable: {error}"));
    let monitor = match Monitor::start() {
        Ok(monitor) => monitor,
        Err(error) => {
            eprintln!("Could not start monitoring: {error}");
            return ExitCode::FAILURE;
        }
    };
    let feed = monitor.feed();
    let result = iced::application(
        move || {
            let mut app = App::boot(feed.clone(), diagnostics);
            app.apps_handle = apps_handle.clone();
            app.pins_handle = pins_handle.clone();
            if let Some(error) = &pins_error {
                app.apps.apply_pins(std::sync::Arc::new(
                    applications::preferences::worker::State {
                        revision: 1,
                        error: Some(error.clone()),
                        ..Default::default()
                    },
                ));
            }
            app.apps.local_error = apps_error.clone();
            app
        },
        App::update,
        App::view,
    )
    .title("Sei-Desk")
    .executor::<validation::OneWorker>()
    .subscription(App::subscription)
    .theme(view::theme)
    .default_font(iced::Font::with_name("Segoe UI"))
    .window(window::Settings {
        size: iced::Size::new(1100.0, 740.0),
        min_size: Some(iced::Size::new(760.0, 520.0)),
        position: window::Position::Centered,
        exit_on_close_request: false,
        ..Default::default()
    })
    .run();
    // The event loop has returned: stop/wake and join outside the UI thread.
    drop(monitor);
    drop(applications);
    drop(pins);
    if diagnostics {
        eprintln!("dashboard_workers_joined");
    }
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Sei-Desk could not run: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_focus_results_cannot_override_a_new_control_focus() {
        let monitor = Monitor::start().unwrap();
        let mut app = App::boot(monitor.feed(), false);
        app.page = Page::Applications;
        app.apps.focus = Some(applications::Focus::Search);
        let generation = app.focus_generation;
        let _ = app.update(Message::AppAction(applications::Action::Refresh));
        let _ = app.update(Message::SearchFocused {
            generation,
            focused: true,
        });
        assert_eq!(app.apps.focus, Some(applications::Focus::Refresh));
        app.composing = true;
        app.apps.local_error = None;
        let _ = app.update(Message::AppAction(applications::Action::Submit));
        assert!(app.apps.local_error.is_none());
    }
    #[test]
    fn tab_reaches_search_results_refresh_and_navigation_without_launching() {
        let monitor = Monitor::start().unwrap();
        let mut app = App::boot(monitor.feed(), false);
        app.page = Page::Applications;
        app.focus = Some(Page::Applications);
        app.apps.focus = Some(applications::Focus::Search);
        let _ = app.update(Message::Traverse(false));
        assert_eq!(app.apps.focus, Some(applications::Focus::Filter));
        for focus in [
            applications::Focus::Presentation,
            applications::Focus::Sort,
            applications::Focus::Group,
            applications::Focus::Results,
            applications::Focus::Refresh,
        ] {
            let _ = app.update(Message::Traverse(false));
            assert_eq!(app.apps.focus, Some(focus));
        }
        let _ = app.update(Message::Traverse(false));
        assert!(app.apps.focus.is_none());
        assert_eq!(app.focus, Some(Page::Overview));
        let _ = app.update(Message::Activate);
        assert_eq!(app.page, Page::Overview);
        assert!(app.apps.frame.launch_name.is_none());
    }
    #[test]
    fn out_of_order_sample_cannot_replace_rates_or_steal_navigation_focus() {
        use sei_windows::network::{Identity, Rates, Reading, Status};
        let monitor = Monitor::start().unwrap();
        let mut app = App::boot(monitor.feed(), false);
        let at = Instant::now();
        let make = |at, value| {
            Message::Monitor(Update::Sample {
                snapshot: sei_windows::metrics::sample(),
                cpu: Some(value as f32),
                at,
                network: Box::new(Reading {
                    at,
                    cohort: vec![Identity { guid: 1, luid: 1 }],
                    status: Status::Live(Rates {
                        receive: value,
                        transmit: 0.0,
                    }),
                }),
            })
        };
        let _ = app.update(Message::Navigate(Page::Applications));
        let _ = app.update(make(at, 10.0));
        let _ = app.update(make(at + std::time::Duration::from_secs(2), 20.0));
        let _ = app.update(make(at, 999.0));
        assert_eq!(app.cpu, Some(20.0));
        assert_eq!(app.traffic.len(), 2);
        assert_eq!(
            app.network.unwrap().status,
            Status::Live(Rates {
                receive: 20.0,
                transmit: 0.0
            })
        );
        assert_eq!(app.page, Page::Applications);
        assert_eq!(app.focus, Some(Page::Applications));
    }
    #[test]
    fn paused_readings_and_navigation_are_independent() {
        let monitor = Monitor::start().unwrap();
        let mut app = App::boot(monitor.feed(), false);
        app.sampled_at = Some(app.now - std::time::Duration::from_secs(6));
        assert!(app.paused());
        let _ = app.update(Message::Traverse(false));
        let _ = app.update(Message::Activate);
        assert_eq!(app.page, Page::Overview);
        let _ = app.update(Message::Traverse(false));
        let _ = app.update(Message::Activate);
        assert_eq!(app.page, Page::Applications);
        assert!(app.paused());
    }
}
