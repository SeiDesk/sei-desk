#![cfg(windows)]
use sei_apps::consumer::{Accepted, Consumer, ConsumerError, Service, Status};
use sei_apps::{Cancellation, Catalog, Entry, Error, ErrorKind, LaunchRequest, Source};
use std::{
    path::PathBuf,
    process::Child,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

// Discovery is an explicit fixture snapshot; launch uses the real Stage 3A
// blocking service. Child ownership retained here solely for exact test cleanup.
struct FixtureService {
    entry: Entry,
    child: Mutex<Option<Child>>,
}
impl Drop for FixtureService {
    fn drop(&mut self) {
        if let Ok(slot) = self.child.get_mut()
            && let Some(mut child) = slot.take()
        {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}
impl Service for FixtureService {
    fn refresh(&self, _: &Cancellation) -> Result<Catalog, Error> {
        Ok(Catalog {
            entries: vec![self.entry.clone()],
            ..Catalog::default()
        })
    }
    fn launch(&self, request: LaunchRequest, cancel: &Cancellation) -> Result<Accepted, Error> {
        let receipt = sei_apps::launch(request, cancel)?; // no mutex during spawn
        let pid = receipt.child.id();
        *self.child.lock().unwrap() = Some(receipt.child);
        Ok(Accepted { pid })
    }
}
fn wait_owned(backend: &FixtureService) {
    let mut child = backend.child.lock().unwrap().take().expect("owned fixture");
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            assert!(status.success());
            return;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("owned consumer fixture timeout");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
#[test]
#[ignore = "requires this workspace's built SEI_APPS_FIXTURE; controlled fixture launch only"]
fn real_windows_launch_requires_selection_and_reports_removed_target() {
    let fixture = PathBuf::from(std::env::var_os("SEI_APPS_FIXTURE").expect("fixture path"));
    assert_eq!(fixture.file_name().unwrap(), "launch_fixture.exe");
    let temp = std::env::temp_dir().join(format!("sei-consumer-{}", std::process::id()));
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
        vec![
            output.clone().into_os_string(),
            "consumer &|% literal".into(),
        ],
        Some(temp.clone()),
    )
    .unwrap();
    let backend = Arc::new(FixtureService {
        entry: Entry {
            id: "fixture-v1".into(),
            display_name: "Test fixture".into(),
            source: Source::UserStartMenu,
            shortcut: temp.join("injected.lnk"),
            request,
            icon: None,
        },
        child: Mutex::new(None),
    });
    let mut consumer = Consumer::with_service(backend.clone());
    assert_eq!(consumer.request_launch(), Err(ConsumerError::NoSelection));
    consumer.request_refresh().unwrap();
    consumer.start_pending().unwrap();
    consumer.wait().unwrap();
    assert!(matches!(
        consumer.refresh_status(),
        Status::Succeeded { .. }
    ));
    assert!(!output.exists());
    assert!(backend.child.lock().unwrap().is_none());
    assert_eq!(consumer.request_launch(), Err(ConsumerError::NoSelection));
    consumer.select("fixture-v1").unwrap();
    let cancelled = consumer.request_launch().unwrap();
    consumer
        .cancel(sei_apps::consumer::Operation::Launch, cancelled)
        .unwrap();
    consumer.start_pending().unwrap();
    assert_eq!(consumer.launch_status(), &Status::Cancelled(cancelled));
    assert!(backend.child.lock().unwrap().is_none());
    assert!(!output.exists());
    let launch = consumer.request_launch().unwrap();
    consumer.start_pending().unwrap();
    consumer.wait().unwrap();
    assert!(
        matches!(consumer.launch_status(), Status::Succeeded { id, value } if *id == launch && value.pid > 0)
    );
    wait_owned(&backend);
    let expected: Vec<std::ffi::OsString> = vec!["consumer &|% literal".into()];
    assert_eq!(
        std::fs::read_to_string(&output).unwrap(),
        format!("{expected:?}\n{temp:?}")
    );
    std::fs::remove_file(&output).unwrap();
    std::fs::remove_file(owned).unwrap();
    let failed = consumer.request_launch().unwrap();
    consumer.start_pending().unwrap();
    consumer.wait().unwrap();
    assert!(
        matches!(consumer.launch_status(), Status::Failed { id, error: ConsumerError::Service(error) } if *id == failed && error.kind == ErrorKind::Missing)
    );
    assert!(!output.exists());
    assert!(backend.child.lock().unwrap().is_none());
    consumer.shutdown();
    assert!(consumer.is_closed());
}
