//! Opaque reference-derived pastel styling for the validation window.
//! Visual labels and local focus are not an implemented UIA semantic tree.
use super::{
    App, INPUT_ID, Message,
    model::{Action, Feedback, Focus},
};
use iced::{
    Border, Color, Element, Length, Theme,
    widget::{button, checkbox, column, container, row, text, text_input},
};

const CANVAS: Color = iced::color!(0xEFF1F7);
const PANEL: Color = iced::color!(0xE3EAF3);
const SLATE: Color = iced::color!(0x293649);
const SECONDARY: Color = iced::color!(0x53647A);
const MINT: Color = iced::color!(0xC5E8E3);
const PINK: Color = iced::color!(0xF3B8C8);
const FOCUS: Color = iced::color!(0x44638A);
const EDGE: Color = iced::color!(0xB7C6DA);

pub fn theme(_: &App) -> Theme {
    Theme::custom(
        "Sei pastel",
        iced::theme::Palette {
            background: CANVAS,
            text: SLATE,
            primary: FOCUS,
            success: FOCUS,
            warning: SECONDARY,
            danger: SLATE,
        },
    )
}

fn action<'a>(label: &'a str, message: Message, focused: bool) -> Element<'a, Message> {
    button(text(label).size(14))
        .on_press(message)
        .padding([10, 16])
        .style(move |_, status| button::Style {
            background: Some(
                if status == button::Status::Hovered {
                    PANEL
                } else {
                    MINT
                }
                .into(),
            ),
            text_color: SLATE,
            border: Border {
                color: if focused { FOCUS } else { EDGE },
                width: if focused { 2.0 } else { 1.0 },
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .into()
}

pub fn content(app: &App) -> Element<'_, Message> {
    let focused = |target| app.window_active && app.model.focus == target;
    let input = text_input("Type sample text", &app.model.value)
        .id(INPUT_ID)
        .on_input(|value| Message::Action(Action::Edit(value)))
        .on_submit(Message::Action(Action::Validate))
        .padding(10)
        .size(14)
        .style(|_, status| text_input::Style {
            background: Color::WHITE.into(),
            border: Border {
                color: if matches!(status, text_input::Status::Focused { .. }) {
                    FOCUS
                } else {
                    EDGE
                },
                width: if matches!(status, text_input::Status::Focused { .. }) {
                    2.0
                } else {
                    1.0
                },
                radius: 8.0.into(),
            },
            icon: SECONDARY,
            placeholder: SECONDARY,
            value: SLATE,
            selection: MINT,
        });
    let motion = container(
        checkbox(app.model.prefer_reduced_motion)
            .label("Prefer reduced motion")
            .on_toggle(|_| Message::PointerMotion)
            .size(20)
            .text_size(14),
    )
    .padding(8)
    .width(Length::Fill)
    .style(move |_| container::Style {
        border: Border {
            color: if focused(Focus::Motion) {
                FOCUS
            } else {
                Color::TRANSPARENT
            },
            width: 2.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    });
    let status: Element<'_, Message> = match app.model.feedback {
        Feedback::Empty => container(
            row![
                text("Enter some text before validating.")
                    .size(14)
                    .width(Length::Fill),
                action(
                    "Dismiss error",
                    Message::PointerDismiss,
                    focused(Focus::DismissError)
                )
            ]
            .spacing(8)
            .align_y(iced::Alignment::Center),
        )
        .padding(8)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(PINK.into()),
            text_color: Some(SLATE),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into(),
        Feedback::Valid => text("The sample is valid.").size(14).into(),
        Feedback::Idle => text("Enter text, then choose Validate.")
            .size(14)
            .color(SECONDARY)
            .into(),
    };
    let motion_note = if app.model.effective_reduced_motion(app.system_reduce) {
        match app.system_reduce {
            Some(true) => "Reduced motion · requested by Windows",
            Some(false) => "Reduced motion · local preference",
            None => "Motion preference unavailable · reduced motion assumed",
        }
    } else {
        "Animation allowed · this validation window has no animations"
    };
    let content = column![
        column![text("Sei-Desk").size(24),
            text("Keyboard and window validation").size(12).color(SECONDARY)].spacing(4),
        column![text("Sample text").size(14), input].spacing(4),
        row![action("Validate", Message::PointerValidate, focused(Focus::Validate)),
            motion].spacing(12).align_y(iced::Alignment::Center),
        text(motion_note).size(12).color(SECONDARY),
        status,
        row![action("Close", Message::Action(Action::Close), focused(Focus::Close)),
            text("Tab / Shift+Tab: move focus · Enter / Space: activate\nEscape: dismiss error, then close").size(12).color(SECONDARY)].spacing(16).align_y(iced::Alignment::Center),
    ].spacing(12).padding(24);
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
