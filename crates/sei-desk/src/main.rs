//! Stage 2A: one ordinary iced window. No production desktop surfaces.
mod model;
mod view;

use iced::{Element, Event, Subscription, Task, keyboard, widget::operation, window};
use model::{Action, Focus, Model};
use sei_windows::{ForegroundBookmark, MotionPreference};
use std::process::ExitCode;

const INPUT_ID: &str = "stage2a-sample-input";
// Iced's focus operation unfocuses nonmatching widgets. Buttons/checkbox have
// no built-in focus state in 0.14.x; local traversal is explicit in Model.
const NON_INPUT_ID: &str = "stage2a-local-action-focus";

#[derive(Debug, Clone)]
enum Message {
    Action(Action),
    PointerValidate,
    PointerMotion,
    PointerDismiss,
    QueryInputFocus,
    InputFocusChecked { generation: u64, focused: bool },
    WindowActive(bool),
    Opened,
    DiagnosticHandle(usize),
}

struct App {
    model: Model,
    bookmark: ForegroundBookmark,
    system_reduce: Option<bool>,
    window_active: bool,
    diagnostics: bool,
    focus_generation: u64,
}

impl App {
    fn boot(
        bookmark: ForegroundBookmark,
        diagnostics: bool,
        error_fixture: bool,
    ) -> (Self, Task<Message>) {
        let mut app = Self {
            model: Model::default(),
            bookmark,
            system_reduce: None,
            window_active: true,
            diagnostics,
            focus_generation: 0,
        };
        app.refresh_motion();
        if error_fixture {
            app.model.apply(Action::Focus(Focus::Validate));
            app.model.apply(Action::Validate);
            if diagnostics {
                eprintln!("fixture_error=true");
            }
        }
        let focus = app.focus_task();
        (app, focus)
    }
    fn refresh_motion(&mut self) {
        self.system_reduce = match sei_windows::motion_preference() {
            Ok(preference) => Some(preference == MotionPreference::Reduce),
            Err(error) => {
                eprintln!("motion_preference_unavailable: {error}");
                None
            }
        };
    }
    fn focus_task(&self) -> Task<Message> {
        operation::focus(if self.model.focus == Focus::Input {
            INPUT_ID
        } else {
            NON_INPUT_ID
        })
    }
    fn apply(&mut self, action: Action) -> Task<Message> {
        let keep_editing_state = matches!(action, Action::Edit(_) | Action::Composition(_));
        if !self.model.apply(action) {
            return Task::none();
        }
        self.focus_generation = self.focus_generation.wrapping_add(1);
        if self.diagnostics {
            // Do not log the typed value, raw keys, window targets or app titles.
            eprintln!(
                "state focus={:?} feedback={:?} closing={} local_reduce={}",
                self.model.focus,
                self.model.feedback,
                self.model.closing,
                self.model.prefer_reduced_motion
            );
        }
        if self.model.closing {
            let outcome = self.bookmark.restore_if_owned();
            if self.diagnostics {
                eprintln!("graceful_exit restoration={outcome:?}");
            }
            iced::exit()
        } else if keep_editing_state {
            // Re-focusing an iced text input moves its cursor to the end. Never
            // do that on ordinary editing/composition; preserve caret/selection.
            Task::none()
        } else {
            self.focus_task()
        }
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        if self.model.closing {
            return Task::none();
        }
        match message {
            Message::Action(action) => self.apply(action),
            Message::PointerValidate => {
                self.model.apply(Action::Focus(Focus::Validate));
                self.apply(Action::Validate)
            }
            Message::PointerMotion => {
                self.model.apply(Action::Focus(Focus::Motion));
                self.apply(Action::ToggleMotion)
            }
            Message::PointerDismiss => {
                self.model.apply(Action::Focus(Focus::DismissError));
                self.apply(Action::DismissError)
            }
            Message::QueryInputFocus => {
                let generation = self.focus_generation;
                operation::is_focused(INPUT_ID).map(move |focused| Message::InputFocusChecked {
                    generation,
                    focused,
                })
            }
            Message::InputFocusChecked {
                generation,
                focused,
            } => {
                if focused && generation == self.focus_generation {
                    self.model.apply(Action::Focus(Focus::Input));
                    self.focus_generation = self.focus_generation.wrapping_add(1);
                }
                Task::none()
            }
            Message::WindowActive(active) => {
                self.window_active = active;
                if self.diagnostics {
                    eprintln!("window_active={active}");
                }
                if active {
                    self.refresh_motion();
                    self.focus_task()
                } else {
                    Task::none()
                }
            }
            Message::Opened => {
                if self.diagnostics {
                    eprintln!("window_opened renderer=tiny-skia");
                }
                if self.diagnostics {
                    let handle = window::oldest().and_then(|id| {
                        window::run(id, |window| {
                            use iced::window::raw_window_handle::RawWindowHandle;
                            let value = window
                                .window_handle()
                                .ok()
                                .and_then(|handle| match handle.as_raw() {
                                    RawWindowHandle::Win32(handle) => {
                                        Some(handle.hwnd.get() as usize)
                                    }
                                    _ => None,
                                })
                                .unwrap_or(0);
                            Message::DiagnosticHandle(value)
                        })
                    });
                    Task::batch([self.focus_task(), handle])
                } else {
                    self.focus_task()
                }
            }
            Message::DiagnosticHandle(handle) => {
                if self.diagnostics {
                    eprintln!("validation_hwnd={handle}");
                }
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        view::content(self)
    }
    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(local_event)
    }
}

/// Only local window events. Widget-consumed text/IME input is never intercepted.
fn local_event(event: Event, status: iced::event::Status, _: window::Id) -> Option<Message> {
    use keyboard::{Key, key::Named};
    match event {
        Event::InputMethod(iced::advanced::input_method::Event::Preedit(content, _)) => {
            Some(Message::Action(Action::Composition(!content.is_empty())))
        }
        Event::InputMethod(
            iced::advanced::input_method::Event::Commit(_)
            | iced::advanced::input_method::Event::Closed,
        ) => Some(Message::Action(Action::Composition(false))),
        Event::Window(window::Event::Opened { .. }) => Some(Message::Opened),
        Event::Window(window::Event::Focused) => Some(Message::WindowActive(true)),
        Event::Window(window::Event::Unfocused) => Some(Message::WindowActive(false)),
        Event::Window(window::Event::CloseRequested) => Some(Message::Action(Action::Close)),
        Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
            Some(Message::QueryInputFocus)
        }
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

/// Iced requires an executor for subscriptions/tasks. Use one idle-blocking
/// worker instead of a pool sized to every logical CPU; no timer/polling runtime.
struct OneWorker(iced::futures::executor::ThreadPool);
impl iced::Executor for OneWorker {
    fn new() -> Result<Self, std::io::Error> {
        iced::futures::executor::ThreadPoolBuilder::new()
            .pool_size(1)
            .name_prefix("sei-iced-")
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

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args
        .iter()
        .any(|flag| flag != "--diagnostics" && flag != "--error-fixture")
    {
        eprintln!("Usage: sei-desk [--diagnostics] [--error-fixture]");
        return ExitCode::from(2);
    }
    let diagnostics = args.iter().any(|flag| flag == "--diagnostics");
    let error_fixture = args.iter().any(|flag| flag == "--error-fixture");
    // Capture before iced creates/activates its window. This owns no registration.
    let bookmark = ForegroundBookmark::capture();
    let result = iced::application(
        move || App::boot(bookmark, diagnostics, error_fixture),
        App::update,
        App::view,
    )
    .title("Sei-Desk — Stage 2A validation")
    .executor::<OneWorker>()
    .subscription(App::subscription)
    .theme(view::theme)
    .default_font(iced::Font::with_name("Segoe UI"))
    .window(window::Settings {
        size: iced::Size::new(640.0, 500.0),
        min_size: Some(iced::Size::new(560.0, 480.0)),
        position: window::Position::Centered,
        exit_on_close_request: false,
        ..Default::default()
    })
    .run();
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
    fn key(key: keyboard::Key, modifiers: keyboard::Modifiers, repeat: bool) -> Event {
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: key.clone(),
            modified_key: key,
            modifiers,
            repeat,
            physical_key: keyboard::key::Physical::Unidentified(
                keyboard::key::NativeCode::Unidentified,
            ),
            location: keyboard::Location::Standard,
            text: None,
        })
    }
    #[test]
    fn captured_text_enter_does_not_validate_twice() {
        let event = key(
            keyboard::Key::Named(keyboard::key::Named::Enter),
            keyboard::Modifiers::empty(),
            false,
        );
        assert!(local_event(event, iced::event::Status::Captured, window::Id::unique()).is_none());
    }
    #[test]
    fn modifier_shortcuts_and_repeated_activation_are_ignored() {
        for modifiers in [
            keyboard::Modifiers::CTRL,
            keyboard::Modifiers::ALT,
            keyboard::Modifiers::LOGO,
        ] {
            assert!(
                local_event(
                    key(
                        keyboard::Key::Named(keyboard::key::Named::Enter),
                        modifiers,
                        false
                    ),
                    iced::event::Status::Ignored,
                    window::Id::unique()
                )
                .is_none()
            );
        }
        assert!(
            local_event(
                key(
                    keyboard::Key::Named(keyboard::key::Named::Enter),
                    keyboard::Modifiers::empty(),
                    true
                ),
                iced::event::Status::Ignored,
                window::Id::unique()
            )
            .is_none()
        );
    }
}
