use super::*;
use crate::applications::{Catalog, Frame};
fn entry(id: &str, name: &str, exe: &str, source: &'static str) -> Entry {
    Entry {
        id: id.into(),
        name: name.into(),
        executable: exe.into(),
        shortcut: String::new(),
        source,
        directory: None,
        token: crate::applications::identity::token(id),
        search: format!("{name}\n{exe}").to_lowercase(),
    }
}
fn catalog(revision: u64, entries: Vec<Entry>) -> Arc<Frame> {
    Arc::new(Frame {
        revision,
        catalog: Some(Arc::new(Catalog {
            generation: revision,
            entries,
            issues: 0,
            truncated: false,
        })),
        ..Default::default()
    })
}
fn loaded() -> Model {
    let mut model = Model::default();
    model.apply_frame(catalog(
        1,
        vec![
            entry("b", "Beta", "z.exe", "Shared Start menu"),
            entry("a", "Alpha", "", "User Start menu"),
            entry("c", "Gamma", "a.exe", ""),
        ],
    ));
    model.apply_pins(Arc::new(super::super::preferences::worker::State {
        revision: 1,
        writable: true,
        ..Default::default()
    }));
    model
}
fn names(model: &Model) -> Vec<&str> {
    model
        .filtered
        .iter()
        .map(|&i| {
            model.frame.catalog.as_ref().unwrap().entries[i]
                .name
                .as_str()
        })
        .collect()
}
fn save(model: &mut Model, pins: Vec<Pin>) {
    model.apply_pins(Arc::new(super::super::preferences::worker::State {
        revision: model.pins.revision + 1,
        pins: Arc::new(pins),
        writable: true,
        error: None,
    }));
}
#[test]
fn sorting_is_deterministic_with_missing_metadata_and_keeps_identity() {
    let mut model = loaded();
    assert_eq!(names(&model), ["Alpha", "Beta", "Gamma"]);
    model.move_selection(1);
    let key = model.key().unwrap();
    model.sort = Sort::Reverse;
    model.filter();
    assert_eq!(names(&model), ["Gamma", "Beta", "Alpha"]);
    assert_eq!(model.key(), Some(key.clone()));
    model.sort = Sort::Executable;
    model.filter();
    assert_eq!(names(&model), ["Alpha", "Gamma", "Beta"]);
    assert_eq!(model.key(), Some(key));
}
#[test]
fn pin_unpin_reorder_and_presentation_preserve_live_identity() {
    let mut model = loaded();
    model.move_selection(1);
    let pins = model.pin_change().unwrap();
    save(&mut model, pins);
    let key = model.key().unwrap();
    assert_eq!(key.id, "b");
    model.sort = Sort::Pinned;
    model.filter();
    assert_eq!(names(&model)[0], "Beta");
    model.presentation = Presentation::Grid;
    model.filter();
    assert_eq!(model.key(), Some(key));
    model.apply_frame(catalog(
        2,
        vec![
            entry("c", "Gamma", "a.exe", ""),
            entry("b", "Beta", "z.exe", "Shared Start menu"),
            entry("a", "Alpha", "", "User Start menu"),
        ],
    ));
    assert_eq!(model.key().unwrap().id, "b");
    let pins = model.pin_change().unwrap();
    save(&mut model, pins);
    assert!(model.pins.pins.is_empty());
    assert_eq!(model.key().unwrap().id, "b");
}
#[test]
fn missing_pin_cannot_launch_and_can_be_removed_without_retargeting() {
    let mut model = loaded();
    save(
        &mut model,
        vec![Pin {
            token: crate::applications::identity::token("gone"),
            name: "Removed".into(),
        }],
    );
    model.pinned_only = true;
    model.filter();
    assert!(model.filtered.is_empty());
    assert_eq!(model.missing.len(), 1);
    assert!(model.key().is_none());
    assert!(model.missing_pin().is_some());
    let pins = model.pin_change().unwrap();
    save(&mut model, pins);
    assert!(model.missing.is_empty());
    assert!(model.key().is_none());
}
#[test]
fn search_pins_sort_and_source_grouping_compose() {
    let mut model = loaded();
    save(
        &mut model,
        vec![
            Pin {
                token: crate::applications::identity::token("b"),
                name: "Beta".into(),
            },
            Pin {
                token: crate::applications::identity::token("c"),
                name: "Gamma".into(),
            },
        ],
    );
    model.grouping = Grouping::Source;
    model.sort = Sort::Reverse;
    model.pinned_only = true;
    model.edit("a".into());
    assert_eq!(names(&model), ["Beta", "Gamma"]);
    assert_eq!(
        model.group(&model.frame.catalog.as_ref().unwrap().entries[model.filtered[1]]),
        "Source unavailable"
    );
    model.grouping = Grouping::Pinned;
    model.pinned_only = false;
    model.filter();
    assert_eq!(names(&model), ["Gamma", "Beta", "Alpha"]);
    assert_eq!(model.key().unwrap().id, "b");
    model.edit("no matches".into());
    assert!(model.filtered.is_empty());
    assert!(model.frame.catalog.as_ref().unwrap().entries.len() == 3);
}
#[test]
fn both_presentations_clamp_navigation_and_empty_pins_are_distinct() {
    for presentation in [Presentation::List, Presentation::Grid] {
        let mut model = loaded();
        model.presentation = presentation;
        model.move_selection(-10);
        assert_eq!(model.position(), 0);
        model.move_selection(100);
        assert_eq!(model.position(), 2);
        model.pinned_only = true;
        model.filter();
        model.move_selection(1);
        assert!(model.key().is_none());
        assert!(model.missing.is_empty());
        assert!(!model.frame.catalog.as_ref().unwrap().entries.is_empty());
        model.apply_frame(catalog(2, vec![]));
        assert!(model.frame.catalog.as_ref().unwrap().entries.is_empty());
    }
}
#[test]
fn pending_failed_and_stale_pin_frames_cannot_clobber_new_preferences() {
    let mut model = loaded();
    let pins = model.pin_change().unwrap();
    model.pins_pending = true;
    assert!(model.pin_change().is_err());
    model.sort = Sort::Reverse;
    model.grouping = Grouping::Source;
    save(&mut model, pins);
    assert!(!model.pins_pending);
    let saved = model.pins.clone();
    model.apply_pins(Arc::new(super::super::preferences::worker::State {
        revision: 1,
        ..Default::default()
    }));
    assert_eq!(model.pins.pins, saved.pins);
    assert_eq!(model.sort, Sort::Reverse);
    assert_eq!(model.grouping, Grouping::Source);
    model.apply_frame(catalog(2, vec![entry("a", "Alpha", "", "User Start menu")]));
    assert_eq!(model.pins.pins, saved.pins);
}
#[test]
fn refresh_disappearance_and_return_keep_the_same_pin_identity() {
    let mut model = loaded();
    model.move_selection(1);
    let pins = model.pin_change().unwrap();
    save(&mut model, pins);
    model.pinned_only = true;
    model.filter();
    model.apply_frame(catalog(2, vec![entry("a", "Alpha", "", "User Start menu")]));
    assert!(model.key().is_none());
    assert_eq!(model.missing_pin().unwrap().name, "Beta");
    model.apply_frame(catalog(
        3,
        vec![
            entry("b", "Renamed Beta", "z.exe", "Shared Start menu"),
            entry("a", "Alpha", "", "User Start menu"),
        ],
    ));
    assert_eq!(model.key().unwrap().id, "b");
    assert!(model.missing_selected.is_none());
}
#[test]
fn save_receipt_reconciles_scroll_target_after_reorder_and_pinned_removal() {
    for presentation in [Presentation::List, Presentation::Grid] {
        let mut model = loaded();
        model.presentation = presentation;
        let mut entries: Vec<_> = (0..24)
            .map(|i| {
                entry(
                    &format!("id-{i:02}"),
                    &format!("Utility {i:02}"),
                    "utility.exe",
                    "User Start menu",
                )
            })
            .collect();
        entries.push(entry("z", "Zebra", "z.exe", "User Start menu"));
        model.apply_frame(catalog(2, entries));
        model.move_selection(100);
        assert_eq!(model.key().unwrap().id, "z");
        model.grouping = Grouping::Pinned;
        model.filter();
        let pins = model.pin_change().unwrap();
        save(&mut model, pins);
        assert_eq!(model.offset(1100.0), 0.0);
        let pins = model.pin_change().unwrap();
        save(&mut model, pins);
        assert!(model.offset(1100.0) > 220.0);
        assert_eq!(model.key().unwrap().id, "z");
        let pins = model.pin_change().unwrap();
        save(&mut model, pins);
        model.pinned_only = true;
        model.filter();
        let pins = model.pin_change().unwrap();
        save(&mut model, pins);
        assert!(model.key().is_none());
        assert_eq!(model.offset(1100.0), 0.0);
    }
}
