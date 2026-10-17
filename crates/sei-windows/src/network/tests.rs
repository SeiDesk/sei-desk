use super::*;
fn adapter(id: u64, rx: u64, tx: u64) -> Interface {
    Interface {
        id: Identity {
            guid: u128::from(id),
            luid: id,
        },
        kind: 6,
        flags: 5,
        operational: true,
        admin_up: true,
        connected: true,
        received: rx,
        transmitted: tx,
    }
}
fn live(reading: Reading) -> Rates {
    match reading.status {
        Status::Live(rates) => rates,
        other => panic!("Expected live: {other:?}"),
    }
}
#[test]
fn baseline_then_elapsed_time_deltas_and_real_zero() {
    let mut tracker = Tracker::default();
    let at = Instant::now();
    assert!(matches!(
        tracker.observe(Ok(vec![adapter(1, 50, 20)]), at).status,
        Status::Baseline(_)
    ));
    assert_eq!(
        live(tracker.observe(
            Ok(vec![adapter(1, 350, 170)]),
            at + Duration::from_millis(1500)
        )),
        Rates {
            receive: 200.0,
            transmit: 100.0
        }
    );
    assert_eq!(
        live(tracker.observe(Ok(vec![adapter(1, 350, 170)]), at + Duration::from_secs(4))),
        Rates {
            receive: 0.0,
            transmit: 0.0
        }
    );
}
#[test]
fn decrease_reset_and_u64_wrap_rebaseline_without_spike() {
    for (old_rx, old_tx, new_rx, new_tx) in [
        (100, 50, 99, 60),
        (100, 50, 110, 1),
        (u64::MAX - 5, 50, 5, 60),
    ] {
        let mut tracker = Tracker::default();
        let at = Instant::now();
        tracker.observe(Ok(vec![adapter(1, old_rx, old_tx)]), at);
        assert!(matches!(
            tracker
                .observe(
                    Ok(vec![adapter(1, new_rx, new_tx)]),
                    at + Duration::from_secs(2)
                )
                .status,
            Status::Baseline(_)
        ));
        assert_eq!(
            live(tracker.observe(
                Ok(vec![adapter(1, new_rx + 20, new_tx + 10)]),
                at + Duration::from_secs(4)
            )),
            Rates {
                receive: 10.0,
                transmit: 5.0
            }
        );
    }
}
#[test]
fn physical_links_sum_once_and_input_order_does_not_matter() {
    let mut tracker = Tracker::default();
    let at = Instant::now();
    tracker.observe(Ok(vec![adapter(1, 50, 100), adapter(2, 0, 20)]), at);
    let reading = tracker.observe(
        Ok(vec![adapter(2, 300, 220), adapter(1, 150, 300)]),
        at + Duration::from_secs(2),
    );
    assert_eq!(reading.cohort.len(), 2);
    assert_eq!(
        live(reading),
        Rates {
            receive: 200.0,
            transmit: 200.0
        }
    );
}
#[test]
fn addition_removal_and_same_index_different_guid_or_luid_require_baseline() {
    let at = Instant::now();
    let mut changed_guid = adapter(1, 999_999, 999_999);
    changed_guid.id.guid = 42;
    let mut changed_luid = adapter(1, 999_999, 999_999);
    changed_luid.id.luid = 42;
    for changed in [
        vec![adapter(1, 100, 100), adapter(2, 999_999, 999_999)],
        vec![adapter(2, 100, 100)],
        vec![changed_guid],
        vec![changed_luid],
    ] {
        let mut tracker = Tracker::default();
        tracker.observe(Ok(vec![adapter(1, 0, 0)]), at);
        assert!(matches!(
            tracker
                .observe(Ok(changed.clone()), at + Duration::from_secs(2))
                .status,
            Status::Baseline(_)
        ));
        assert_eq!(
            live(tracker.observe(Ok(changed), at + Duration::from_secs(4))),
            Rates {
                receive: 0.0,
                transmit: 0.0
            }
        );
    }
}
#[test]
fn duplicate_guid_or_luid_never_double_counts() {
    for second in [
        adapter(1, 100, 100),
        Interface {
            id: Identity { guid: 1, luid: 2 },
            ..adapter(2, 100, 100)
        },
        Interface {
            id: Identity { guid: 2, luid: 1 },
            ..adapter(2, 100, 100)
        },
    ] {
        let reading =
            Tracker::default().observe(Ok(vec![adapter(1, 100, 100), second]), Instant::now());
        assert!(matches!(reading.status, Status::Unavailable(_)));
    }
}
#[test]
fn special_disconnected_and_filter_layers_are_excluded() {
    let physical = adapter(1, 0, 0);
    assert!(physical.included());
    assert!(
        Interface {
            kind: 71,
            ..physical.clone()
        }
        .included()
    );
    for kind in [1, 24, 53, 131] {
        assert!(
            !Interface {
                kind,
                ..physical.clone()
            }
            .included()
        );
    }
    for flags in [0, 3, 5 | 8, 5 | 16, 5 | 32, 5 | 64, 5 | 128] {
        assert!(
            !Interface {
                flags,
                ..physical.clone()
            }
            .included()
        );
    }
    for excluded in [
        Interface {
            connected: false,
            ..physical.clone()
        },
        Interface {
            operational: false,
            ..physical.clone()
        },
        Interface {
            admin_up: false,
            ..physical
        },
    ] {
        assert!(!excluded.included());
        assert!(matches!(
            Tracker::default()
                .observe(Ok(vec![excluded]), Instant::now())
                .status,
            Status::Unavailable(_)
        ));
    }
}
#[test]
fn disconnect_reconnect_and_failed_queries_break_baselines() {
    let mut tracker = Tracker::default();
    let at = Instant::now();
    tracker.observe(Ok(vec![adapter(1, 0, 0)]), at);
    assert!(matches!(
        tracker
            .observe(Ok(vec![]), at + Duration::from_secs(2))
            .status,
        Status::Unavailable(_)
    ));
    assert!(matches!(
        tracker
            .observe(Ok(vec![adapter(1, 500, 500)]), at + Duration::from_secs(4))
            .status,
        Status::Baseline(_)
    ));
    let failed = tracker.observe(
        Err(NativeError {
            operation: "test",
            code: 5,
        }),
        at + Duration::from_secs(6),
    );
    assert!(matches!(failed.status, Status::Failed(_)));
    assert!(failed.stale(at + Duration::from_secs(11)));
    assert!(matches!(
        tracker
            .observe(Ok(vec![adapter(1, 999, 999)]), at + Duration::from_secs(8))
            .status,
        Status::Baseline(_)
    ));
}
#[test]
fn invalid_times_long_gap_and_delayed_interval_are_explicit() {
    let at = Instant::now();
    let mut tracker = Tracker::default();
    tracker.observe(Ok(vec![adapter(1, 100, 100)]), at);
    for instant in [at, at - Duration::from_secs(1)] {
        assert!(matches!(
            tracker
                .observe(Ok(vec![adapter(1, 200, 200)]), instant)
                .status,
            Status::Unavailable(_)
        ));
    }
    assert_eq!(
        live(tracker.observe(Ok(vec![adapter(1, 700, 400)]), at + Duration::from_secs(3))),
        Rates {
            receive: 200.0,
            transmit: 100.0
        }
    );
    assert!(matches!(
        tracker
            .observe(
                Ok(vec![adapter(1, 10_000, 10_000)]),
                at + Duration::from_secs(14)
            )
            .status,
        Status::Baseline(_)
    ));
    assert!(matches!(
        tracker
            .observe(
                Ok(vec![adapter(1, 10_001, 10_001)]),
                at + Duration::from_secs(14) + Duration::from_millis(1)
            )
            .status,
        Status::Baseline(_)
    ));
}
#[test]
fn invalid_id_and_oversize_table_fail_closed() {
    let at = Instant::now();
    assert!(matches!(
        Tracker::default()
            .observe(Ok(vec![adapter(0, 0, 0)]), at)
            .status,
        Status::Unavailable(_)
    ));
    assert!(matches!(
        Tracker::default()
            .observe(Ok(vec![adapter(1, 0, 0); MAX_INTERFACES + 1]), at)
            .status,
        Status::Unavailable(_)
    ));
}
