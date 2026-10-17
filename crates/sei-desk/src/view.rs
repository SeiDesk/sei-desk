//! Pastel management surfaces and slate rail from approved design plate 05.
use super::{App, Battery, Capacity, Message, Page, traffic, traffic_chart};
use iced::{
    Border, Color, Element, Length, Theme,
    widget::{Space, button, column, container, progress_bar, row, scrollable, text, text_input},
};
use sei_windows::network::Status as NetworkStatus;
const CANVAS: Color = iced::color!(0xEFF1F7);
const PANEL: Color = iced::color!(0xE3EAF3);
const SLATE: Color = iced::color!(0x293649);
const SECONDARY: Color = iced::color!(0x53647A);
const MINT: Color = iced::color!(0xC5E8E3);
const LAVENDER: Color = iced::color!(0xCBC2DE);
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
fn surface(color: Color) -> container::Style {
    container::Style {
        background: Some(color.into()),
        text_color: Some(SLATE),
        border: Border {
            color: EDGE,
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}
fn note<'a>(value: impl Into<String>) -> iced::widget::Text<'a> {
    text(value.into()).size(13).color(SECONDARY)
}
fn nav(app: &App, page: Page, label: &str) -> Element<'static, Message> {
    let selected = app.page == page;
    let focused =
        app.focus == Some(page) && (app.page != Page::Applications || app.apps.focus.is_none());
    button(text(format!("{}{}", if selected { "•  " } else { "   " }, label)).size(15))
        .width(Length::Fill)
        .padding([12, 10])
        .on_press(Message::Navigate(page))
        .style(move |_, status| button::Style {
            background: Some(
                if selected {
                    MINT
                } else if status == button::Status::Hovered {
                    CANVAS
                } else {
                    PANEL
                }
                .into(),
            ),
            text_color: SLATE,
            border: Border {
                color: if focused { FOCUS } else { Color::TRANSPARENT },
                width: 2.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .into()
}
struct Metric {
    title: String,
    value: String,
    detail: String,
    caption: String,
    percent: Option<f32>,
    tint: Color,
}
fn capacity(
    title: &str,
    value: Option<&Result<Capacity, sei_windows::NativeError>>,
    tint: Color,
    loading: bool,
) -> Metric {
    match value {
        Some(Ok(cap)) if cap.used().is_some() => Metric {
            title: title.into(),
            value: format!(
                "{:.1} / {:.1} GB",
                cap.used().unwrap_or(0) as f64 / 1e9,
                cap.total as f64 / 1e9
            ),
            detail: format!(
                "{:.1}% used · {:.1} GB available",
                cap.percent().unwrap_or(0.0),
                cap.available as f64 / 1e9
            ),
            caption: "Capacity · decimal GB".into(),
            percent: cap.percent(),
            tint,
        },
        _ => Metric {
            title: title.into(),
            value: if loading {
                "Collecting…"
            } else {
                "Unavailable"
            }
            .into(),
            detail: match value {
                Some(Err(error)) => error.to_string(),
                _ => "Waiting for a valid capacity reading".into(),
            },
            caption: "Capacity · decimal GB".into(),
            percent: None,
            tint,
        },
    }
}
fn card(metric: Metric) -> Element<'static, Message> {
    let bar: Element<'static, Message> = if let Some(percent) = metric.percent {
        progress_bar(0.0..=100.0, percent)
            .girth(6)
            .style(|_| progress_bar::Style {
                background: EDGE.into(),
                bar: FOCUS.into(),
                border: Border {
                    radius: 3.0.into(),
                    ..Default::default()
                },
            })
            .into()
    } else {
        Space::new().height(6).into()
    };
    container(
        column![
            row![
                container(Space::new().width(7).height(18)).style(move |_| container::Style {
                    background: Some(metric.tint.into()),
                    border: Border {
                        radius: 3.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                text(metric.title).size(16)
            ]
            .spacing(9),
            text(metric.value).size(27),
            note(metric.detail),
            Space::new().height(Length::Fill),
            bar,
            note(metric.caption),
        ]
        .spacing(10),
    )
    .padding(18)
    .height(205)
    .width(Length::Fill)
    .style(|_| surface(PANEL))
    .into()
}
fn network_state(app: &App) -> String {
    match app.network.as_ref() {
        None => "Collecting network counters…".into(),
        Some(reading) if reading.stale(app.now) => {
            "Data paused · last network reading is stale".into()
        }
        Some(reading) => match &reading.status {
            NetworkStatus::Live(_) => format!(
                "{} physical adapter(s) · interval average",
                reading.cohort.len()
            ),
            NetworkStatus::Baseline(reason) | NetworkStatus::Unavailable(reason) => {
                (*reason).into()
            }
            NetworkStatus::Failed(error) => error.to_string(),
        },
    }
}
fn network_card(app: &App) -> Element<'static, Message> {
    let rates = app
        .network
        .as_ref()
        .filter(|reading| !reading.stale(app.now))
        .and_then(|reading| {
            if let NetworkStatus::Live(rates) = reading.status {
                Some(rates)
            } else {
                None
            }
        });
    let rx = rates
        .map(|r| traffic::format_rate(r.receive))
        .unwrap_or_else(|| "—".into());
    let tx = rates
        .map(|r| traffic::format_rate(r.transmit))
        .unwrap_or_else(|| "—".into());
    container(
        column![
            text("Network").size(16),
            row![
                text("Receive")
                    .size(13)
                    .color(traffic_chart::RECEIVE)
                    .width(76),
                text(rx).size(22)
            ]
            .spacing(4),
            row![
                text("Transmit")
                    .size(13)
                    .color(traffic_chart::TRANSMIT)
                    .width(76),
                text(tx).size(22)
            ]
            .spacing(4),
            note(network_state(app)),
        ]
        .spacing(10),
    )
    .padding(18)
    .height(205)
    .width(Length::Fill)
    .style(|_| surface(PANEL))
    .into()
}
fn network_history(app: &App) -> Element<'_, Message> {
    let plot: Element<'_, Message> = if app.traffic.len() == 0 {
        container(note(
            "No valid traffic samples yet. Rates need two counter readings.",
        ))
        .height(88)
        .center_y(88)
        .into()
    } else {
        traffic_chart::chart(&app.traffic, app.now)
    };
    container(column![
        text("Traffic history").size(17),
        row![text("Receive · blue").size(13).color(traffic_chart::RECEIVE), text("Transmit · lavender").size(13).color(traffic_chart::TRANSMIT)].spacing(20),
        note(format!("Scale 0–{} · {} measured intervals", traffic::format_rate(app.traffic.peak()), app.traffic.len())),
        plot,
        row![note("60 s ago"), Space::new().width(Length::Fill), note("Now")],
        note("Last 60 s · time-spaced bars · gaps are unmeasured. Ethernet + Wi-Fi hardware only; traffic does not establish internet connectivity."),
    ].spacing(8)).padding(18).width(Length::Fill).style(|_| surface(PANEL)).into()
}
fn overview(app: &App) -> Element<'_, Message> {
    let paused = app.paused();
    let loading = app.snapshot.is_none();
    let snapshot = app.snapshot.as_ref();
    let cpu_detail = match snapshot.map(|value| &value.cpu) {
        Some(Err(error)) => error.to_string(),
        _ if app.cpu.is_none() => "Waiting for two valid samples".into(),
        _ => "Average over the last sample interval".into(),
    };
    let cpu = Metric {
        title: "CPU".into(),
        value: app
            .cpu
            .map(|value| format!("{value:.1}%"))
            .unwrap_or_else(|| {
                if loading {
                    "Collecting…"
                } else if snapshot.is_some_and(|s| s.cpu.is_err()) {
                    "Unavailable"
                } else {
                    "Sampling…"
                }
                .into()
            }),
        detail: cpu_detail,
        caption: "Processor group · sampled every 2 s".into(),
        percent: app.cpu,
        tint: LAVENDER,
    };
    let memory = capacity("Memory", snapshot.map(|s| &s.memory), MINT, loading);
    let disk_title = snapshot
        .filter(|s| !s.disk_root.is_empty())
        .map(|s| format!("Disk ({})", s.disk_root.trim_end_matches('\\')))
        .unwrap_or_else(|| "System disk".into());
    let mut disk = capacity(&disk_title, snapshot.map(|s| &s.disk), PINK, loading);
    disk.caption = "Capacity · activity not measured".into();
    let network = network_card(app);
    let cards: Element<'static, Message> = if app.width >= 1180.0 {
        row![card(cpu), card(memory), card(disk), network]
            .spacing(14)
            .into()
    } else {
        column![
            row![card(cpu), card(memory)].spacing(14),
            row![card(disk), network].spacing(14)
        ]
        .spacing(14)
        .into()
    };
    let battery = match snapshot.map(|s| &s.battery) {
        None => "Collecting battery status…".into(),
        Some(Err(error)) => format!("Unavailable · {error}"),
        Some(Ok(Battery::Absent)) => "No battery detected".into(),
        Some(Ok(Battery::Unknown)) => "Battery status unavailable".into(),
        Some(Ok(Battery::Present {
            percent,
            charging,
            ac,
        })) => format!(
            "{} · {}",
            percent
                .map(|p| format!("{p}%"))
                .unwrap_or_else(|| "Charge unavailable".into()),
            if *charging {
                "Charging"
            } else {
                match ac {
                    Some(true) => "Plugged in",
                    Some(false) => "On battery",
                    None => "Power source unknown",
                }
            }
        ),
    };
    let status = if let Some(error) = &app.error {
        error.clone()
    } else if app.paused() {
        "Data paused · showing the last received readings".into()
    } else if loading {
        "Collecting readings…".into()
    } else {
        format!(
            "Updated {} s ago · samples every 2 s",
            app.sampled_at
                .map(|at| app.now.saturating_duration_since(at).as_secs())
                .unwrap_or(0)
        )
    };
    column![
        text("Overview").size(30), note("Your system, at a glance"),
        container(text(status).size(14)).padding([10, 14]).width(Length::Fill).style(move |_| surface(if paused { PINK } else { CANVAS })),
        cards,
        network_history(app),
        container(column![text("Power").size(17), text(battery).size(16), note("Status reported by Windows")].spacing(7)).padding(18).width(Length::Fill).style(|_| surface(PANEL)),
        note("Read-only monitoring. Capacity readings are instantaneous; CPU uses counter differences. No temperature or hardware-health assessment."),
    ].spacing(14).into()
}
fn app_button<'a>(
    label: impl Into<String>,
    action: Option<super::applications::Action>,
    focused: bool,
) -> Element<'a, Message> {
    use super::applications::{Action, Focus};
    let id = match &action {
        Some(Action::Filter) => Focus::Filter.id(),
        Some(Action::Presentation) => Focus::Presentation.id(),
        Some(Action::Sort) => Focus::Sort.id(),
        Some(Action::Group) => Focus::Group.id(),
        Some(Action::Refresh) => Focus::Refresh.id(),
        Some(Action::Launch) => Focus::Launch.id(),
        Some(Action::Cancel) => Focus::Cancel.id(),
        Some(Action::Pin) => Focus::Pin.id(),
        _ => "apps-disabled",
    };
    container(
        button(text(label.into()).size(14))
            .padding([10, 16])
            .on_press_maybe(action.map(Message::AppAction))
            .style(move |_, status| button::Style {
                background: Some(
                    if status == button::Status::Disabled {
                        PANEL
                    } else if status == button::Status::Hovered {
                        CANVAS
                    } else {
                        MINT
                    }
                    .into(),
                ),
                text_color: if status == button::Status::Disabled {
                    SECONDARY
                } else {
                    SLATE
                },
                border: Border {
                    color: if focused { FOCUS } else { EDGE },
                    width: if focused { 2.0 } else { 1.0 },
                    radius: 8.0.into(),
                },
                ..Default::default()
            }),
    )
    .id(id)
    .into()
}
fn result_button(
    name: String,
    caption: String,
    action: super::applications::Action,
    selected: bool,
    focused: bool,
    grid: bool,
) -> Element<'static, Message> {
    button(
        column![
            text(name)
                .size(14)
                .wrapping(iced::widget::text::Wrapping::Glyph),
            note(caption)
        ]
        .spacing(4),
    )
    .on_press(Message::AppAction(action))
    .height(if grid { 98 } else { 54 })
    .width(Length::Fill)
    .padding([6, 12])
    .style(move |_, status| button::Style {
        background: Some(
            if selected {
                MINT
            } else if status == button::Status::Hovered {
                PANEL
            } else {
                CANVAS
            }
            .into(),
        ),
        text_color: SLATE,
        border: Border {
            color: if selected && focused {
                FOCUS
            } else if grid {
                EDGE
            } else {
                Color::TRANSPARENT
            },
            width: if selected && focused { 2.0 } else { 1.0 },
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .into()
}
fn applications(app: &App) -> Element<'_, Message> {
    use super::applications::{Action, Focus};
    use sei_apps::consumer::Status;
    let model = &app.apps;
    let focused = |focus| model.focus == Some(focus);
    let catalog = model.frame.catalog.as_ref();
    let refresh_text = match &model.frame.refresh {
        _ if model.frame.cancel_pending => "Cancelling refresh…".into(),
        _ if model.refreshing() => "Refreshing catalog…".into(),
        Status::Failed { error, .. } => format!(
            "Refresh failed: {} Last known catalog retained; choose Refresh to retry.",
            super::applications::error_text(error)
        ),
        Status::Cancelled(_) => "Refresh cancelled. Last known catalog retained.".into(),
        _ => catalog
            .map(|c| {
                format!(
                    "{} supported applications · {} source issues{} · catalog #{}",
                    c.entries.len(),
                    c.issues,
                    if c.truncated {
                        " · scan limit reached"
                    } else {
                        ""
                    },
                    c.generation
                )
            })
            .unwrap_or_else(|| "Catalog unavailable. Choose Refresh to retry.".into()),
    };
    let search = text_input("Search names and executable filenames", &model.query)
        .id("applications-search")
        .on_input(|query| Message::AppAction(Action::Query(query)))
        .on_submit(Message::AppAction(Action::Submit))
        .padding(11)
        .size(15)
        .style(|_, status| text_input::Style {
            background: Color::WHITE.into(),
            border: Border {
                color: if matches!(status, text_input::Status::Focused { .. }) {
                    FOCUS
                } else {
                    EDGE
                },
                width: 2.0,
                radius: 8.0.into(),
            },
            icon: SECONDARY,
            placeholder: SECONDARY,
            value: SLATE,
            selection: MINT,
        });
    let mut controls = row![
        app_button(
            "Refresh",
            (!model.refreshing()).then_some(Action::Refresh),
            focused(Focus::Refresh)
        ),
        app_button(
            if model.launching() {
                "Launching…"
            } else {
                "Launch selected"
            },
            (model.key().is_some() && !model.launching()).then_some(Action::Launch),
            focused(Focus::Launch)
        )
    ]
    .spacing(10);
    controls = controls.push(app_button(
        if model.pins_pending {
            "Saving pin…"
        } else if model.missing_pin().is_some() {
            "Remove saved pin"
        } else if model.selected().is_some_and(|e| model.is_pinned(e)) {
            "Unpin selected"
        } else {
            "Pin selected"
        },
        model.pin_enabled().then_some(Action::Pin),
        focused(Focus::Pin),
    ));
    let mut controls = column![controls].spacing(8);
    if model.refreshing() {
        controls = controls.push(app_button(
            "Cancel refresh",
            Some(Action::Cancel),
            focused(Focus::Cancel),
        ));
    }
    let browsing = column![
        row![
            app_button(
                if model.pinned_only {
                    "Show: Pinned"
                } else {
                    "Show: All"
                },
                Some(Action::Filter),
                focused(Focus::Filter)
            ),
            app_button(
                if model.presentation == super::applications::Presentation::List {
                    "View: List"
                } else {
                    "View: Grid"
                },
                Some(Action::Presentation),
                focused(Focus::Presentation)
            ),
        ]
        .spacing(10),
        row![
            app_button(
                format!("Sort: {}", model.sort.label()),
                Some(Action::Sort),
                focused(Focus::Sort)
            ),
            app_button(
                format!("Group: {}", model.grouping.label()),
                Some(Action::Group),
                focused(Focus::Group)
            ),
        ]
        .spacing(10),
        note("Activate View, Sort or Group to cycle choices. Up/Down follows reading order."),
    ]
    .spacing(8);
    let mut results = column![].spacing(0);
    if let Some(catalog) = catalog {
        let columns = if model.presentation == super::applications::Presentation::Grid {
            super::applications::Model::grid_columns(app.width)
        } else {
            1
        };
        let mut cursor = 0;
        while cursor < model.filtered.len() {
            let group = model.group(&catalog.entries[model.filtered[cursor]]);
            let mut end = cursor + 1;
            while end < model.filtered.len()
                && model.group(&catalog.entries[model.filtered[end]]) == group
            {
                end += 1;
            }
            if !group.is_empty() {
                results = results.push(container(text(group).size(14)).height(28).padding([4, 12]));
            }
            for chunk in model.filtered[cursor..end].chunks(columns) {
                let mut line = row![].spacing(if columns > 1 { 8 } else { 0 });
                for &index in chunk {
                    let entry = &catalog.entries[index];
                    let selected = model.selected.as_ref() == Some(&entry.id)
                        && model.missing_selected.is_none();
                    let limit = if columns > 1 { 36 } else { 58 };
                    let name: String = entry.name.chars().take(limit).collect();
                    let name = if entry.name.chars().count() > limit {
                        format!("{name}…")
                    } else {
                        name
                    };
                    let caption = format!(
                        "{}{}",
                        if model.is_pinned(entry) {
                            "Pinned · "
                        } else {
                            ""
                        },
                        entry.source
                    );
                    line = line.push(result_button(
                        name,
                        caption,
                        Action::Select(entry.id.clone()),
                        selected,
                        focused(Focus::Results),
                        columns > 1,
                    ));
                }
                for _ in chunk.len()..columns {
                    line = line.push(Space::new().width(Length::Fill));
                }
                results = results.push(line);
                if columns > 1 {
                    results = results.push(Space::new().height(8));
                }
            }
            cursor = end;
        }
    }
    if !model.missing.is_empty() {
        results = results.push(
            container(
                text(if catalog.is_some() {
                    "Unavailable pins"
                } else {
                    "Saved pins · availability not checked"
                })
                .size(14),
            )
            .height(28)
            .padding([4, 12]),
        );
        for &index in &model.missing {
            let pin = &model.pins.pins[index];
            let name: String = pin.name.chars().take(58).collect();
            results = results.push(result_button(
                name,
                "Saved pin · launch unavailable".into(),
                Action::Missing(pin.token.clone()),
                model.missing_selected.as_ref() == Some(&pin.token),
                focused(Focus::Results),
                false,
            ));
        }
    }
    let results: Element<'_, Message> = if model.filtered.is_empty() && model.missing.is_empty() {
        container(note(if model.refreshing() && catalog.is_none() {
            "Loading applications…"
        } else if catalog.is_none() {
            "Catalog unavailable."
        } else if model.pinned_only && model.pins.pins.is_empty() {
            "No pinned applications yet. Show All, select an application, then choose Pin selected."
        } else if model.pinned_only {
            "No saved pins match this search. Clear the query to browse your pins."
        } else if catalog.is_some_and(|c| c.entries.is_empty()) {
            "The catalog contains no supported applications."
        } else {
            "No applications match this search. Clear the query to browse all."
        }))
        .padding(16)
        .height(160)
        .width(Length::Fill)
        .into()
    } else {
        scrollable(results)
            .id("applications-results")
            .height(220)
            .width(Length::Fill)
            .spacing(8)
            .into()
    };
    let results = container(results)
        .id("applications-results-panel")
        .padding(4)
        .width(Length::Fill)
        .style(move |_| {
            let mut style = surface(CANVAS);
            if focused(Focus::Results) {
                style.border.color = FOCUS;
                style.border.width = 2.0;
            }
            style
        });
    let details: Element<'_, Message> = if let Some(entry) = model.selected() {
        container(
            column![
                text(&entry.name).size(18),
                note(format!("Source: {}", entry.source)),
                text(format!("Executable: {}", entry.executable))
                    .size(13)
                    .wrapping(iced::widget::text::Wrapping::Glyph),
                text(format!("Shortcut: {}", entry.shortcut))
                    .size(13)
                    .wrapping(iced::widget::text::Wrapping::Glyph),
                text(format!(
                    "Working directory: {}",
                    entry
                        .directory
                        .as_deref()
                        .unwrap_or("Executable directory (service default)")
                ))
                .size(13)
                .wrapping(iced::widget::text::Wrapping::Glyph),
            ]
            .spacing(6),
        )
        .padding(16)
        .width(Length::Fill)
        .style(|_| surface(PANEL))
        .into()
    } else if let Some(pin) = model.missing_pin() {
        container(column![text(&pin.name).size(18),note("This saved identity is unavailable in the current catalog. It cannot be launched. Choose Remove saved pin to forget it; Refresh can check availability again.")].spacing(8))
            .padding(16).width(Length::Fill).style(|_|surface(PANEL)).into()
    } else {
        note("No application selected.").into()
    };
    let target = model.frame.launch_name.as_deref().unwrap_or("application");
    let launch_text = match &model.frame.launch {
        _ if model.launching() && super::applications::busy(&model.frame.launch) => {
            format!("Launching {target}…")
        }
        _ if model.launching() => "Launch requested…".into(),
        Status::Succeeded { .. } => {
            format!("Last launch: Process accepted for {target}. Window opening is not verified.")
        }
        Status::Failed { error, .. } => format!(
            "Could not launch {target}: {}",
            super::applications::error_text(error)
        ),
        Status::Cancelled(_) => format!("Launch cancelled: {target}"),
        _ => "Select a result, then choose Launch selected. Enter launches the selection.".into(),
    };
    let mut feedback = column![
        container(text(launch_text).size(14))
            .padding(12)
            .width(Length::Fill)
            .style(|_| surface(PANEL))
    ]
    .spacing(8);
    if let Some(error) = model.local_error.as_ref().cloned().or_else(|| {
        model
            .frame
            .error
            .as_ref()
            .map(super::applications::error_text)
    }) {
        feedback = feedback.push(text(error).size(14));
    }
    if model.frame.stale {
        feedback = feedback.push(note(
            "An outdated catalog request was ignored. Select from the current results.",
        ));
    }
    if let Some(error) = &model.pins.error {
        feedback = feedback.push(text(error).size(14));
    }
    feedback = feedback.push(note(if model.pins_pending {
        "Saving favorites… Changes appear after the save succeeds."
    } else if model.pins.revision == 0 {
        "Loading favorites…"
    } else if model.pins.writable {
        "Favorites loaded · pins are saved on this computer."
    } else {
        "Favorites are read-only. Resolve the configuration message above and restart."
    }));
    let content = column![
        text("Applications").size(30),
        note("Supported Start menu applications"),
        column![text("Search applications").size(14), search].spacing(6),
        browsing,
        controls,
        feedback,
        note(refresh_text),
        text(format!(
            "{} available results · {} unavailable pins · Up/Down selects",
            model.filtered.len(),
            model.missing.len()
        ))
        .size(13),
        results,
        details,
    ]
    .spacing(12);
    content.push(note("Keyboard focus is local. Screen-reader support remains under validation. Launched applications continue when Sei-Desk closes.")).into()
}
pub fn content(app: &App) -> Element<'_, Message> {
    let rail = container(
        column![
            text("Sei-Desk").size(23),
            note("Desktop overview"),
            Space::new().height(18),
            nav(app, Page::Overview, "Overview"),
            nav(app, Page::Applications, "Applications"),
            Space::new().height(Length::Fill),
            note("Tab to navigate"),
            note("Enter to open"),
            note("PgUp / PgDn to scroll"),
        ]
        .spacing(8),
    )
    .padding(18)
    .width(184)
    .height(Length::Fill)
    .style(|_| container::Style {
        background: Some(PANEL.into()),
        ..Default::default()
    });
    let page = match app.page {
        Page::Overview => overview(app),
        Page::Applications => applications(app),
    };
    let content = column![
        row![
            text("System resources").size(14).color(SECONDARY),
            Space::new().width(Length::Fill),
            note(app.clock.clone())
        ]
        .spacing(12),
        scrollable(container(page).width(Length::Fill).padding(iced::Padding {
            top: 20.0,
            right: 0.0,
            bottom: 16.0,
            left: 0.0
        }))
        .id("dashboard-content")
        .width(Length::Fill)
        .spacing(10)
        .height(Length::Fill),
    ]
    .spacing(8)
    .padding(24)
    .width(Length::Fill);
    container(row![rail, content])
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}
