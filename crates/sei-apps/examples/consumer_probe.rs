//! Explicit read-only Windows consumer baseline. Never selects/launches an app.
use sei_apps::consumer::{Consumer, Status};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut consumer = Consumer::new();
    println!("consumer_inline_bytes={}", std::mem::size_of::<Consumer>());
    let mut first = None;
    for number in 1..=2 {
        let start = std::time::Instant::now();
        let id = consumer.request_refresh()?;
        consumer.start_pending()?;
        consumer.wait()?;
        let Status::Succeeded { value, .. } = consumer.refresh_status() else {
            return Err(format!("refresh failed: {:?}", consumer.refresh_status()).into());
        };
        let snapshot = consumer.snapshot().ok_or("missing snapshot")?;
        println!(
            "refresh={number} generation={} ms={:.3} entries={} visited={} issues={} truncated={}",
            id.get(),
            start.elapsed().as_secs_f64() * 1000.0,
            value.entries,
            snapshot.catalog.visited,
            value.issues,
            value.truncated
        );
        if let Some(first) = &first {
            let same = first == &snapshot.catalog.entries;
            println!("entries_identical={same}");
            if !same {
                return Err("catalog changed between explicit refreshes".into());
            }
        } else {
            first = Some(snapshot.catalog.entries.clone());
        }
    }
    assert!(consumer.selection().is_none());
    assert!(matches!(consumer.launch_status(), Status::Idle));
    // Measurement tool only: keep cached data alive, with no consumer polling.
    std::thread::sleep(std::time::Duration::from_secs(2));
    consumer.shutdown();
    println!("shutdown_closed={}", consumer.is_closed());
    Ok(())
}
