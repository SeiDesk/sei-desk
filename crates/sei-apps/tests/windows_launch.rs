#![cfg(windows)]
use sei_apps::{Cancellation, ErrorKind, LaunchRequest, launch_async};
use std::{
    ffi::OsString,
    path::PathBuf,
    time::{Duration, Instant},
};

/// Explicit opt-in: only the freshly built launch_fixture.exe is started.
/// Never enumerates or launches installed/discovered applications.
#[test]
#[ignore = "requires SEI_APPS_FIXTURE pointing to this workspace's built fixture"]
fn real_createprocess_structured_arguments_and_cancellation() {
    let fixture =
        PathBuf::from(std::env::var_os("SEI_APPS_FIXTURE").expect("fixture path required"));
    assert_eq!(fixture.file_name().unwrap(), "launch_fixture.exe");
    let temp = std::env::temp_dir().join(format!("sei-apps-launch-{}", std::process::id()));
    std::fs::create_dir(&temp).unwrap();
    struct Cleanup(PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    let _cleanup = Cleanup(temp.clone());
    let output = temp.join("received.txt");
    let values: Vec<OsString> = [
        "",
        "two words",
        "a\"b",
        "tail\\",
        "\\\"",
        "&|%PATH%$()",
        "日本語",
    ]
    .into_iter()
    .map(Into::into)
    .collect();
    let mut args = vec![output.clone().into_os_string()];
    args.extend(values.clone());
    let request = LaunchRequest::executable(fixture.clone(), args, Some(temp.clone())).unwrap();
    let cancel = Cancellation::default();
    let mut receipt = launch_async(request.clone(), cancel.clone())
        .unwrap()
        .join()
        .unwrap()
        .unwrap();
    // Cancellation after spawn cannot revoke acceptance or kill the child.
    cancel.cancel();
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(status) = receipt.child.try_wait().unwrap() {
            assert!(status.success());
            break;
        }
        if Instant::now() >= deadline {
            let _ = receipt.child.kill();
            let _ = receipt.child.wait();
            panic!("owned fixture timeout");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(
        std::fs::read_to_string(&output).unwrap(),
        format!("{values:?}\n{temp:?}")
    );
    std::fs::remove_file(&output).unwrap();
    assert_eq!(
        launch_async(request, cancel)
            .unwrap()
            .join()
            .unwrap()
            .unwrap_err()
            .kind,
        ErrorKind::Cancelled
    );
    assert!(!output.exists());

    // Existing but invalid PE reaches actual CreateProcess error 193/216, rather
    // than a mocked mapping or merely the metadata preflight.
    let invalid = temp.join("invalid.exe");
    std::fs::write(&invalid, b"not a PE executable").unwrap();
    let error = launch_async(
        LaunchRequest::executable(invalid, vec![], None).unwrap(),
        Cancellation::default(),
    )
    .unwrap()
    .join()
    .unwrap()
    .unwrap_err();
    assert_eq!(error.kind, ErrorKind::Invalid);
    assert!(matches!(error.code, Some(193 | 216)));
}

#[test]
#[ignore = "requires this workspace's built SEI_APPS_FIXTURE; test-owned copy only"]
fn acceptance_stale_locked_target_and_default_working_directory() {
    use std::os::windows::fs::OpenOptionsExt;
    let fixture = PathBuf::from(std::env::var_os("SEI_APPS_FIXTURE").expect("fixture path"));
    assert_eq!(fixture.file_name().unwrap(), "launch_fixture.exe");
    let temp =
        std::env::temp_dir().join(format!("sei-apps-acceptance-launch-{}", std::process::id()));
    std::fs::create_dir(&temp).unwrap();
    struct Cleanup(PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    let _cleanup = Cleanup(temp.clone());
    let owned = temp.join("Owned.exe");
    std::fs::copy(fixture, &owned).unwrap();
    let output = temp.join("received.txt");
    let request = LaunchRequest::executable(
        owned.clone(),
        vec![output.clone().into_os_string(), "literal &|%PATH%".into()],
        None,
    )
    .unwrap();
    let mut receipt = launch_async(request.clone(), Cancellation::default())
        .unwrap()
        .join()
        .unwrap()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(status) = receipt.child.try_wait().unwrap() {
            assert!(status.success());
            break;
        }
        if Instant::now() >= deadline {
            let _ = receipt.child.kill();
            let _ = receipt.child.wait();
            panic!("owned fixture timeout");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let expected: Vec<OsString> = vec!["literal &|%PATH%".into()];
    assert_eq!(
        std::fs::read_to_string(&output).unwrap(),
        format!("{expected:?}\n{temp:?}")
    );
    drop(receipt);
    std::fs::remove_file(&output).unwrap();

    let lock = std::fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(&owned)
        .unwrap();
    let error = match launch_async(request.clone(), Cancellation::default())
        .unwrap()
        .join()
        .unwrap()
    {
        Err(error) => error,
        Ok(mut unexpected) => {
            let _ = unexpected.child.kill();
            let _ = unexpected.child.wait();
            panic!("exclusively locked fixture was unexpectedly launched");
        }
    };
    println!(
        "acceptance locked target: {:?} {:?} {:?}",
        error.kind, error.operation, error.code
    );
    assert!(matches!(
        error.kind,
        ErrorKind::AccessDenied | ErrorKind::ProcessCreation
    ));
    assert!(error.code.is_some());
    assert!(!output.exists());
    drop(lock);
    request.validate().unwrap();
    std::fs::remove_file(owned).unwrap();
    let error = launch_async(request, Cancellation::default())
        .unwrap()
        .join()
        .unwrap()
        .unwrap_err();
    assert_eq!(error.kind, ErrorKind::Missing);
    println!(
        "acceptance stale target: {:?} {:?} {:?}",
        error.kind, error.operation, error.code
    );
    assert!(!output.exists());
}
