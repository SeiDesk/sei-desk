use super::*;
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
pub(crate) struct Temp(pub PathBuf);
impl Temp {
    pub(crate) fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "sei-apps-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}
struct Fake {
    exe: PathBuf,
    calls: usize,
}
impl LinkReader for Fake {
    fn read(&mut self, path: &Path) -> Result<Shortcut, Error> {
        self.calls += 1;
        match path.file_stem().unwrap().to_str().unwrap() {
            "bad" => Err(Error::new(ErrorKind::Invalid, "fixture malformed")),
            "denied" => Err(Error::new(ErrorKind::AccessDenied, "fixture denied")),
            "stale" => Ok(Shortcut {
                executable: self.exe.with_file_name("missing.exe"),
                arguments: String::new(),
                working_directory: None,
                icon: None,
            }),
            name => Ok(Shortcut {
                executable: self.exe.clone(),
                arguments: if name == "different" {
                    " /mode".into()
                } else {
                    String::new()
                },
                working_directory: None,
                icon: None,
            }),
        }
    }
}
fn fixture(names: &[&str]) -> (Temp, Fake, Vec<Root>) {
    let temp = Temp::new();
    let exe = temp.0.join("fixture.exe");
    std::fs::write(&exe, b"not executable, never launched").unwrap();
    for name in names {
        std::fs::write(temp.0.join(format!("{name}.lnk")), b"mock").unwrap();
    }
    let roots = vec![Root {
        source: Source::UserStartMenu,
        path: temp.0.clone(),
    }];
    (temp, Fake { exe, calls: 0 }, roots)
}
#[test]
fn normalized_identity_dedup_and_order_are_repeatable() {
    let (_temp, mut reader, roots) = fixture(&[" Z  App ", "A", "different"]);
    let a = discover(
        &roots,
        &mut reader,
        Limits::default(),
        &Cancellation::default(),
        None,
    )
    .unwrap();
    let b = discover(
        &roots,
        &mut reader,
        Limits::default(),
        &Cancellation::default(),
        None,
    )
    .unwrap();
    assert_eq!(a.entries, b.entries);
    assert_eq!(a.entries.len(), 2);
    assert_eq!(a.entries[0].display_name, "different");
    assert_eq!(a.entries[1].display_name, "Z App"); // normalized representative
    assert_ne!(a.entries[0].id, a.entries[1].id);
}
#[test]
fn user_source_wins_duplicate_independent_of_input_order() {
    let (_temp, mut reader, mut roots) = fixture(&["App"]);
    roots.insert(
        0,
        Root {
            source: Source::CommonStartMenu,
            path: roots[0].path.clone(),
        },
    );
    let result = discover(
        &roots,
        &mut reader,
        Limits::default(),
        &Cancellation::default(),
        None,
    )
    .unwrap();
    assert_eq!(result.entries.len(), 1);
    assert_eq!(result.entries[0].source, Source::UserStartMenu);
}
#[test]
fn malformed_stale_and_denied_entries_do_not_abort_refresh() {
    let (_temp, mut reader, roots) = fixture(&["bad", "denied", "stale", "valid"]);
    let result = discover(
        &roots,
        &mut reader,
        Limits::default(),
        &Cancellation::default(),
        None,
    )
    .unwrap();
    assert_eq!(result.entries.len(), 1);
    for kind in [
        ErrorKind::Invalid,
        ErrorKind::AccessDenied,
        ErrorKind::Missing,
    ] {
        assert!(result.issues.iter().any(|i| i.error.kind == kind));
    }
}
#[test]
fn missing_and_inaccessible_sources_are_explicit() {
    let (_temp, mut reader, roots) = fixture(&[]);
    let missing = Root {
        path: roots[0].path.join("missing"),
        source: Source::UserStartMenu,
    };
    let result = discover(
        &[missing],
        &mut reader,
        Limits::default(),
        &Cancellation::default(),
        None,
    )
    .unwrap();
    assert_eq!(result.issues[0].error.kind, ErrorKind::Missing);
    // Access-denied mapping is injected; no real ACL modification in tests.
    assert_eq!(
        Error::io("read source", std::io::Error::from_raw_os_error(5)).kind,
        ErrorKind::AccessDenied
    );
}
#[test]
fn overflowing_directory_is_not_an_arbitrary_prefix() {
    let (_temp, mut reader, roots) = fixture(&["one", "two"]);
    let result = discover(
        &roots,
        &mut reader,
        Limits {
            max_nodes: 1,
            ..Limits::default()
        },
        &Cancellation::default(),
        None,
    )
    .unwrap();
    assert!(result.truncated);
    assert!(result.entries.is_empty());
    assert_eq!(reader.calls, 0);
}
#[test]
fn depth_and_file_size_limits_are_enforced() {
    let (temp, mut reader, roots) = fixture(&["one"]);
    std::fs::create_dir(temp.0.join("nested")).unwrap();
    let result = discover(
        &roots,
        &mut reader,
        Limits {
            max_depth: 0,
            max_link_bytes: 1,
            ..Limits::default()
        },
        &Cancellation::default(),
        None,
    )
    .unwrap();
    assert!(result.entries.is_empty());
    assert_eq!(reader.calls, 0);
    assert_eq!(result.issues.len(), 2);
}
#[test]
fn cancelled_refresh_does_not_read_or_launch() {
    let (_temp, mut reader, roots) = fixture(&["one"]);
    let cancel = Cancellation::default();
    cancel.cancel();
    assert_eq!(
        discover(&roots, &mut reader, Limits::default(), &cancel, None)
            .unwrap_err()
            .kind,
        ErrorKind::Cancelled
    );
    assert_eq!(reader.calls, 0);
    let request = LaunchRequest::executable(reader.exe, vec![], None).unwrap();
    assert_eq!(
        launch(request, &cancel).unwrap_err().kind,
        ErrorKind::Cancelled
    );
}
#[test]
fn missing_target_and_invalid_request_are_structured_errors() {
    let temp = Temp::new();
    let request = LaunchRequest::executable(temp.0.join("missing.exe"), vec![], None).unwrap();
    assert_eq!(request.validate().unwrap_err().kind, ErrorKind::Missing);
    assert!(LaunchRequest::executable("relative.exe".into(), vec![], None).is_err());
    assert!(LaunchRequest::executable(temp.0.join("script.cmd"), vec![], None).is_err());
    assert!(
        LaunchRequest::executable(temp.0.join("app.exe"), vec!["bad\0arg".into()], None).is_err()
    );
}
#[test]
fn launch_error_mapping_preserves_code() {
    for (code, kind) in [
        (2, ErrorKind::Missing),
        (3, ErrorKind::Missing),
        (5, ErrorKind::AccessDenied),
        (740, ErrorKind::AccessDenied),
        (193, ErrorKind::Invalid),
        (216, ErrorKind::Invalid),
        (8, ErrorKind::ProcessCreation),
    ] {
        let error = Error::io("CreateProcess", std::io::Error::from_raw_os_error(code));
        assert_eq!(error.kind, kind);
        assert_eq!(error.code, Some(code));
    }
}
#[test]
fn progress_backpressure_never_blocks_refresh() {
    let (_temp, mut reader, roots) = fixture(&["App"]);
    let (tx, _rx) = std::sync::mpsc::sync_channel(0);
    assert_eq!(
        discover(
            &roots,
            &mut reader,
            Limits::default(),
            &Cancellation::default(),
            Some(&tx)
        )
        .unwrap()
        .entries
        .len(),
        1
    );
}

#[test]
fn identity_normalizes_path_case_and_preserves_argument_case() {
    assert_eq!(
        crate::catalog::path_key(Path::new("C:/Tools/./APP.exe")).unwrap(),
        "c:\\tools\\app.exe"
    );
    let (_temp, reader, _roots) = fixture(&[]);
    let a = LaunchRequest::executable(
        reader.exe.clone(),
        vec!["MODE".into(), "two words".into()],
        None,
    )
    .unwrap();
    let b = LaunchRequest::executable(reader.exe, vec!["mode".into(), "two words".into()], None)
        .unwrap();
    assert_ne!(a.arguments, b.arguments);
    let link = |arguments: &str, cwd| Shortcut {
        executable: a.executable.clone(),
        arguments: arguments.into(),
        working_directory: cwd,
        icon: None,
    };
    let first = crate::catalog::normalize(
        Source::UserStartMenu,
        PathBuf::from("First.lnk"),
        link("MODE", None),
    )
    .unwrap();
    let second = crate::catalog::normalize(
        Source::CommonStartMenu,
        PathBuf::from("Second.lnk"),
        link("MODE", a.executable.parent().map(Path::to_path_buf)),
    )
    .unwrap();
    assert_eq!(first.id, second.id);
    let third = crate::catalog::normalize(
        Source::UserStartMenu,
        PathBuf::from("Third.lnk"),
        link("mode", None),
    )
    .unwrap();
    assert_ne!(first.id, third.id);
}
#[test]
fn refresh_cancellation_during_reader_discards_partial_catalog() {
    struct Cancelling(Fake, Cancellation);
    impl LinkReader for Cancelling {
        fn read(&mut self, path: &Path) -> Result<Shortcut, Error> {
            self.1.cancel();
            self.0.read(path)
        }
    }
    let (_temp, reader, roots) = fixture(&["App"]);
    let cancel = Cancellation::default();
    let mut reader = Cancelling(reader, cancel.clone());
    assert_eq!(
        discover(&roots, &mut reader, Limits::default(), &cancel, None)
            .unwrap_err()
            .kind,
        ErrorKind::Cancelled
    );
}
#[test]
fn unsupported_shell_uri_packaged_and_network_targets_are_rejected() {
    let temp = Temp::new();
    for name in ["cmd.exe", "powershell.exe", "explorer.exe", "script.cmd"] {
        assert_eq!(
            LaunchRequest::executable(temp.0.join(name), vec![], None)
                .unwrap_err()
                .kind,
            ErrorKind::Unsupported
        );
    }
    for path in [
        "https://example.invalid",
        "shell:AppsFolder\\Test!App",
        "\\\\server\\share\\app.exe",
    ] {
        assert!(LaunchRequest::executable(path.into(), vec![], None).is_err());
    }
}
