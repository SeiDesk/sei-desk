use super::*;
use sei_apps::{Arguments, Catalog as NativeCatalog, Entry, Error, ErrorKind, Source};
use std::time::{Duration, Instant};
#[derive(Default)]
struct TestService {
    empty: AtomicBool,
    refresh_fail: AtomicBool,
    launch_fail: AtomicBool,
    launch_panic: AtomicBool,
    block_refresh: AtomicBool,
    block_launch: AtomicBool,
    reverse: AtomicBool,
    launched: Mutex<Vec<LaunchRequest>>,
}
fn entry(id: &str) -> Entry {
    Entry {
        id: id.into(),
        display_name: id.to_uppercase(),
        source: Source::UserStartMenu,
        shortcut: format!("C:\\Test\\{id}.lnk").into(),
        request: LaunchRequest::executable(format!("C:\\Test\\{id}.exe").into(), vec![], None)
            .unwrap(),
        icon: None,
    }
}
impl Service for TestService {
    fn refresh(&self, cancel: &Cancellation) -> Result<NativeCatalog, Error> {
        wait_gate(&self.block_refresh, cancel)?;
        if self.refresh_fail.load(Ordering::Acquire) {
            return Err(Error {
                kind: ErrorKind::AccessDenied,
                operation: "fixture refresh",
                code: Some(5),
            });
        }
        let mut entries = vec![entry("a"), entry("b")];
        if self.empty.load(Ordering::Acquire) {
            entries.clear();
        }
        if self.reverse.load(Ordering::Acquire) {
            entries.reverse();
        }
        Ok(NativeCatalog {
            entries,
            ..Default::default()
        })
    }
    fn launch(&self, request: LaunchRequest, cancel: &Cancellation) -> Result<Accepted, Error> {
        self.launched.lock().unwrap().push(request);
        wait_gate(&self.block_launch, cancel)?;
        assert!(
            !self.launch_panic.load(Ordering::Acquire),
            "controlled service panic"
        );
        if self.launch_fail.load(Ordering::Acquire) {
            return Err(Error {
                kind: ErrorKind::Missing,
                operation: "fixture launch",
                code: Some(2),
            });
        }
        Ok(Accepted { pid: 42 })
    }
}
fn wait_gate(block: &AtomicBool, cancel: &Cancellation) -> Result<(), Error> {
    let deadline = Instant::now() + Duration::from_secs(3);
    while block.load(Ordering::Acquire) {
        if cancel.is_cancelled() {
            return Err(Error {
                kind: ErrorKind::Cancelled,
                operation: "test gate",
                code: None,
            });
        }
        assert!(Instant::now() < deadline, "test gate timed out");
        std::thread::sleep(Duration::from_millis(2));
    }
    Ok(())
}
#[track_caller]
fn wait(handle: &Handle, predicate: impl Fn(&Frame) -> bool) -> Arc<Frame> {
    let deadline = Instant::now() + Duration::from_secs(4);
    loop {
        if let Some(frame) = handle.feed.0.value.lock().unwrap().clone()
            && predicate(&frame)
        {
            return frame;
        }
        assert!(
            Instant::now() < deadline,
            "adapter did not publish expected state; last={:?}",
            handle.feed.0.value.lock().unwrap()
        );
        std::thread::sleep(Duration::from_millis(2));
    }
}
fn load(handle: &Handle) -> Arc<Frame> {
    let ack = handle.send(Command::Refresh).unwrap();
    wait(handle, |f| {
        f.ack >= ack && matches!(f.refresh, Status::Succeeded { .. })
    })
}
fn key(frame: &Frame, id: &str) -> Key {
    Key {
        generation: frame.catalog.as_ref().unwrap().generation,
        id: id.into(),
    }
}
#[test]
fn refresh_failure_keeps_last_good_catalog_and_rejects_stale_generation_launch() {
    let service = Arc::new(TestService::default());
    let owner = Owner::with_service(service.clone()).unwrap();
    let handle = owner.handle();
    let first = load(&handle);
    service.refresh_fail.store(true, Ordering::Release);
    let ack = handle.send(Command::Refresh).unwrap();
    let failed = wait(&handle, |f| {
        f.ack >= ack && matches!(f.refresh, Status::Failed { .. })
    });
    assert!(Arc::ptr_eq(
        first.catalog.as_ref().unwrap(),
        failed.catalog.as_ref().unwrap()
    ));
    service.refresh_fail.store(false, Ordering::Release);
    service.reverse.store(true, Ordering::Release);
    let second = load(&handle);
    assert_ne!(
        first.catalog.as_ref().unwrap().generation,
        second.catalog.as_ref().unwrap().generation
    );
    let ack = handle
        .send(Command::Launch(Some(key(&first, "a"))))
        .unwrap();
    let stale = wait(&handle, |f| f.ack >= ack && f.stale);
    assert_eq!(stale.error, Some(ConsumerError::StaleSelection));
    assert!(service.launched.lock().unwrap().is_empty());
}
#[test]
fn launch_requires_selection_and_captures_identity_across_changes_and_duplicate_requests() {
    let service = Arc::new(TestService::default());
    let owner = Owner::with_service(service.clone()).unwrap();
    let handle = owner.handle();
    let frame = load(&handle);
    let ack = handle.send(Command::Launch(None)).unwrap();
    wait(&handle, |f| {
        f.ack >= ack && f.error == Some(ConsumerError::NoSelection)
    });
    assert!(service.launched.lock().unwrap().is_empty());
    service.block_launch.store(true, Ordering::Release);
    let ack = handle
        .send(Command::Launch(Some(key(&frame, "a"))))
        .unwrap();
    wait(&handle, |f| f.ack >= ack && busy(&f.launch));
    let ack = handle
        .send(Command::Select(Some(key(&frame, "b"))))
        .unwrap();
    wait(&handle, |f| f.ack >= ack);
    let ack = handle
        .send(Command::Launch(Some(key(&frame, "b"))))
        .unwrap();
    wait(&handle, |f| {
        f.ack >= ack && f.error == Some(ConsumerError::Busy(Operation::Launch))
    });
    service.block_launch.store(false, Ordering::Release);
    let success = wait(&handle, |f| matches!(f.launch, Status::Succeeded { .. }));
    assert_eq!(success.launch_name.as_deref(), Some("A"));
    let requests = service.launched.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].executable, entry("a").request.executable);
    assert_eq!(requests[0].arguments, Arguments::Structured(vec![]));
    drop(requests);
    service.launch_fail.store(true, Ordering::Release);
    let ack = handle
        .send(Command::Launch(Some(key(&frame, "b"))))
        .unwrap();
    let failure = wait(&handle, |f| {
        f.ack >= ack && matches!(f.launch, Status::Failed { .. })
    });
    assert_eq!(failure.launch_name.as_deref(), Some("B"));
}
#[test]
fn cancelled_refresh_cannot_publish_late_data_and_shutdown_joins_blocked_work() {
    let service = Arc::new(TestService::default());
    let owner = Owner::with_service(service.clone()).unwrap();
    let handle = owner.handle();
    let first = load(&handle);
    service.block_refresh.store(true, Ordering::Release);
    let ack = handle.send(Command::Refresh).unwrap();
    wait(&handle, |f| f.ack >= ack && busy(&f.refresh));
    let ack = handle.send(Command::Cancel).unwrap();
    let cancelled = wait(&handle, |f| {
        f.ack >= ack && matches!(f.refresh, Status::Cancelled(_))
    });
    assert!(Arc::ptr_eq(
        first.catalog.as_ref().unwrap(),
        cancelled.catalog.as_ref().unwrap()
    ));
    let ack = handle.send(Command::Refresh).unwrap();
    wait(&handle, |f| f.ack >= ack && busy(&f.refresh));
    let start = Instant::now();
    drop(owner);
    assert!(start.elapsed() < Duration::from_secs(2));
    assert!(handle.feed.0.closed.load(Ordering::Acquire));
}
#[test]
fn service_panic_wakes_consumer_and_terminal_results_survive_ui_backpressure() {
    let service = Arc::new(TestService::default());
    let owner = Owner::with_service(service.clone()).unwrap();
    let handle = owner.handle();
    let frame = load(&handle);
    service.launch_panic.store(true, Ordering::Release);
    let ack = handle
        .send(Command::Launch(Some(key(&frame, "a"))))
        .unwrap();
    let failed = wait(&handle, |f| {
        f.ack >= ack
            && matches!(
                f.launch,
                Status::Failed {
                    error: ConsumerError::WorkerPanicked,
                    ..
                }
            )
    });
    assert_eq!(failed.launch_name.as_deref(), Some("A"));
    service.launch_panic.store(false, Ordering::Release);
    let ack = handle
        .send(Command::Launch(Some(key(&frame, "a"))))
        .unwrap();
    wait(&handle, |f| {
        f.ack >= ack && matches!(f.launch, Status::Succeeded { .. })
    });
    let refreshed = load(&handle);
    assert!(matches!(refreshed.launch, Status::Succeeded { .. }));
    assert_eq!(refreshed.launch_name.as_deref(), Some("A"));
    // No UI stream has consumed any publication; only the latest complete
    // snapshot remains and it still contains the accepted launch receipt.
    assert_eq!(
        handle
            .feed
            .0
            .value
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .revision,
        refreshed.revision
    );
}
#[test]
fn command_capacity_is_bounded_and_latest_mailbox_is_one_slot() {
    let (sender, _receiver) = mpsc::channel(4);
    let latest = Arc::new(Latest::default());
    let handle = Handle {
        sender: Arc::new(Mutex::new(sender)),
        sequence: Arc::new(Mutex::new(0)),
        stop: Arc::new(Stop::default()),
        feed: Feed(latest.clone()),
    };
    let accepted = (0..1000)
        .filter(|_| handle.send(Command::Refresh).is_ok())
        .count();
    assert_eq!(accepted, 5);
    for revision in 1..1000 {
        latest.publish(Frame {
            revision,
            ..Default::default()
        });
    }
    assert_eq!(latest.value.lock().unwrap().as_ref().unwrap().revision, 999);
    assert!(latest.value.lock().unwrap().take().is_some());
    assert!(latest.value.lock().unwrap().is_none());
}

#[test]
fn empty_refresh_and_two_simultaneous_lanes_publish_without_ui_drain() {
    let service = Arc::new(TestService::default());
    let owner = Owner::with_service(service.clone()).unwrap();
    let handle = owner.handle();
    let first = load(&handle);
    service.block_launch.store(true, Ordering::Release);
    let ack = handle
        .send(Command::Launch(Some(key(&first, "a"))))
        .unwrap();
    wait(&handle, |f| f.ack >= ack && busy(&f.launch));
    service.block_refresh.store(true, Ordering::Release);
    let ack = handle.send(Command::Refresh).unwrap();
    wait(&handle, |f| {
        f.ack >= ack && busy(&f.refresh) && busy(&f.launch)
    });
    service.empty.store(true, Ordering::Release);
    service.block_launch.store(false, Ordering::Release);
    service.block_refresh.store(false, Ordering::Release);
    let final_frame = wait(&handle, |f| {
        matches!(f.refresh, Status::Succeeded { .. })
            && matches!(f.launch, Status::Succeeded { .. })
    });
    assert!(final_frame.catalog.as_ref().unwrap().entries.is_empty());
    assert_eq!(final_frame.launch_name.as_deref(), Some("A"));
    assert_eq!(service.launched.lock().unwrap().len(), 1);
}

#[test]
fn immediate_cancel_cannot_lose_a_pre_entry_completion() {
    let service = Arc::new(TestService::default());
    service.block_refresh.store(true, Ordering::Release);
    let owner = Owner::with_service(service).unwrap();
    let handle = owner.handle();
    for _ in 0..20 {
        handle.send(Command::Refresh).unwrap();
        let ack = handle.send(Command::Cancel).unwrap();
        let frame = wait(&handle, |f| {
            f.ack >= ack && matches!(f.refresh, Status::Cancelled(_))
        });
        assert!(frame.catalog.is_none());
    }
}
