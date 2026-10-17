use super::*;
fn reading(at: Instant, rx: f64, tx: f64) -> Reading {
    Reading {
        at,
        cohort: vec![Identity { guid: 1, luid: 1 }],
        status: Status::Live(Rates {
            receive: rx,
            transmit: tx,
        }),
    }
}
#[test]
fn empty_single_and_zero_samples_have_safe_geometry() {
    let at = Instant::now();
    let mut history = History::default();
    assert!(positions(&history, at).is_empty());
    history.observe(&reading(at, 0.0, 0.0));
    assert_eq!(positions(&history, at), vec![(1.0, 0.0, 0.0)]);
    assert!(positions(&history, at - Duration::from_secs(1)).is_empty());
}
#[test]
fn capacity_and_time_window_evict_without_fabrication() {
    let at = Instant::now();
    let mut history = History::default();
    for second in 0..100 {
        history.observe(&reading(
            at + Duration::from_secs(second),
            second as f64,
            1.0,
        ));
    }
    assert_eq!(history.len(), CAPACITY);
    assert_eq!(
        history.points().next().unwrap().at,
        at + Duration::from_secs(68)
    );
    history.prune(at + Duration::from_secs(161));
    assert_eq!(history.len(), 0);
}
#[test]
fn constant_spikes_and_irregular_intervals_use_real_time_and_shared_scale() {
    let at = Instant::now();
    let mut history = History::default();
    for (seconds, rx, tx) in [(0, 10.0, 20.0), (2, 10.0, 20.0), (17, 1000.0, 20.0)] {
        history.observe(&reading(at + Duration::from_secs(seconds), rx, tx));
    }
    let points = positions(&history, at + Duration::from_secs(20));
    assert_eq!(points.len(), 3);
    assert_eq!(history.peak(), 1000.0);
    assert!((points[1].0 - points[0].0 - 2.0 / 60.0).abs() < 0.0001);
    assert!((points[2].0 - points[1].0 - 15.0 / 60.0).abs() < 0.0001);
    assert_eq!(points[2].1, 1.0);
    assert_eq!(points[0].2, 0.02);
}
#[test]
fn failures_and_baselines_make_gaps_and_cohort_change_clears_history() {
    let at = Instant::now();
    let mut history = History::default();
    history.observe(&reading(at, 5.0, 1.0));
    history.observe(&Reading {
        at: at + Duration::from_secs(2),
        cohort: vec![],
        status: Status::Failed(sei_windows::NativeError {
            operation: "test",
            code: 5,
        }),
    });
    history.observe(&Reading {
        status: Status::Baseline("reset"),
        ..reading(at + Duration::from_secs(4), 0.0, 0.0)
    });
    assert_eq!(history.len(), 1);
    history.observe(&reading(at + Duration::from_secs(6), 6.0, 1.0));
    assert_eq!(history.len(), 2);
    let mut changed = reading(at + Duration::from_secs(8), 5.0, 5.0);
    changed.cohort[0].luid = 2;
    changed.status = Status::Baseline("new");
    history.observe(&changed);
    assert_eq!(history.len(), 0);
}
#[test]
fn stale_replayed_and_nonfinite_points_cannot_enter_history() {
    let at = Instant::now();
    let mut history = History::default();
    history.observe(&reading(at, 1.0, 1.0));
    history.observe(&reading(at, 999.0, 999.0));
    for (second, value) in [(1, f64::NAN), (2, f64::INFINITY), (3, -1.0)] {
        history.observe(&reading(at + Duration::from_secs(second), value, 1.0));
    }
    assert_eq!(history.len(), 1);
    assert_eq!(history.peak(), 1.0);
    assert!(reading(at, 1.0, 1.0).stale(at + Duration::from_secs(5)));
    assert!(positions(&history, at + Duration::from_secs(61)).is_empty());
}
#[test]
fn formatting_is_decimal_and_rejects_invalid_rates() {
    for (value, expected) in [
        (0.0, "0.0 B/s"),
        (999.0, "999.0 B/s"),
        (1000.0, "1.0 KB/s"),
        (1_500_000.0, "1.5 MB/s"),
        (2e9, "2.0 GB/s"),
    ] {
        assert_eq!(format_rate(value), expected);
    }
    for invalid in [-1.0, f64::NAN, f64::INFINITY] {
        assert_eq!(format_rate(invalid), "Unavailable");
    }
}
