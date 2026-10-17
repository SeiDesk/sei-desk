//! Independent review regressions; controlled Rust services, not native OS claims.
use super::*;

#[test]
fn terminated_worker_with_panicking_payload_drop_must_report_failure() {
    struct Payload;
    impl Drop for Payload {
        fn drop(&mut self) {
            panic!("controlled panic payload destructor");
        }
    }
    struct Panics;
    impl Service for Panics {
        fn refresh(&self, _: &Cancellation) -> Result<Catalog, Error> {
            std::panic::panic_any(Payload)
        }
        fn launch(&self, _: LaunchRequest, _: &Cancellation) -> Result<Accepted, Error> {
            unreachable!()
        }
    }
    let mut consumer = Consumer::with_service(Arc::new(Panics));
    let id = consumer.request_refresh().unwrap();
    consumer.start_pending().unwrap();
    // Join first so missing completion is proved deterministically after worker
    // termination. Never call wait() on a known-empty queue and hang this test.
    let handle = consumer
        .refresh_work
        .as_mut()
        .unwrap()
        .handle
        .take()
        .unwrap();
    let exited = handle.join();
    println!("worker_join_panicked={}", exited.is_err());
    let completion = consumer.receiver.try_recv();
    println!("terminal_completion_present={}", completion.is_ok());
    assert!(
        matches!(completion, Ok(Completion::Refresh(result_id, Err(ConsumerError::WorkerPanicked))) if result_id == id),
        "a terminated worker must report failure; otherwise wait() retains a live sender and can block forever"
    );
}

#[test]
fn repeated_cancellation_and_dispatch_do_not_create_extra_workers() {
    let (mut consumer, calls, _) = setup();
    publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    let refresh = consumer.request_refresh().unwrap();
    let launch = consumer.request_launch().unwrap();
    consumer.start_pending().unwrap();
    let a = next(&calls);
    let b = next(&calls);
    for _ in 0..3 {
        consumer.start_pending().unwrap();
        assert!(consumer.cancel(Operation::Refresh, refresh).unwrap());
        assert!(consumer.cancel(Operation::Launch, launch).unwrap());
        assert_eq!(
            consumer.request_refresh(),
            Err(ConsumerError::Busy(Operation::Refresh))
        );
        assert_eq!(
            consumer.request_launch(),
            Err(ConsumerError::Busy(Operation::Launch))
        );
    }
    assert!(matches!(calls.try_recv(), Err(TryRecvError::Empty)));
    for call in [a, b] {
        match call {
            Call::Refresh(token, reply) => {
                assert!(token.is_cancelled());
                reply.send(Ok(catalog("discarded"))).unwrap();
            }
            Call::Launch(_, token, reply) => {
                assert!(token.is_cancelled());
                reply.send(Ok(Accepted { pid: 91 })).unwrap();
            }
        }
    }
    consumer.wait().unwrap();
    consumer.wait().unwrap();
    assert_eq!(consumer.refresh_status(), &Status::Cancelled(refresh));
    assert_eq!(consumer.snapshot().unwrap().catalog.entries[0].id, "app");
    assert_eq!(
        consumer.launch_status(),
        &Status::Succeeded {
            id: launch,
            value: Accepted { pid: 91 }
        }
    );
    assert!(!consumer.cancel(Operation::Refresh, refresh).unwrap());
    assert!(!consumer.cancel(Operation::Launch, launch).unwrap());
}

#[test]
fn shutdown_keeps_blocked_worker_handles_until_services_are_released() {
    let (mut consumer, calls, backend) = setup();
    publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    consumer.request_refresh().unwrap();
    consumer.request_launch().unwrap();
    consumer.start_pending().unwrap();
    let a = next(&calls);
    let b = next(&calls);
    consumer.begin_shutdown();
    assert!(
        !consumer
            .refresh_work
            .as_ref()
            .unwrap()
            .handle
            .as_ref()
            .unwrap()
            .is_finished()
    );
    assert!(
        !consumer
            .launch_work
            .as_ref()
            .unwrap()
            .handle
            .as_ref()
            .unwrap()
            .is_finished()
    );
    let barrier = Arc::new(std::sync::Barrier::new(2));
    let worker_barrier = barrier.clone();
    let joining = std::thread::spawn(move || {
        worker_barrier.wait();
        consumer.shutdown();
        assert!(consumer.snapshot().is_none());
        assert!(consumer.refresh_work.is_none() && consumer.launch_work.is_none());
        assert_eq!(consumer.refresh_status(), &Status::Disposed);
        assert_eq!(consumer.launch_status(), &Status::Disposed);
        assert!(matches!(
            consumer.receiver.try_recv(),
            Err(TryRecvError::Empty)
        ));
    });
    barrier.wait();
    // Fake calls deliberately ignore cancellation until explicitly released.
    // This exercises ownership during a bounded simulated blocking call; it is
    // not a claim that an indefinitely stuck Windows call was interrupted.
    for call in [a, b] {
        match call {
            Call::Refresh(token, reply) => {
                assert!(token.is_cancelled());
                reply.send(Ok(catalog("late"))).unwrap();
            }
            Call::Launch(_, token, reply) => {
                assert!(token.is_cancelled());
                reply.send(Ok(Accepted { pid: 92 })).unwrap();
            }
        }
    }
    joining.join().unwrap();
    assert_eq!(Arc::strong_count(&backend), 1);
}

#[test]
fn ordinary_launch_worker_panic_is_not_retried_and_keeps_selection() {
    struct LaunchPanics;
    impl Service for LaunchPanics {
        fn refresh(&self, _: &Cancellation) -> Result<Catalog, Error> {
            Ok(catalog("app"))
        }
        fn launch(&self, _: LaunchRequest, _: &Cancellation) -> Result<Accepted, Error> {
            panic!("controlled launch worker panic")
        }
    }
    let mut consumer = Consumer::with_service(Arc::new(LaunchPanics));
    consumer.request_refresh().unwrap();
    consumer.start_pending().unwrap();
    consumer.wait().unwrap();
    consumer.select("app").unwrap();
    let id = consumer.request_launch().unwrap();
    consumer.start_pending().unwrap();
    consumer.wait().unwrap();
    assert_eq!(
        consumer.launch_status(),
        &Status::Failed {
            id,
            error: ConsumerError::WorkerPanicked
        }
    );
    assert_eq!(consumer.selection().unwrap().entry_id, "app");
    assert!(consumer.launch_work.is_none());
    assert!(consumer.drain_ready().unwrap().is_empty());
}

// Deadline is a test-hang safeguard, not the runtime completion mechanism.
fn wait_bounded(mut consumer: Consumer) -> Consumer {
    let (tx, rx) = mpsc::sync_channel(1);
    let waiter = std::thread::spawn(move || {
        let notice = consumer.wait().unwrap().expect("terminal wakeup");
        assert!(!notice.stale_discarded);
        tx.send(consumer).unwrap();
    });
    let consumer = rx
        .recv_timeout(Duration::from_secs(10))
        .expect("wait must wake after worker failure");
    waiter.join().unwrap();
    consumer
}

#[test]
fn public_wait_wakes_on_payload_cleanup_panic_in_both_lanes() {
    struct Payload;
    impl Drop for Payload {
        fn drop(&mut self) {
            panic!("payload cleanup panic");
        }
    }
    struct Panics(Operation);
    impl Service for Panics {
        fn refresh(&self, _: &Cancellation) -> Result<Catalog, Error> {
            if self.0 == Operation::Refresh {
                std::panic::panic_any(Payload);
            }
            Ok(catalog("app"))
        }
        fn launch(&self, _: LaunchRequest, _: &Cancellation) -> Result<Accepted, Error> {
            std::panic::panic_any(Payload)
        }
    }
    for operation in [Operation::Refresh, Operation::Launch] {
        let mut consumer = Consumer::with_service(Arc::new(Panics(operation)));
        let id = match operation {
            Operation::Refresh => consumer.request_refresh().unwrap(),
            Operation::Launch => {
                consumer.request_refresh().unwrap();
                consumer.start_pending().unwrap();
                consumer = wait_bounded(consumer);
                consumer.select("app").unwrap();
                consumer.request_launch().unwrap()
            }
        };
        consumer.start_pending().unwrap();
        let mut consumer = wait_bounded(consumer);
        match operation {
            Operation::Refresh => assert_eq!(
                consumer.refresh_status(),
                &Status::Failed {
                    id,
                    error: ConsumerError::WorkerPanicked
                }
            ),
            Operation::Launch => assert_eq!(
                consumer.launch_status(),
                &Status::Failed {
                    id,
                    error: ConsumerError::WorkerPanicked
                }
            ),
        }
        assert!(consumer.drain_ready().unwrap().is_empty());
        assert!(consumer.wait().unwrap().is_none());
    }
}

/// Inject only the worker exit path, bypassing Service and normal publication.
/// Both tests still use the real publisher, channel, wait, finish and join.
fn exit_only_worker(consumer: &mut Consumer, operation: Operation, accepted: bool, panic: bool) {
    let slot = match operation {
        Operation::Refresh => &mut consumer.refresh_work,
        Operation::Launch => &mut consumer.launch_work,
    };
    let work = slot.as_mut().unwrap();
    let id = work.id;
    let sender = consumer.sender.clone();
    work.handle = Some(std::thread::spawn(move || {
        let publisher = TerminalPublisher::new(sender, operation, id);
        if accepted {
            publisher.publish(Completion::Launch(id, Ok(Accepted { pid: 101 })));
        }
        if panic {
            panic!("exit without normal publication / after acceptance");
        }
        // Otherwise normal early return, with only guard publication.
    }));
}

#[test]
fn exit_without_normal_publication_wakes_wait_on_return_and_unwind() {
    for panic in [false, true] {
        let mut consumer = Consumer::new();
        let id = consumer.request_refresh().unwrap();
        exit_only_worker(&mut consumer, Operation::Refresh, false, panic);
        let mut consumer = wait_bounded(consumer);
        assert_eq!(
            consumer.refresh_status(),
            &Status::Failed {
                id,
                error: ConsumerError::WorkerPanicked
            }
        );
        assert!(consumer.drain_ready().unwrap().is_empty());
        assert!(consumer.refresh_work.is_none());
    }
}

#[test]
fn published_acceptance_survives_later_panic_cancel_and_duplicate_result() {
    let (mut consumer, calls, _) = setup();
    publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    let id = consumer.request_launch().unwrap();
    exit_only_worker(&mut consumer, Operation::Launch, true, true);
    consumer.cancel(Operation::Launch, id).unwrap();
    let mut consumer = wait_bounded(consumer);
    let expected = Status::Succeeded {
        id,
        value: Accepted { pid: 101 },
    };
    assert_eq!(consumer.launch_status(), &expected);
    assert!(consumer.drain_ready().unwrap().is_empty());
    let notice = consumer.finish(Completion::Launch(id, Err(ConsumerError::WorkerPanicked)));
    assert!(notice.stale_discarded);
    assert_eq!(consumer.launch_status(), &expected);
}

#[test]
fn failed_worker_join_disposes_nested_panicking_payloads_without_unwinding_owner() {
    struct First;
    struct Second;
    impl Drop for First {
        fn drop(&mut self) {
            std::panic::panic_any(Second);
        }
    }
    impl Drop for Second {
        fn drop(&mut self) {
            panic!("last finite cleanup panic");
        }
    }
    let mut consumer = Consumer::new();
    let id = consumer.request_refresh().unwrap();
    let sender = consumer.sender.clone();
    consumer.refresh_work.as_mut().unwrap().handle = Some(std::thread::spawn(move || {
        let _publisher = TerminalPublisher::new(sender, Operation::Refresh, id);
        // No inner catch: original payload reaches join, exercising owner-side
        // disposal outside unwinding. Each replacement payload is also dropped.
        std::panic::panic_any(First);
    }));
    let consumer = wait_bounded(consumer);
    assert_eq!(
        consumer.refresh_status(),
        &Status::Failed {
            id,
            error: ConsumerError::WorkerPanicked
        }
    );
}

#[test]
fn shutdown_joins_exit_fallback_workers_without_publishing_after_disposal() {
    let mut consumer = Consumer::new();
    consumer.request_refresh().unwrap();
    exit_only_worker(&mut consumer, Operation::Refresh, false, true);
    consumer.begin_shutdown();
    consumer.shutdown();
    consumer.shutdown();
    assert_eq!(consumer.refresh_status(), &Status::Disposed);
    assert!(consumer.refresh_work.is_none());
    assert!(consumer.receiver.try_recv().is_err());
}

#[test]
fn both_exit_fallbacks_fit_the_queue_during_shutdown_without_a_receiver_drain() {
    let (mut consumer, calls, _) = setup();
    publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    consumer.request_refresh().unwrap();
    consumer.request_launch().unwrap();
    exit_only_worker(&mut consumer, Operation::Refresh, false, true);
    exit_only_worker(&mut consumer, Operation::Launch, false, true);
    // No receives until shutdown has joined both workers. A duplicate send
    // would exhaust the two-event bound and deadlock the joining thread.
    let (tx, rx) = mpsc::sync_channel(1);
    let joining = std::thread::spawn(move || {
        consumer.shutdown();
        assert_eq!(consumer.refresh_status(), &Status::Disposed);
        assert_eq!(consumer.launch_status(), &Status::Disposed);
        assert!(consumer.receiver.try_recv().is_err());
        tx.send(()).unwrap();
    });
    rx.recv_timeout(Duration::from_secs(10))
        .expect("both exits must join");
    joining.join().unwrap();
}

#[test]
fn owner_wait_is_woken_by_failure_released_from_a_service_gate() {
    struct Gated {
        entered: SyncSender<()>,
        release: std::sync::Mutex<Receiver<()>>,
    }
    impl Service for Gated {
        fn refresh(&self, _: &Cancellation) -> Result<Catalog, Error> {
            self.entered.send(()).unwrap();
            self.release
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(10))
                .unwrap();
            panic!("gate released into worker failure");
        }
        fn launch(&self, _: LaunchRequest, _: &Cancellation) -> Result<Accepted, Error> {
            unreachable!()
        }
    }
    let (entered, entered_rx) = mpsc::sync_channel(1);
    let (release, release_rx) = mpsc::sync_channel(1);
    let mut consumer = Consumer::with_service(Arc::new(Gated {
        entered,
        release: std::sync::Mutex::new(release_rx),
    }));
    let id = consumer.request_refresh().unwrap();
    consumer.start_pending().unwrap();
    entered_rx.recv_timeout(Duration::from_secs(10)).unwrap();
    let (ready, ready_rx) = mpsc::sync_channel(1);
    let (done, done_rx) = mpsc::sync_channel(1);
    let waiter = std::thread::spawn(move || {
        ready.send(()).unwrap();
        assert_eq!(consumer.wait().unwrap().unwrap().id, id);
        assert_eq!(
            consumer.refresh_status(),
            &Status::Failed {
                id,
                error: ConsumerError::WorkerPanicked
            }
        );
        done.send(()).unwrap();
    });
    ready_rx.recv_timeout(Duration::from_secs(10)).unwrap();
    // Scheduling may place the receive just before or just after this release;
    // either ordering must work, with no timing-based lost-wakeup assumption.
    release.send(()).unwrap();
    done_rx
        .recv_timeout(Duration::from_secs(10))
        .expect("failure must wake waiter");
    waiter.join().unwrap();
}
