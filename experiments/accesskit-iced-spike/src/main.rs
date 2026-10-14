//! Standalone feasibility window; deliberately not integrated into Sei-Desk.
#[path = "../../../crates/sei-desk/src/model.rs"]
mod model;
mod native;
mod semantics;
mod wrapper;

use iced::futures::channel::mpsc;
use iced::{
    Element, Event, Subscription, Task, keyboard,
    widget::{self, button, checkbox, column, container, row, text, text_input},
    window,
};
use model::{Action, Feedback, Focus};
use semantics::{Bounds, INPUT_ID, State};

#[derive(Debug, Clone)]
enum Message {
    Opened(window::Id),
    Attached(Result<(), String>),
    Bounds(Bounds),
    Published,
    Action(Action),
    Pointer(Action, Focus),
    Native(accesskit::ActionRequest),
    Geometry,
    CheckInput,
    InputFocused(bool),
    Exit,
}
struct App {
    state: State,
    id: Option<window::Id>,
    sender: mpsc::Sender<accesskit::ActionRequest>,
    attached: bool,
    shown: bool,
}
impl App {
    fn boot(sender: mpsc::Sender<accesskit::ActionRequest>) -> (Self, Task<Message>) {
        (
            Self {
                state: State::default(),
                id: None,
                sender,
                attached: false,
                shown: false,
            },
            Task::none(),
        )
    }
    fn collect(&self) -> Task<Message> {
        if !self.attached {
            return Task::none();
        }
        iced::advanced::widget::operate(wrapper::Collect::default()).map(Message::Bounds)
    }
    fn focus(&self) -> Task<Message> {
        widget::operation::focus(if self.state.model.focus == Focus::Input {
            INPUT_ID
        } else {
            "spike-no-input"
        })
    }
    fn action(&mut self, action: Action) -> Task<Message> {
        let edit = matches!(action, Action::Edit(_) | Action::Composition(_));
        if !self.state.apply(action) {
            return Task::none();
        }
        eprintln!(
            "state focus={:?} error={} checked={} closing={}",
            self.state.model.focus,
            self.state.model.feedback == Feedback::Empty,
            self.state.model.prefer_reduced_motion,
            self.state.model.closing
        );
        if self.state.model.closing {
            return self.id.map_or_else(iced::exit, |id| {
                window::run(id, |_| {
                    native::detach();
                    Message::Exit
                })
            });
        }
        if edit {
            self.collect()
        } else {
            self.focus().chain(self.collect())
        }
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        if matches!(message, Message::Exit) {
            return iced::exit();
        }
        if self.state.model.closing {
            return Task::none();
        }
        match message {
            Message::Opened(id) => {
                self.id = Some(id);
                let sender = self.sender.clone();
                let initial = semantics::tree(&self.state, &Bounds::new(), 1.0);
                window::run(id, move |window| {
                    use iced::window::raw_window_handle::RawWindowHandle;
                    let result = window
                        .window_handle()
                        .map_err(|error| error.to_string())
                        .and_then(|handle| match handle.as_raw() {
                            RawWindowHandle::Win32(handle) => {
                                native::install(handle.hwnd.get() as usize, initial, sender)
                            }
                            _ => Err("Windows HWND required".into()),
                        });
                    Message::Attached(result)
                })
            }
            Message::Attached(Ok(())) => {
                self.attached = true;
                self.focus().chain(self.collect())
            }
            Message::Attached(Err(error)) => {
                eprintln!("attachment_failed: {error}");
                iced::exit()
            }
            Message::Bounds(bounds) => {
                let state_tree = semantics::tree(&self.state, &bounds, 1.0);
                let id = self.id.unwrap();
                window::run(id, move |_| {
                    let mut update = state_tree;
                    let scale = native::scale();
                    for (_, node) in &mut update.nodes {
                        if let Some(rect) = node.bounds() {
                            node.set_bounds(accesskit::Rect::new(
                                rect.x0 * scale,
                                rect.y0 * scale,
                                rect.x1 * scale,
                                rect.y1 * scale,
                            ));
                        }
                    }
                    native::publish(update);
                    Message::Published
                })
            }
            Message::Published if !self.shown => {
                self.shown = true;
                window::set_mode(self.id.unwrap(), window::Mode::Windowed)
            }
            Message::Published => Task::none(),
            Message::Geometry => self.collect(),
            Message::Action(action) => self.action(action),
            Message::Pointer(action, focus) => {
                self.state.apply(Action::Focus(focus));
                self.action(action)
            }
            Message::Native(request) => {
                let Some(action) = self.state.accessible_action(&request) else {
                    eprintln!("stale_or_unsupported_action_rejected");
                    return Task::none();
                };
                eprintln!(
                    "uia_action={:?} node={:?}",
                    request.action, request.target_node
                );
                if matches!(action, Action::Edit(_)) {
                    // ValuePattern updates the value, not keyboard/foreground focus.
                    let focus = self.state.model.focus;
                    self.state.apply(action);
                    self.state.apply(Action::Focus(focus));
                    self.collect()
                } else {
                    self.action(action)
                }
            }
            Message::CheckInput => {
                widget::operation::is_focused(INPUT_ID).map(Message::InputFocused)
            }
            Message::InputFocused(true) => {
                self.state.apply(Action::Focus(Focus::Input));
                self.collect()
            }
            Message::InputFocused(false) => Task::none(),
            Message::Exit => iced::exit(),
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::event::listen_with(event),
            Subscription::run(native::actions).map(Message::Native),
        ])
    }
    fn view(&self) -> Element<'_, Message> {
        use semantics::*;
        use wrapper::wrap;
        let focused = |focus| self.state.model.focus == focus;
        let action = |id, label, model_action, focus| {
            wrap(
                id,
                button(text(label).size(14))
                    .padding([10, 16])
                    .on_press(Message::Pointer(model_action, focus))
                    .style(move |_, _| widget::button::Style {
                        background: Some(iced::color!(0xC5E8E3).into()),
                        text_color: iced::color!(0x293649),
                        border: iced::Border {
                            color: iced::color!(0x44638A),
                            width: if focused(focus) { 2.0 } else { 0.0 },
                            radius: 8.0.into(),
                        },
                        ..Default::default()
                    }),
            )
        };
        let input = wrap(
            INPUT,
            text_input("Type sample text", &self.state.model.value)
                .id(INPUT_ID)
                .on_input(|value| Message::Action(Action::Edit(value)))
                .on_submit(Message::Action(Action::Validate))
                .padding(10)
                .size(14),
        );
        let motion = wrap(
            MOTION,
            container(
                checkbox(self.state.model.prefer_reduced_motion)
                    .label("Prefer reduced motion")
                    .on_toggle(|_| Message::Pointer(Action::ToggleMotion, Focus::Motion)),
            )
            .padding(8)
            .style(move |_| widget::container::Style {
                border: iced::Border {
                    color: iced::color!(0x44638A),
                    width: if focused(Focus::Motion) { 2.0 } else { 0.0 },
                    radius: 8.0.into(),
                },
                ..Default::default()
            }),
        );
        let feedback: Element<'_, Message> = if self.state.model.feedback == Feedback::Empty {
            container(
                row![
                    wrap(
                        self.state.error_id(),
                        text("Enter some text before validating.").size(14)
                    ),
                    action(
                        self.state.dismiss_id(),
                        "Dismiss error",
                        Action::DismissError,
                        Focus::DismissError
                    )
                ]
                .spacing(12)
                .align_y(iced::Alignment::Center),
            )
            .padding(12)
            .style(|_| widget::container::Style {
                background: Some(iced::color!(0xF3B8C8).into()),
                ..Default::default()
            })
            .into()
        } else {
            text(if self.state.model.feedback == Feedback::Valid {
                "The sample is valid."
            } else {
                "Enter text, then choose Validate."
            })
            .size(14)
            .into()
        };
        container(column![text("AccessKit + iced feasibility").size(24),wrap(LABEL,text("Sample text").size(14)),input,row![action(VALIDATE,"Validate",Action::Validate,Focus::Validate),motion].spacing(12).align_y(iced::Alignment::Center),text(if self.state.model.effective_reduced_motion(Some(false)){"Local motion preference: reduced · no animations"}else{"Local motion preference: default · no animations"}).size(12),feedback,action(CLOSE,"Close",Action::Close,Focus::Close),text("Tab / Shift+Tab: focus · Enter / Space: action\nEscape: dismiss error, then close").size(12)].spacing(12).padding(24)).width(iced::Length::Fill).height(iced::Length::Fill).into()
    }
}
fn event(event: Event, status: iced::event::Status, id: window::Id) -> Option<Message> {
    use keyboard::{Key, key::Named};
    match event {
        Event::Window(window::Event::Opened { .. }) => Some(Message::Opened(id)),
        Event::Window(window::Event::CloseRequested) => Some(Message::Action(Action::Close)),
        Event::Window(
            window::Event::Moved(_)
            | window::Event::Resized(_)
            | window::Event::Rescaled(_)
            | window::Event::Focused
            | window::Event::Unfocused,
        ) => Some(Message::Geometry),
        Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
            Some(Message::CheckInput)
        }
        Event::InputMethod(iced::advanced::input_method::Event::Preedit(value, _)) => {
            Some(Message::Action(Action::Composition(!value.is_empty())))
        }
        Event::InputMethod(
            iced::advanced::input_method::Event::Commit(_)
            | iced::advanced::input_method::Event::Closed,
        ) => Some(Message::Action(Action::Composition(false))),
        Event::Keyboard(keyboard::Event::KeyPressed {
            key,
            modifiers,
            repeat,
            ..
        }) if !modifiers.control() && !modifiers.alt() && !modifiers.logo() => {
            let action = match key.as_ref() {
                Key::Named(Named::Tab) => Action::Traverse {
                    reverse: modifiers.shift(),
                },
                Key::Named(Named::Escape) if !repeat => Action::Escape,
                Key::Named(Named::Enter | Named::Space) | Key::Character(" ")
                    if !repeat && status == iced::event::Status::Ignored =>
                {
                    Action::Activate
                }
                _ => return None,
            };
            Some(Message::Action(action))
        }
        _ => None,
    }
}
struct OneWorker(iced::futures::executor::ThreadPool);
impl iced::Executor for OneWorker {
    fn new() -> Result<Self, std::io::Error> {
        iced::futures::executor::ThreadPoolBuilder::new()
            .pool_size(1)
            .create()
            .map(Self)
    }
    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        self.0.spawn_ok(future);
    }
    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        iced::futures::executor::block_on(future)
    }
}
fn main() -> std::process::ExitCode {
    let sender = native::prepare();
    let result = iced::application(move || App::boot(sender.clone()), App::update, App::view)
        .title("AccessKit iced spike")
        .executor::<OneWorker>()
        .subscription(App::subscription)
        .theme(|_: &App| {
            iced::Theme::custom(
                "Sei pastel",
                iced::theme::Palette {
                    background: iced::color!(0xEFF1F7),
                    text: iced::color!(0x293649),
                    primary: iced::color!(0x44638A),
                    success: iced::color!(0x44638A),
                    warning: iced::color!(0x53647A),
                    danger: iced::color!(0x293649),
                },
            )
        })
        .default_font(iced::Font::with_name("Segoe UI"))
        .window(window::Settings {
            visible: false,
            size: iced::Size::new(640.0, 400.0),
            min_size: Some(iced::Size::new(600.0, 400.0)),
            exit_on_close_request: false,
            ..Default::default()
        })
        .run();
    if let Err(error) = result {
        eprintln!("iced_failed: {error}");
        std::process::ExitCode::FAILURE
    } else {
        std::process::ExitCode::SUCCESS
    }
}
