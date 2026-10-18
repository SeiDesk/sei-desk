use super::*;
use crate::{Monitor, Page};
fn setup() -> (Monitor, Desktop) {
    let monitor = Monitor::start().unwrap();
    let mut desktop = Desktop::new(App::boot(monitor.feed(), false));
    desktop.manager_id = Some(window::Id::unique());
    desktop.manager.apps = applications::tests::loaded();
    desktop.shared = desktop.manager.apps.frame.clone();
    (monitor, desktop)
}
pub(crate) fn open(desktop: &mut Desktop, token: u64) -> window::Id {
    desktop.ticket = token;
    desktop.opening = Some((token, Instant::now() + Duration::from_secs(3)));
    let p = Placement::new(
        sei_windows::launcher::Rect {
            left: 0,
            top: 0,
            right: 1920,
            bottom: 1040,
        },
        96,
    )
    .unwrap();
    let _ = desktop.update(Message::Geometry(token, Ok(p)));
    let popup = desktop.popup.as_mut().unwrap();
    popup.shown = true;
    popup.focused = true;
    popup.deadline = None;
    popup.id
}
#[test]
fn launcher_opens_and_reopens_with_all_applications() {
    let (_monitor, mut desktop) = setup();
    desktop.manager.apps.pinned_only = true;
    desktop.manager.apps.edit("béta".into());
    let id = open(&mut desktop, 1);
    let model = &desktop.popup.as_ref().unwrap().model;
    assert!(!model.pinned_only);
    assert_eq!(model.filtered.len(), 2);
    assert_eq!(model.selected.as_deref(), Some("a"));
    let _ = desktop.update(Message::Launcher(id, Action::Browse));
    assert!(desktop.popup.as_ref().unwrap().model.pinned_only);
    assert!(desktop.popup.as_ref().unwrap().model.filtered.is_empty());
    let _ = desktop.update(Message::Launcher(id, Action::Dismiss));
    let _ = desktop.update(Message::Event(
        id,
        Event::Window(window::Event::Closed),
        iced::event::Status::Ignored,
    ));
    open(&mut desktop, 2);
    let model = &desktop.popup.as_ref().unwrap().model;
    assert!(!model.pinned_only);
    assert_eq!(model.filtered.len(), 2);
    assert_eq!(model.query, "");
    assert!(desktop.manager.apps.pinned_only);
    assert_eq!(desktop.manager.apps.query, "béta");
}
#[test]
fn changing_query_clears_local_launch_delivery_error() {
    let (_monitor, mut desktop) = setup();
    let id = open(&mut desktop, 1);
    let _ = desktop.update(Message::Launcher(id, Action::Query("alpha".into())));
    let _ = desktop.update(Message::Launcher(id, Action::Submit));
    assert!(desktop.popup.as_ref().unwrap().model.local_error.is_some());
    let _ = desktop.update(Message::Launcher(id, Action::Query("béta".into())));
    let model = &desktop.popup.as_ref().unwrap().model;
    assert_eq!(model.selected.as_deref(), Some("b"));
    assert!(model.local_error.is_none());
}
#[test]
fn enter_without_results_preserves_empty_state_and_browse_recovery() {
    let (_monitor, mut desktop) = setup();
    desktop.manager.apps.edit("béta".into());
    let manager_key = desktop.manager.apps.key();
    let id = open(&mut desktop, 1);
    {
        let popup = desktop.popup.as_mut().unwrap();
        popup.model.pinned_only = true;
        popup.model.filter();
        popup.focus = Focus::Search;
    }
    for _ in 0..2 {
        let _ = desktop.update(Message::Launcher(id, Action::Submit));
    }
    let popup = desktop.popup.as_ref().unwrap();
    assert!(popup.model.filtered.is_empty());
    assert!(popup.model.local_error.is_none());
    assert!(popup.pending_launch.is_none());
    assert_eq!(popup.focus, Focus::Search);
    let _ = desktop.update(Message::Launcher(id, Action::Browse));
    let model = &desktop.popup.as_ref().unwrap().model;
    assert_eq!(model.selected.as_deref(), Some("a"));
    assert!(model.local_error.is_none());
    let _ = desktop.update(Message::Launcher(id, Action::Query("no-such-app".into())));
    let _ = desktop.update(Message::Launcher(id, Action::Submit));
    let popup = desktop.popup.as_ref().unwrap();
    assert_eq!(popup.model.query, "no-such-app");
    assert!(popup.model.filtered.is_empty());
    assert!(popup.model.local_error.is_none());
    assert!(popup.pending_launch.is_none());
    let _ = desktop.update(Message::Launcher(id, Action::Query(String::new())));
    assert_eq!(
        desktop.popup.as_ref().unwrap().model.selected.as_deref(),
        Some("a")
    );
    assert_eq!(desktop.manager.apps.query, "béta");
    assert_eq!(desktop.manager.apps.key(), manager_key);
}
#[test]
fn popup_query_navigation_and_close_do_not_change_the_manager() {
    let (_monitor, mut desktop) = setup();
    desktop.manager.page = Page::Applications;
    desktop.manager.apps.edit("alpha".into());
    let selected = desktop.manager.apps.selected.clone();
    let id = open(&mut desktop, 1);
    let _ = desktop.update(Message::Launcher(id, Action::Query("béta".into())));
    assert_eq!(
        desktop.popup.as_ref().unwrap().model.selected.as_deref(),
        Some("b")
    );
    let _ = desktop.update(Message::Event(
        id,
        Event::Window(window::Event::Resized(iced::Size::new(400.0, 300.0))),
        iced::event::Status::Ignored,
    ));
    let _ = desktop.update(Message::Launcher(id, Action::Dismiss));
    assert_eq!(desktop.manager.apps.query, "alpha");
    assert_eq!(desktop.manager.apps.selected, selected);
    assert_eq!(desktop.manager.width, 1100.0);
    assert!(!desktop.closing);
}
#[test]
fn repeated_invocation_reuses_one_window_and_old_messages_cannot_target_its_replacement() {
    let (_monitor, mut desktop) = setup();
    let id = open(&mut desktop, 1);
    let _ = desktop.open();
    let _ = desktop.open();
    assert_eq!(desktop.popup.as_ref().unwrap().id, id);
    assert_eq!(desktop.ticket, 2); // one outstanding inspection, not one per invocation
    let _ = desktop.update(Message::Launcher(id, Action::Dismiss));
    let _ = desktop.open();
    assert!(desktop.opening.is_none()); // wait for actual destruction before replacement
    let _ = desktop.update(Message::Event(
        id,
        Event::Window(window::Event::Closed),
        iced::event::Status::Ignored,
    ));
    let next = open(&mut desktop, 3);
    let _ = desktop.update(Message::Launcher(next, Action::Query("alpha".into())));
    for message in [
        Message::Launcher(id, Action::Query("wrong".into())),
        Message::Placed(id, Err("late placement".into())),
        Message::ObservedFocus(id, 0, false),
        Message::Geometry(2, Err("late inspection".into())),
        Message::Event(
            id,
            Event::Window(window::Event::CloseRequested),
            iced::event::Status::Ignored,
        ),
    ] {
        let _ = desktop.update(message);
    }
    assert_eq!(desktop.popup.as_ref().unwrap().id, next);
    assert_eq!(desktop.popup.as_ref().unwrap().model.query, "alpha");
    assert!(desktop.error.is_none());
    assert!(!desktop.closing);
}
#[test]
fn stale_launch_success_and_error_are_not_delivered_to_replacement_or_manager() {
    let (_monitor, mut desktop) = setup();
    let id = open(&mut desktop, 7);
    let _ = desktop.update(Message::Launcher(id, Action::Dismiss));
    desktop.closing_popup = None;
    let next = open(&mut desktop, 8);
    let frame = Frame {
        revision: 2,
        ack: 20,
        launch_origin: 7,
        error_origin: 7,
        launch: Status::Succeeded {
            id: operation_id(),
            value: sei_apps::consumer::Accepted { pid: 42 },
        },
        error: Some(sei_apps::consumer::ConsumerError::StaleSelection),
        ..(*desktop.shared).clone()
    };
    let _ = desktop.update(Message::Manager(ManagerMessage::Applications(Arc::new(
        frame,
    ))));
    assert_eq!(desktop.popup.as_ref().unwrap().id, next);
    assert!(matches!(
        desktop.popup.as_ref().unwrap().model.frame.launch,
        Status::Idle
    ));
    assert!(matches!(desktop.manager.apps.frame.launch, Status::Idle));
    assert!(desktop.manager.apps.frame.error.is_none());
    assert!(desktop.popup.as_ref().unwrap().model.frame.error.is_none());
}
fn operation_id() -> sei_apps::consumer::OperationId {
    // Allocate a real ID through the public coordinator, not a fabricated integer.
    sei_apps::consumer::Consumer::new()
        .request_refresh()
        .unwrap()
}
#[test]
fn invalid_placement_and_unconfirmed_focus_fail_closed_without_retrying_or_exiting() {
    let (_monitor, mut desktop) = setup();
    desktop.opening = Some((1, Instant::now()));
    let _ = desktop.update(Message::Geometry(1, Err("No monitor".into())));
    assert!(desktop.popup.is_none());
    assert!(desktop.error.is_some());
    let id = open(&mut desktop, 2);
    let _ = desktop.update(Message::ObservedFocus(id, 0, false));
    desktop.popup.as_mut().unwrap().deadline = Some(Instant::now() - Duration::from_secs(1));
    let _ = desktop.update(Message::Manager(ManagerMessage::Monitor(
        crate::Update::Pulse("clock".into()),
    )));
    assert!(desktop.popup.is_none());
    assert!(!desktop.closing);
    assert!(desktop.opening.is_none());
}

#[test]
fn focus_changes_invalidate_old_observations_and_manager_loss_cancels_pending_open() {
    let (_monitor, mut desktop) = setup();
    let id = open(&mut desktop, 1);
    let _ = desktop.update(Message::Event(
        id,
        Event::Window(window::Event::Unfocused),
        iced::event::Status::Ignored,
    ));
    let _ = desktop.update(Message::Event(
        id,
        Event::Window(window::Event::Focused),
        iced::event::Status::Ignored,
    ));
    let _ = desktop.update(Message::ObservedFocus(id, 0, false));
    assert!(desktop.popup.as_ref().unwrap().focused);
    assert!(desktop.popup.as_ref().unwrap().deadline.is_none());
    let _ = desktop.update(Message::Launcher(id, Action::Browse));
    let _ = desktop.update(Message::SearchFocus(id, 0, true));
    assert_eq!(
        desktop.popup.as_ref().unwrap().focus,
        crate::launcher::Focus::Browse
    );
    let _ = desktop.update(Message::Launcher(id, Action::Dismiss));
    desktop.closing_popup = None;
    let _ = desktop.open();
    let ticket = desktop.ticket;
    let _ = desktop.update(Message::Event(
        desktop.manager_id.unwrap(),
        Event::Window(window::Event::Unfocused),
        iced::event::Status::Ignored,
    ));
    assert!(desktop.opening.is_none());
    let plan = Placement::new(
        sei_windows::launcher::Rect {
            left: 0,
            top: 0,
            right: 1920,
            bottom: 1080,
        },
        96,
    )
    .unwrap();
    let _ = desktop.update(Message::Geometry(ticket, Ok(plan)));
    assert!(desktop.popup.is_none());
}

#[test]
fn manager_destruction_exits_even_with_a_companion_and_ignores_late_work() {
    let (_monitor, mut desktop) = setup();
    let id = open(&mut desktop, 1);
    let _ = desktop.update(Message::Event(
        desktop.manager_id.unwrap(),
        Event::Window(window::Event::Closed),
        iced::event::Status::Ignored,
    ));
    assert!(desktop.closing);
    assert!(desktop.popup.is_none());
    let _ = desktop.update(Message::Launcher(id, Action::Query("late".into())));
    assert_eq!(desktop.manager.apps.query, "");
}

#[test]
fn an_old_geometry_reply_cannot_replace_a_newer_inspection_after_reuse() {
    let (_monitor, mut desktop) = setup();
    let id = open(&mut desktop, 1);
    let _ = desktop.geometry(false); // old periodic inspection
    let old_ticket = desktop.ticket;
    let _ = desktop.open(); // explicit reuse supersedes it
    let ticket = desktop.ticket;
    let plan = desktop.popup.as_ref().unwrap().placement;
    let _ = desktop.update(Message::Geometry(ticket, Ok(plan)));
    let _ = desktop.geometry(false); // next inspection is now pending
    let _ = desktop.update(Message::Geometry(
        old_ticket,
        Err("late old geometry".into()),
    ));
    assert_eq!(desktop.popup.as_ref().map(|p| p.id), Some(id));
    assert!(desktop.error.is_none());
}
#[test]
fn actual_companion_widget_and_routed_events_apply_enter_policy_and_keep_manager_unchanged() {
    use iced::advanced::{Layout, Shell, layout, mouse, widget::Tree};
    use iced::keyboard::{Key, Location, Modifiers, key};
    let (_monitor, mut desktop) = setup();
    let id = open(&mut desktop, 1);
    let renderer = iced::Renderer::new(iced::Font::DEFAULT, iced::Pixels(16.0));
    let viewport = iced::Rectangle::with_size(iced::Size::new(580.0, 440.0));
    let mut actual = vec![];
    for (modifiers, repeat) in [
        (Modifiers::empty(), false),
        (Modifiers::CTRL, false),
        (Modifiers::ALT, false),
        (Modifiers::LOGO, false),
        (Modifiers::empty(), true),
    ] {
        let mut element = desktop.view(id);
        let mut tree = Tree::new(element.as_widget());
        let node = element.as_widget_mut().layout(
            &mut tree,
            &renderer,
            &layout::Limits::new(iced::Size::ZERO, viewport.size()),
        );
        let layout = Layout::new(&node);
        element.as_widget_mut().operate(
            &mut tree,
            layout,
            &renderer,
            &mut iced::advanced::widget::operation::focusable::focus::<()>(
                "companion-search-1".into(),
            ),
        );
        let mut messages = vec![];
        let mut shell = Shell::new(&mut messages);
        let mut clipboard = iced::advanced::clipboard::Null;
        let event = Event::Keyboard(keyboard::Event::KeyPressed {
            key: Key::Named(key::Named::Enter),
            modified_key: Key::Named(key::Named::Enter),
            physical_key: key::Physical::Unidentified(key::NativeCode::Unidentified),
            location: Location::Standard,
            modifiers,
            text: None,
            repeat,
        });
        element.as_widget_mut().update(
            &mut tree,
            &event,
            layout,
            mouse::Cursor::Unavailable,
            &renderer,
            &mut clipboard,
            &mut shell,
            &viewport,
        );
        let status = shell.event_status();
        drop(element);
        actual.push(
            messages
                .iter()
                .filter(|m| matches!(m, Message::Launcher(_, Action::Submit)))
                .count(),
        );
        for message in messages {
            let _ = desktop.update(message);
        }
        let _ = desktop.update(Message::Event(id, event, status));
    }
    assert_eq!(actual, [1, 0, 0, 0, 0]);
    assert_eq!(desktop.manager.apps.query, "");
    assert!(desktop.popup.as_ref().unwrap().model.local_error.is_some()); // unavailable service retains UI
    let _ = desktop.update(Message::Event(
        id,
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: Key::Named(key::Named::Escape),
            modified_key: Key::Named(key::Named::Escape),
            physical_key: key::Physical::Unidentified(key::NativeCode::Unidentified),
            location: Location::Standard,
            modifiers: Modifiers::empty(),
            text: None,
            repeat: false,
        }),
        iced::event::Status::Captured,
    ));
    assert!(desktop.popup.is_none());
    assert!(!desktop.closing);
}
