use super::*;
use iced::advanced::{
    Layout, layout, mouse,
    renderer::{Headless, Style},
    widget::Tree,
};
use iced::widget::column;
use std::sync::Arc;

fn render(element: Element<'_, Message>, width: u32, height: u32) -> Vec<u8> {
    render_at(element, width, height, None)
}
fn render_at(
    mut element: Element<'_, Message>,
    width: u32,
    height: u32,
    scroll: Option<(String, f32)>,
) -> Vec<u8> {
    let mut renderer = iced::Renderer::new(Font::with_name("Segoe UI"), iced::Pixels(16.0));
    let viewport = iced::Rectangle::with_size(iced::Size::new(width as f32, height as f32));
    let mut tree = Tree::new(element.as_widget());
    let node = element.as_widget_mut().layout(
        &mut tree,
        &renderer,
        &layout::Limits::new(viewport.size(), viewport.size()),
    );
    if let Some((id, y)) = scroll {
        element.as_widget_mut().operate(
            &mut tree,
            Layout::new(&node),
            &renderer,
            &mut iced::advanced::widget::operation::scrollable::scroll_to::<()>(
                id.into(),
                iced::advanced::widget::operation::scrollable::AbsoluteOffset {
                    x: None,
                    y: Some(y),
                },
            ),
        );
    }
    element.as_widget().draw(
        &tree,
        &mut renderer,
        &theme(),
        &Style { text_color: TEXT },
        Layout::new(&node),
        mouse::Cursor::Unavailable,
        &viewport,
    );
    renderer.screenshot(iced::Size::new(width, height), 1.0, BACK)
}

#[test]
fn long_title_does_not_paint_over_secondary_text_or_outside_its_column() {
    for selected in [false, true] {
        let short = render(
            column![
                result_title("Short", selected),
                text("Application").size(12)
            ]
            .spacing(4)
            .width(Length::Fill)
            .into(),
            180,
            50,
        );
        let long = render(
            column![
                result_title(&"Wide application title ".repeat(8), selected),
                text("Application").size(12)
            ]
            .spacing(4)
            .width(Length::Fill)
            .into(),
            180,
            50,
        );
        assert_ne!(short, long);
        assert_eq!(
            &short[180 * 22 * 4..],
            &long[180 * 22 * 4..],
            "title must not paint into subtitle"
        );
    }
}

#[test]
#[ignore = "opt-in controlled render evidence; set SEI_LAUNCHER_RENDER_DIR under target"]
fn launcher_controlled_state_render_evidence() {
    // Optional ignored-target artifacts; no native windows, real catalog or launch.
    let root = std::env::var_os("SEI_LAUNCHER_RENDER_DIR")
        .expect("set controlled render output directory");
    let root = std::path::PathBuf::from(root);
    std::fs::create_dir_all(&root).unwrap();
    let monitor = crate::Monitor::start().unwrap();
    let mut desktop = crate::desktop::Desktop::new(crate::App::boot(monitor.feed(), false));
    desktop.manager.apps = applications::tests::loaded();
    desktop.shared = desktop.manager.apps.frame.clone();
    crate::desktop::tests::open(&mut desktop, 1);
    let popup = desktop.popup.as_mut().unwrap();
    for state in [
        "results",
        "long-title",
        "no-match",
        "launch-failure",
        "loading",
        "refresh-failure",
        "empty-favorites",
    ] {
        popup.model = applications::tests::loaded();
        popup.launch_attempt = None;
        popup.focus = Focus::Search;
        match state {
            "long-title" => {
                let frame = Arc::get_mut(&mut popup.model.frame).unwrap();
                let catalog = Arc::get_mut(frame.catalog.as_mut().unwrap()).unwrap();
                catalog.entries[0].name =
                    "Wide application title without a short display label ".repeat(5);
            }
            "no-match" => popup.model.edit("unmatched-query".into()),
            "launch-failure" => {
                popup.launch_attempt = popup.model.key();
                popup.focus = Focus::Retry;
                let frame = Arc::get_mut(&mut popup.model.frame).unwrap();
                frame.launch = Status::Failed {
                    id: sei_apps::consumer::Consumer::new()
                        .request_refresh()
                        .unwrap(),
                    error: sei_apps::consumer::ConsumerError::StaleSelection,
                };
            }
            "loading" => {
                popup.model = Model::default();
                popup.model.mark_refresh(1);
            }
            "refresh-failure" => {
                let frame = Arc::get_mut(&mut popup.model.frame).unwrap();
                frame.refresh = Status::Failed {
                    id: sei_apps::consumer::Consumer::new()
                        .request_refresh()
                        .unwrap(),
                    error: sei_apps::consumer::ConsumerError::StaleSelection,
                };
            }
            "empty-favorites" => {
                popup.model.pinned_only = true;
                popup.model.filter();
            }
            _ => (),
        }
        for (width, height) in [(580, 440), (360, 300)] {
            let scroll = (popup.focus == Focus::Retry).then(|| {
                (
                    popup.results_id(),
                    popup.model.position() as f32 * 58.0 + 58.0,
                )
            });
            let bytes = render_at(view(popup, false), width, height, scroll);
            assert_eq!(bytes.len(), (width * height * 4) as usize);
            std::fs::write(root.join(format!("{state}-{width}x{height}.rgba")), bytes).unwrap();
        }
    }
}
