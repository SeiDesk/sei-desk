use super::*;
fn entry(id: &str, name: &str, filename: &str) -> Entry {
    Entry {
        id: id.into(),
        token: super::identity::token(id),
        name: name.into(),
        executable: format!("C:\\Private\\{filename}"),
        shortcut: "C:\\Private\\ignored.lnk".into(),
        source: "User Start menu",
        directory: None,
        search: format!("{name}\n{filename}").to_lowercase(),
    }
}
fn frame(revision: u64, generation: u64, entries: Vec<Entry>) -> Arc<Frame> {
    Arc::new(Frame {
        revision,
        catalog: Some(Arc::new(Catalog {
            generation,
            entries,
            issues: 0,
            truncated: false,
        })),
        ..Default::default()
    })
}
pub(crate) fn loaded() -> Model {
    let mut model = Model::default();
    model.apply_frame(frame(
        1,
        1,
        vec![
            entry("a", "Alpha Editor", "alpha.exe"),
            entry("b", "Béta Notes", "memo.exe"),
        ],
    ));
    model
}
#[test]
fn empty_and_case_insensitive_queries_use_only_names_and_executable_filenames() {
    let mut model = loaded();
    assert_eq!(model.filtered.len(), 2);
    for query in ["ALPHA", " editor ", "alpha.EXE"] {
        model.edit(query.into());
        assert_eq!(model.selected.as_deref(), Some("a"));
        assert_eq!(model.filtered.len(), 1);
    }
    model.edit("BÉTA".into());
    assert_eq!(model.selected.as_deref(), Some("b"));
    model.edit("memo.exe".into());
    assert_eq!(model.selected.as_deref(), Some("b"));
    for unsupported in ["Private", "User Start menu", "ignored.lnk"] {
        model.edit(unsupported.into());
        assert!(model.filtered.is_empty());
    }
    model.edit("  ".into());
    assert_eq!(model.filtered.len(), 2);
}
#[test]
fn query_changes_reconcile_identity_not_indices_and_empty_states_are_distinct() {
    let mut model = loaded();
    model.move_selection(1);
    assert_eq!(model.key().unwrap().id, "b");
    model.edit("Alpha".into());
    assert_eq!(model.key().unwrap().id, "a");
    model.edit("not present".into());
    assert!(model.key().is_none());
    assert!(!model.frame.catalog.as_ref().unwrap().entries.is_empty());
    model.apply_frame(frame(2, 2, vec![]));
    assert!(model.frame.catalog.as_ref().unwrap().entries.is_empty());
    assert!(model.filtered.is_empty());
    assert!(model.selected().is_none());
}
#[test]
fn reordered_refresh_keeps_stable_identity_and_removal_selects_first_valid_entry() {
    let mut model = loaded();
    model.move_selection(1);
    model.apply_frame(frame(
        2,
        10,
        vec![entry("b", "Béta", "b.exe"), entry("a", "Alpha", "a.exe")],
    ));
    assert_eq!(
        model.key(),
        Some(Key {
            generation: 10,
            id: "b".into()
        })
    );
    model.apply_frame(frame(3, 11, vec![entry("a", "Alpha", "a.exe")]));
    assert_eq!(model.key().unwrap().id, "a");
    model.apply_frame(frame(4, 12, vec![]));
    assert!(model.key().is_none());
}
#[test]
fn stale_publication_never_replaces_a_new_catalog_or_selection() {
    let mut model = loaded();
    model.apply_frame(frame(5, 9, vec![entry("b", "Béta", "b.exe")]));
    model.apply_frame(frame(2, 2, vec![entry("a", "Old", "old.exe")]));
    assert_eq!(model.frame.revision, 5);
    assert_eq!(model.key().unwrap().generation, 9);
    assert_eq!(model.key().unwrap().id, "b");
}
#[test]
fn keyboard_boundaries_empty_results_and_unicode_query_bounds() {
    let mut model = loaded();
    model.move_selection(-1);
    assert_eq!(model.position(), 0);
    model.move_selection(100);
    assert_eq!(model.position(), 1);
    model.selected = None;
    model.move_selection(1);
    assert_eq!(model.selected.as_deref(), Some("a"));
    model.move_selection(1);
    assert_eq!(model.position(), 1);
    model.edit("🙂".repeat(400));
    assert_eq!(model.query.chars().count(), 256);
    model.move_selection(-1);
    assert!(model.key().is_none());
}

#[test]
fn user_feedback_preserves_native_error_code_without_internal_debug_text() {
    let error = sei_apps::consumer::ConsumerError::Service(sei_apps::Error {
        kind: sei_apps::ErrorKind::Missing,
        operation: "target metadata",
        code: Some(2),
    });
    let text = error_text(&error);
    assert!(text.contains("missing"));
    assert!(text.contains("Windows error 2"));
    assert!(!text.contains("Service(Error"));
    let elevation = sei_apps::consumer::ConsumerError::Service(sei_apps::Error {
        kind: sei_apps::ErrorKind::AccessDenied,
        operation: "CreateProcess",
        code: Some(740),
    });
    let text = error_text(&elevation);
    assert!(text.contains("Administrator permission is required"));
    assert!(text.contains("through Windows"));
    assert!(text.contains("Windows error 740"));
}
#[test]
fn launch_is_guarded_until_acknowledged_and_terminal_service_status() {
    let mut model = loaded();
    model.mark_launch(7);
    assert!(model.launching());
    let mut consumer = sei_apps::consumer::Consumer::new();
    let id = consumer.request_refresh().unwrap();
    model.apply_frame(Arc::new(Frame {
        revision: 2,
        ack: 7,
        catalog: model.frame.catalog.clone(),
        launch: Status::Running {
            id,
            cancellation_requested: false,
        },
        ..Default::default()
    }));
    assert!(model.launching());
    model.apply_frame(Arc::new(Frame {
        revision: 3,
        ack: 7,
        catalog: model.frame.catalog.clone(),
        launch: Status::Succeeded {
            id,
            value: Accepted { pid: 42 },
        },
        ..Default::default()
    }));
    assert!(!model.launching());
}
