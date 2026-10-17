use super::*;
use crate::{Entry, Issue, Source};
use std::{
    path::PathBuf,
    sync::mpsc::{RecvTimeoutError, TryRecvError},
    time::Duration,
};

enum Call {
    Refresh(Cancellation, SyncSender<Result<Catalog, Error>>),
    Launch(
        LaunchRequest,
        Cancellation,
        SyncSender<Result<Accepted, Error>>,
    ),
}
struct Controlled {
    calls: SyncSender<Call>,
}
impl Service for Controlled {
    fn refresh(&self, cancel: &Cancellation) -> Result<Catalog, Error> {
        let (tx, rx) = mpsc::sync_channel(0);
        self.calls.send(Call::Refresh(cancel.clone(), tx)).unwrap();
        rx.recv_timeout(Duration::from_secs(10))
            .expect("test must release refresh gate")
    }
    fn launch(&self, request: LaunchRequest, cancel: &Cancellation) -> Result<Accepted, Error> {
        let (tx, rx) = mpsc::sync_channel(0);
        self.calls
            .send(Call::Launch(request, cancel.clone(), tx))
            .unwrap();
        rx.recv_timeout(Duration::from_secs(10))
            .expect("test must release launch gate")
    }
}
fn setup() -> (Consumer, Receiver<Call>, Arc<Controlled>) {
    let (tx, rx) = mpsc::sync_channel(2);
    let backend = Arc::new(Controlled { calls: tx });
    (Consumer::with_service(backend.clone()), rx, backend)
}
fn next(rx: &Receiver<Call>) -> Call {
    rx.recv_timeout(Duration::from_secs(5))
        .expect("worker must reach test gate")
}
fn catalog(name: &str) -> Catalog {
    // State-only fixture. No metadata validation or process creation is claimed.
    Catalog {
        entries: vec![Entry {
            id: name.into(),
            display_name: name.into(),
            source: Source::UserStartMenu,
            shortcut: PathBuf::from("fixture.lnk"),
            request: LaunchRequest {
                executable: PathBuf::from("fixture.exe"),
                arguments: crate::Arguments::Structured(vec![]),
                working_directory: None,
            },
            icon: None,
        }],
        ..Catalog::default()
    }
}
fn publish(consumer: &mut Consumer, calls: &Receiver<Call>, name: &str) -> OperationId {
    let id = consumer.request_refresh().unwrap();
    assert_eq!(consumer.refresh_status(), &Status::Pending(id));
    consumer.start_pending().unwrap();
    let Call::Refresh(_, reply) = next(calls) else {
        panic!("expected refresh")
    };
    reply.send(Ok(catalog(name))).unwrap();
    assert!(!consumer.wait().unwrap().unwrap().stale_discarded);
    id
}
#[test]
fn refresh_requests_rejected_while_pending_running_or_cancel_requested() {
    let (mut consumer, calls, _) = setup();
    assert_eq!(consumer.refresh_status(), &Status::Idle);
    let id = consumer.request_refresh().unwrap();
    assert_eq!(
        consumer.request_refresh(),
        Err(ConsumerError::Busy(Operation::Refresh))
    );
    assert!(matches!(calls.try_recv(), Err(TryRecvError::Empty)));
    consumer.start_pending().unwrap();
    let Call::Refresh(cancel, reply) = next(&calls) else {
        panic!()
    };
    assert_eq!(
        consumer.request_refresh(),
        Err(ConsumerError::Busy(Operation::Refresh))
    );
    consumer.cancel(Operation::Refresh, id).unwrap();
    assert!(cancel.is_cancelled());
    assert_eq!(
        consumer.request_refresh(),
        Err(ConsumerError::Busy(Operation::Refresh))
    );
    reply.send(Ok(catalog("cancelled"))).unwrap();
    consumer.wait().unwrap();
    assert_eq!(consumer.refresh_status(), &Status::Cancelled(id));
    assert!(consumer.snapshot().is_none());
    assert!(consumer.request_refresh().unwrap() > id);
}
#[test]
fn selection_is_explicit_and_never_launches_on_refresh_or_selection() {
    let (mut consumer, calls, _) = setup();
    assert_eq!(consumer.request_launch(), Err(ConsumerError::NoSelection));
    publish(&mut consumer, &calls, "app");
    assert_eq!(consumer.request_launch(), Err(ConsumerError::NoSelection));
    consumer.select("app").unwrap();
    assert!(matches!(calls.try_recv(), Err(TryRecvError::Empty)));
    assert_eq!(consumer.launch_status(), &Status::Idle);
    assert_eq!(consumer.select("unknown"), Err(ConsumerError::UnknownEntry));
    assert!(consumer.selection().is_none());
    assert_eq!(consumer.request_launch(), Err(ConsumerError::NoSelection));
}
#[test]
fn successful_replacement_invalidates_selection_even_for_same_identity() {
    let (mut consumer, calls, _) = setup();
    let first = publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    let next = publish(&mut consumer, &calls, "app");
    assert!(next > first);
    assert!(consumer.selection().is_none());
    assert_eq!(consumer.request_launch(), Err(ConsumerError::NoSelection));
}
#[test]
fn failed_refresh_keeps_snapshot_selection_and_partial_catalog_issues() {
    let (mut consumer, calls, _) = setup();
    let generation = publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    let id = consumer.request_refresh().unwrap();
    consumer.start_pending().unwrap();
    let Call::Refresh(_, reply) = next(&calls) else {
        panic!()
    };
    let error = Error::new(ErrorKind::AccessDenied, "fake source");
    reply.send(Err(error.clone())).unwrap();
    consumer.wait().unwrap();
    assert_eq!(
        consumer.refresh_status(),
        &Status::Failed {
            id,
            error: ConsumerError::Service(error)
        }
    );
    assert_eq!(consumer.snapshot().unwrap().generation, generation);
    assert_eq!(consumer.selection().unwrap().entry_id, "app");
    let next_id = consumer.request_refresh().unwrap();
    consumer.start_pending().unwrap();
    let Call::Refresh(_, reply) = next(&calls) else {
        panic!()
    };
    let mut partial = catalog("known");
    partial.truncated = true;
    partial.issues.push(Issue {
        source: Source::UserStartMenu,
        path: PathBuf::new(),
        error: Error::new(ErrorKind::Missing, "fake partial"),
    });
    reply.send(Ok(partial)).unwrap();
    consumer.wait().unwrap();
    assert_eq!(
        consumer.refresh_status(),
        &Status::Succeeded {
            id: next_id,
            value: RefreshSummary {
                entries: 1,
                issues: 1,
                truncated: true
            }
        }
    );
}
#[test]
fn replayed_and_out_of_order_refresh_completions_cannot_replace_current_state() {
    let (mut consumer, calls, _) = setup();
    let old = publish(&mut consumer, &calls, "old");
    let new = consumer.request_refresh().unwrap();
    consumer.start_pending().unwrap();
    let Call::Refresh(_, reply) = next(&calls) else {
        panic!()
    };
    // Serial policy forbids real simultaneous refreshes. Replay injection tests
    // the defensive boundary separately from the actual worker schedule.
    let notice = consumer.finish(Completion::Refresh(old, Ok(catalog("replayed"))));
    assert!(notice.stale_discarded);
    assert_eq!(consumer.snapshot().unwrap().catalog.entries[0].id, "old");
    assert!(matches!(consumer.refresh_status(), Status::Running { id, .. } if *id == new));
    reply.send(Ok(catalog("new"))).unwrap();
    consumer.wait().unwrap();
    assert!(
        consumer
            .finish(Completion::Refresh(old, Ok(catalog("late"))))
            .stale_discarded
    );
    assert_eq!(consumer.snapshot().unwrap().generation, new);
    assert_eq!(consumer.snapshot().unwrap().catalog.entries[0].id, "new");
    assert_eq!(consumer.last_discarded().unwrap().id, old);
}
#[test]
fn id_exhaustion_never_wraps_or_mutates_existing_snapshot() {
    let (mut consumer, calls, _) = setup();
    consumer.next_id = u64::MAX - 1;
    let last = publish(&mut consumer, &calls, "app");
    assert_eq!(last.get(), u64::MAX);
    consumer.select("app").unwrap();
    assert_eq!(consumer.request_refresh(), Err(ConsumerError::IdExhausted));
    assert_eq!(consumer.request_launch(), Err(ConsumerError::IdExhausted));
    assert_eq!(consumer.next_id, u64::MAX);
    assert_eq!(consumer.snapshot().unwrap().generation, last);
    assert!(consumer.selection().is_some());
    assert_eq!(consumer.launch_status(), &Status::Idle);
}
#[test]
fn catalog_replacement_rejects_pending_old_snapshot_launch_before_effects() {
    let (mut consumer, calls, _) = setup();
    publish(&mut consumer, &calls, "old");
    consumer.select("old").unwrap();
    consumer.request_refresh().unwrap();
    consumer.start_pending().unwrap();
    let Call::Refresh(_, reply) = next(&calls) else {
        panic!()
    };
    let launch = consumer.request_launch().unwrap();
    reply.send(Ok(catalog("new"))).unwrap();
    consumer.wait().unwrap();
    consumer.start_pending().unwrap();
    assert_eq!(
        consumer.launch_status(),
        &Status::Failed {
            id: launch,
            error: ConsumerError::StaleSelection
        }
    );
    assert!(matches!(calls.try_recv(), Err(TryRecvError::Empty)));
}
#[test]
fn refresh_and_launch_complete_in_either_order_without_status_conflicts() {
    for refresh_first in [true, false] {
        let (mut consumer, calls, _) = setup();
        publish(&mut consumer, &calls, "old");
        consumer.select("old").unwrap();
        let refresh = consumer.request_refresh().unwrap();
        consumer.start_pending().unwrap();
        let Call::Refresh(_, refresh_reply) = next(&calls) else {
            panic!()
        };
        let launch = consumer.request_launch().unwrap();
        consumer.start_pending().unwrap();
        let Call::Launch(request, _, launch_reply) = next(&calls) else {
            panic!()
        };
        assert_eq!(request.executable, PathBuf::from("fixture.exe"));
        assert_eq!(
            consumer.request_launch(),
            Err(ConsumerError::Busy(Operation::Launch))
        );
        if refresh_first {
            refresh_reply.send(Ok(catalog("new"))).unwrap();
            consumer.wait().unwrap();
            assert!(matches!(consumer.launch_status(), Status::Running { .. }));
            launch_reply.send(Ok(Accepted { pid: 42 })).unwrap();
            consumer.wait().unwrap();
        } else {
            launch_reply.send(Ok(Accepted { pid: 42 })).unwrap();
            consumer.wait().unwrap();
            assert!(matches!(consumer.refresh_status(), Status::Running { .. }));
            refresh_reply.send(Ok(catalog("new"))).unwrap();
            consumer.wait().unwrap();
        }
        assert!(
            matches!(consumer.refresh_status(), Status::Succeeded { id, .. } if *id == refresh)
        );
        assert_eq!(
            consumer.launch_status(),
            &Status::Succeeded {
                id: launch,
                value: Accepted { pid: 42 }
            }
        );
        assert!(consumer.selection().is_none());
    }
}
#[test]
fn launch_failure_retains_selection_and_maps_stale_target_error() {
    let (mut consumer, calls, _) = setup();
    publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    let id = consumer.request_launch().unwrap();
    consumer.start_pending().unwrap();
    let Call::Launch(_, _, reply) = next(&calls) else {
        panic!()
    };
    let error = Error::new(ErrorKind::Missing, "fake stale target");
    reply.send(Err(error.clone())).unwrap();
    consumer.wait().unwrap();
    assert_eq!(
        consumer.launch_status(),
        &Status::Failed {
            id,
            error: ConsumerError::Service(error)
        }
    );
    assert_eq!(consumer.selection().unwrap().entry_id, "app");
    assert!(matches!(calls.try_recv(), Err(TryRecvError::Empty)));
}
#[test]
fn pending_cancellation_never_dispatches_a_service_call() {
    let (mut consumer, calls, _) = setup();
    let id = consumer.request_refresh().unwrap();
    consumer.cancel(Operation::Refresh, id).unwrap();
    consumer.start_pending().unwrap();
    assert_eq!(consumer.refresh_status(), &Status::Cancelled(id));
    assert!(matches!(calls.try_recv(), Err(TryRecvError::Empty)));
    publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    let id = consumer.request_launch().unwrap();
    consumer.cancel(Operation::Launch, id).unwrap();
    consumer.start_pending().unwrap();
    assert_eq!(consumer.launch_status(), &Status::Cancelled(id));
    assert!(matches!(calls.try_recv(), Err(TryRecvError::Empty)));
}
#[test]
fn cancellation_after_acceptance_cannot_hide_success() {
    let (mut consumer, calls, _) = setup();
    publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    let id = consumer.request_launch().unwrap();
    consumer.start_pending().unwrap();
    let Call::Launch(_, cancel, reply) = next(&calls) else {
        panic!()
    };
    consumer.cancel(Operation::Launch, id).unwrap();
    assert!(cancel.is_cancelled());
    reply.send(Ok(Accepted { pid: 7 })).unwrap();
    consumer.wait().unwrap();
    assert_eq!(
        consumer.launch_status(),
        &Status::Succeeded {
            id,
            value: Accepted { pid: 7 }
        }
    );
}
#[test]
fn running_launch_cancelled_before_creation_is_distinct_from_accepted() {
    let (mut consumer, calls, _) = setup();
    publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    let id = consumer.request_launch().unwrap();
    consumer.start_pending().unwrap();
    let Call::Launch(_, cancel, reply) = next(&calls) else {
        panic!()
    };
    consumer.cancel(Operation::Launch, id).unwrap();
    assert!(cancel.is_cancelled());
    reply
        .send(Err(Error::new(ErrorKind::Cancelled, "fake pre-spawn")))
        .unwrap();
    consumer.wait().unwrap();
    assert_eq!(consumer.launch_status(), &Status::Cancelled(id));
}
#[test]
fn shutdown_pending_work_never_starts_workers() {
    let (mut consumer, calls, backend) = setup();
    publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    consumer.request_refresh().unwrap();
    consumer.request_launch().unwrap();
    consumer.shutdown();
    consumer.shutdown();
    assert!(consumer.is_closed());
    assert!(consumer.snapshot().is_none());
    assert!(consumer.selection().is_none());
    assert_eq!(consumer.refresh_status(), &Status::Disposed);
    assert_eq!(consumer.launch_status(), &Status::Disposed);
    assert!(matches!(calls.try_recv(), Err(TryRecvError::Empty)));
    assert_eq!(Arc::strong_count(&backend), 2); // test + consumer; no worker
    assert_eq!(consumer.request_refresh(), Err(ConsumerError::Closed));
    assert_eq!(consumer.request_launch(), Err(ConsumerError::Closed));
    assert_eq!(consumer.start_pending(), Err(ConsumerError::Closed));
}
#[test]
fn shutdown_joins_two_running_workers_and_does_not_publish_late_results() {
    let (mut consumer, calls, backend) = setup();
    publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    consumer.request_refresh().unwrap();
    consumer.request_launch().unwrap();
    consumer.start_pending().unwrap();
    let a = next(&calls);
    let b = next(&calls);
    consumer.begin_shutdown();
    for call in [a, b] {
        match call {
            Call::Refresh(cancel, reply) => {
                assert!(cancel.is_cancelled());
                reply.send(Ok(catalog("late"))).unwrap();
            }
            Call::Launch(_, cancel, reply) => {
                assert!(cancel.is_cancelled());
                reply.send(Ok(Accepted { pid: 8 })).unwrap();
            }
        }
    }
    consumer.shutdown();
    assert!(consumer.refresh_work.is_none() && consumer.launch_work.is_none());
    assert!(consumer.snapshot().is_none());
    assert_eq!(consumer.launch_status(), &Status::Disposed);
    assert_eq!(consumer.drain_ready(), Err(ConsumerError::Closed));
    assert_eq!(Arc::strong_count(&backend), 2);
}
#[test]
fn drop_joins_completed_worker_instead_of_detaching_it() {
    let (mut consumer, calls, backend) = setup();
    consumer.request_refresh().unwrap();
    consumer.start_pending().unwrap();
    let Call::Refresh(cancel, reply) = next(&calls) else {
        panic!()
    };
    reply.send(Ok(catalog("unused"))).unwrap();
    drop(consumer);
    assert!(cancel.is_cancelled());
    assert_eq!(Arc::strong_count(&backend), 1);
}
#[test]
fn service_panic_becomes_structured_failure_and_owner_remains_usable() {
    struct Panics;
    impl Service for Panics {
        fn refresh(&self, _: &Cancellation) -> Result<Catalog, Error> {
            panic!("controlled service panic")
        }
        fn launch(&self, _: LaunchRequest, _: &Cancellation) -> Result<Accepted, Error> {
            unreachable!()
        }
    }
    let mut consumer = Consumer::with_service(Arc::new(Panics));
    let id = consumer.request_refresh().unwrap();
    consumer.start_pending().unwrap();
    consumer.wait().unwrap();
    assert_eq!(
        consumer.refresh_status(),
        &Status::Failed {
            id,
            error: ConsumerError::WorkerPanicked
        }
    );
    assert!(consumer.request_refresh().is_ok());
}
#[test]
fn idle_wait_and_bounded_drain_do_not_poll_or_create_workers() {
    let (mut consumer, calls, _) = setup();
    assert_eq!(consumer.wait().unwrap(), None);
    assert!(consumer.drain_ready().unwrap().is_empty());
    consumer.request_refresh().unwrap();
    assert_eq!(consumer.wait().unwrap(), None); // pending is not running
    assert!(matches!(
        calls.recv_timeout(Duration::ZERO),
        Err(RecvTimeoutError::Timeout)
    ));
}

#[test]
fn ready_drain_handles_both_lanes_once_without_more_work() {
    let (mut consumer, calls, _) = setup();
    publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    consumer.request_refresh().unwrap();
    consumer.request_launch().unwrap();
    consumer.start_pending().unwrap();
    for call in [next(&calls), next(&calls)] {
        match call {
            Call::Refresh(_, reply) => reply.send(Ok(catalog("new"))).unwrap(),
            Call::Launch(_, _, reply) => reply.send(Ok(Accepted { pid: 5 })).unwrap(),
        }
    }
    // Arrange two already-received completions back into the private queue,
    // avoiding a race or sleep when testing the nonblocking drain specifically.
    let a = consumer.receiver.recv().unwrap();
    let b = consumer.receiver.recv().unwrap();
    consumer.sender.send(a).unwrap();
    consumer.sender.send(b).unwrap();
    let notices = consumer.drain_ready().unwrap();
    assert_eq!(notices.len(), 2);
    assert!(notices.iter().all(|n| !n.stale_discarded));
    assert!(consumer.drain_ready().unwrap().is_empty());
    assert!(matches!(
        consumer.refresh_status(),
        Status::Succeeded { .. }
    ));
    assert!(matches!(consumer.launch_status(), Status::Succeeded { .. }));
}
#[test]
fn changing_selection_does_not_retarget_an_already_requested_launch() {
    let (mut consumer, calls, _) = setup();
    publish(&mut consumer, &calls, "app");
    consumer.select("app").unwrap();
    let launch = consumer.request_launch().unwrap();
    consumer.clear_selection().unwrap();
    consumer.start_pending().unwrap();
    let Call::Launch(request, _, reply) = next(&calls) else {
        panic!()
    };
    assert_eq!(request.executable, PathBuf::from("fixture.exe"));
    reply.send(Ok(Accepted { pid: 6 })).unwrap();
    consumer.wait().unwrap();
    assert_eq!(
        consumer.launch_status(),
        &Status::Succeeded {
            id: launch,
            value: Accepted { pid: 6 }
        }
    );
}

#[test]
fn stale_cancellation_id_cannot_cancel_a_new_operation() {
    let (mut consumer, calls, _) = setup();
    let old = publish(&mut consumer, &calls, "old");
    let new = consumer.request_refresh().unwrap();
    consumer.start_pending().unwrap();
    let Call::Refresh(cancel, reply) = next(&calls) else {
        panic!()
    };
    assert!(!consumer.cancel(Operation::Refresh, old).unwrap());
    assert!(!cancel.is_cancelled());
    assert!(
        matches!(consumer.refresh_status(), Status::Running { id, cancellation_requested: false } if *id == new)
    );
    reply.send(Ok(catalog("new"))).unwrap();
    consumer.wait().unwrap();
    assert_eq!(consumer.snapshot().unwrap().generation, new);
}

mod acceptance;
