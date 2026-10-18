//! Per-companion view state. Services and confirmed pins remain shared.
use crate::{
    applications::{self, Model},
    desktop::Message,
};
use iced::{
    Border, Color, Element, Font, Length, Task, Theme,
    widget::{Space, button, column, container, row, scrollable, text, text_input, tooltip},
    window,
};
use sei_apps::consumer::Status;
use sei_windows::launcher::Placement;
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum Action {
    Query(String),
    Move(i32),
    Select(String),
    Submit,
    Retry(applications::Key),
    Activate,
    Traverse(bool),
    Browse,
    Refresh,
    Dismiss,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Search,
    Results,
    Retry,
    Browse,
    Refresh,
    Dismiss,
}
pub struct Launcher {
    pub id: window::Id,
    pub token: u64,
    pub model: Model,
    pub composing: bool,
    pub focused: bool,
    pub focus: Focus,
    pub focus_generation: u64,
    pub shown: bool,
    pub deadline: Option<Instant>,
    pub placement: Placement,
    pub checking: Option<u64>,
    pub pending_launch: Option<u64>,
    pub launch_attempt: Option<applications::Key>,
}
impl Launcher {
    pub fn retry_key(&self) -> Option<applications::Key> {
        if self.pending_launch.is_none()
            && self.model.frame.error.is_none()
            && matches!(self.model.frame.launch, Status::Failed { .. })
            && self.launch_attempt == self.model.key()
        {
            self.launch_attempt.clone()
        } else {
            None
        }
    }
    pub fn search_id(&self) -> String {
        format!("companion-search-{}", self.token)
    }
    pub fn results_id(&self) -> String {
        format!("companion-results-{}", self.token)
    }
    pub fn focus_task(&self) -> Task<Message> {
        iced::widget::operation::focus(if self.focus == Focus::Search {
            self.search_id()
        } else {
            format!("companion-local-focus-{}", self.token)
        })
    }
    pub fn reveal(&self) -> Task<Message> {
        iced::widget::operation::scroll_to(
            self.results_id(),
            iced::widget::operation::AbsoluteOffset {
                x: None,
                y: Some(
                    self.model.position() as f32 * 58.0
                        + if self.focus == Focus::Retry {
                            58.0
                        } else {
                            0.0
                        },
                ),
            },
        )
    }
    pub fn traverse(&mut self, reverse: bool) {
        let mut order = vec![Focus::Search, Focus::Results];
        if self.retry_key().is_some() {
            order.push(Focus::Retry);
        }
        order.extend([Focus::Browse, Focus::Refresh, Focus::Dismiss]);
        let index = order.iter().position(|f| *f == self.focus).unwrap_or(0);
        self.focus = order[if reverse {
            (index + order.len() - 1) % order.len()
        } else {
            (index + 1) % order.len()
        }];
    }
}
const BACK: Color = iced::color!(0x293649);
const FIELD: Color = iced::color!(0x40536B);
const TEXT: Color = iced::color!(0xF4F7FC);
const SECONDARY: Color = iced::color!(0xCBD7E6);
const EDGE: Color = iced::color!(0x7188A4);
const MINT: Color = iced::color!(0xC5E8E3);
pub fn theme() -> Theme {
    Theme::custom(
        "Sei launcher",
        iced::theme::Palette {
            background: BACK,
            text: TEXT,
            primary: MINT,
            success: MINT,
            warning: SECONDARY,
            danger: TEXT,
        },
    )
}
fn control(
    label: &str,
    id: window::Id,
    action: Action,
    focus: bool,
    enabled: bool,
) -> Element<'_, Message> {
    button(text(label).size(13))
        .padding([8, 12])
        .on_press_maybe(enabled.then_some(Message::Launcher(id, action)))
        .style(move |_, status| button::Style {
            background: Some(
                if status == button::Status::Hovered {
                    FIELD
                } else {
                    BACK
                }
                .into(),
            ),
            text_color: if status == button::Status::Disabled {
                SECONDARY
            } else {
                TEXT
            },
            border: Border {
                color: if focus { MINT } else { EDGE },
                width: if focus { 2.0 } else { 1.0 },
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .into()
}
fn result_title(name: &str, selected: bool) -> Element<'_, Message> {
    let short: String = name.chars().take(52).collect();
    let short = if name.chars().count() > 52 {
        format!("{short}…")
    } else {
        short
    };
    tooltip(
        container(
            text(short)
                .size(14)
                .font(Font {
                    weight: if selected {
                        iced::font::Weight::Semibold
                    } else {
                        iced::font::Weight::Normal
                    },
                    ..Font::with_name("Segoe UI")
                })
                .wrapping(iced::widget::text::Wrapping::None),
        )
        .width(Length::Fill)
        .height(18)
        .clip(true),
        container(
            text(name)
                .size(13)
                .wrapping(iced::widget::text::Wrapping::WordOrGlyph),
        )
        .padding(8)
        .max_width(280)
        .style(|_| container::Style {
            background: Some(FIELD.into()),
            text_color: Some(TEXT),
            ..Default::default()
        }),
        tooltip::Position::Top,
    )
    .into()
}
pub fn view(launcher: &Launcher, service_busy: bool) -> Element<'_, Message> {
    let id = launcher.id;
    let model = &launcher.model;
    let search = applications::search::guarded(
        text_input("Search applications", &model.query)
            .id(launcher.search_id())
            .on_input(move |s| Message::Launcher(id, Action::Query(s)))
            .on_submit(Message::Launcher(id, Action::Submit))
            .padding(12)
            .size(15)
            .style(|_, status| text_input::Style {
                background: FIELD.into(),
                border: Border {
                    color: if matches!(status, text_input::Status::Focused { .. }) {
                        MINT
                    } else {
                        EDGE
                    },
                    width: 2.0,
                    radius: 8.0.into(),
                },
                icon: SECONDARY,
                placeholder: SECONDARY,
                value: TEXT,
                selection: iced::color!(0x526C7E),
            }),
    );
    let mut results = column![].spacing(0);
    if let Some(catalog) = &model.frame.catalog {
        for &index in &model.filtered {
            let entry = &catalog.entries[index];
            let selected = model.selected.as_ref() == Some(&entry.id);
            results = results.push(
                button(
                    row![
                        column![
                            result_title(&entry.name, selected),
                            text(if model.is_pinned(entry) {
                                "Favorite"
                            } else {
                                entry.source
                            })
                            .size(12)
                            .color(if selected {
                                TEXT
                            } else {
                                SECONDARY
                            })
                        ]
                        .spacing(4)
                        .width(Length::Fill),
                        text(if selected {
                            if launcher.retry_key().is_some() {
                                ""
                            } else {
                                "Enter"
                            }
                        } else {
                            ""
                        })
                        .size(12)
                        .color(TEXT)
                        .width(38),
                    ]
                    .spacing(12)
                    .align_y(iced::Alignment::Center),
                )
                .width(Length::Fill)
                .height(58)
                .padding([8, 12])
                .on_press(Message::Launcher(id, Action::Select(entry.id.clone())))
                .style(move |_, status| button::Style {
                    background: Some(
                        if selected {
                            iced::color!(0x526C7E)
                        } else if status == button::Status::Hovered {
                            FIELD
                        } else {
                            BACK
                        }
                        .into(),
                    ),
                    text_color: TEXT,
                    border: Border {
                        color: if selected { MINT } else { Color::TRANSPARENT },
                        width: if selected { 2.0 } else { 1.0 },
                        radius: 8.0.into(),
                    },
                    ..Default::default()
                }),
            );
            if selected
                && let Some(key) = launcher.retry_key()
                && let Status::Failed { error, .. } = &model.frame.launch
            {
                results = results.push(
                    container(
                        column![
                            row![
                                text("Couldn't launch.").size(13).color(TEXT),
                                Space::new().width(Length::Fill),
                                control(
                                    "Retry launch",
                                    id,
                                    Action::Retry(key),
                                    launcher.focus == Focus::Retry,
                                    !service_busy
                                ),
                            ]
                            .spacing(8)
                            .align_y(iced::Alignment::Center),
                            text(applications::error_text(error))
                                .size(13)
                                .color(TEXT)
                                .wrapping(iced::widget::text::Wrapping::WordOrGlyph),
                        ]
                        .spacing(8),
                    )
                    .padding(12)
                    .width(Length::Fill)
                    .style(|_| container::Style {
                        background: Some(FIELD.into()),
                        ..Default::default()
                    }),
                );
            }
        }
    }
    let empty = if model.frame.catalog.is_none() && model.refreshing() {
        "Loading applications…"
    } else if model.frame.catalog.is_none() {
        "Catalog unavailable. Choose Refresh to retry."
    } else if model.pinned_only && model.pins.pins.is_empty() {
        "No favorites yet. Choose All applications to browse."
    } else if model.filtered.is_empty() && model.query.trim().is_empty() {
        "No supported applications available."
    } else {
        "No applications found\nTry a different application name or check spelling."
    };
    let content: Element<'_, Message> = if model.filtered.is_empty() {
        container(text(empty).size(14).color(SECONDARY))
            .padding(12)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    } else {
        scrollable(results)
            .id(launcher.results_id())
            .height(Length::Fill)
            .into()
    };
    let feedback = if let Some(error) = &model.local_error {
        error.clone()
    } else if let Some(error) = &model.frame.error {
        applications::error_text(error)
    } else if let Status::Failed { error, .. } = &model.frame.launch
        && launcher.retry_key().is_none()
    {
        format!("Could not launch: {}", applications::error_text(error))
    } else if let Status::Failed { error, .. } = &model.frame.refresh {
        format!("Refresh failed: {}", applications::error_text(error))
    } else if service_busy {
        "A launch is pending…".into()
    } else if model.refreshing() {
        "Refreshing applications…".into()
    } else {
        format!(
            "{} · {} results",
            if model.pinned_only {
                "Favorites"
            } else {
                "Applications"
            },
            model.filtered.len()
        )
    };
    container(
        column![
            row![
                text("Launcher").size(18),
                Space::new().width(Length::Fill),
                control(
                    "Dismiss · Esc",
                    id,
                    Action::Dismiss,
                    launcher.focus == Focus::Dismiss,
                    true
                )
            ]
            .align_y(iced::Alignment::Center),
            search,
            row![
                control(
                    if model.pinned_only {
                        "All applications"
                    } else {
                        "Favorites"
                    },
                    id,
                    Action::Browse,
                    launcher.focus == Focus::Browse,
                    true
                ),
                control(
                    if matches!(model.frame.refresh, Status::Failed { .. }) {
                        "Retry refresh"
                    } else {
                        "Refresh"
                    },
                    id,
                    Action::Refresh,
                    launcher.focus == Focus::Refresh,
                    !model.refreshing()
                )
            ]
            .spacing(8),
            text(feedback).size(13).color(SECONDARY),
            content,
            text(if launcher.retry_key().is_some() {
                "Tab to Retry launch · Esc closes"
            } else {
                "Up/Down selects · Enter launches · Tab changes focus"
            })
            .size(12)
            .color(SECONDARY),
        ]
        .spacing(12),
    )
    .padding(16)
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_| container::Style {
        background: Some(BACK.into()),
        text_color: Some(TEXT),
        border: Border {
            color: EDGE,
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    })
    .into()
}

#[cfg(test)]
mod tests;
