//! Explicit refresh measurement; emits counts only, no discovered personal paths.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cancel = sei_apps::Cancellation::default();
    let start = std::time::Instant::now();
    let catalog = sei_apps::refresh_async(cancel, None)?
        .join()
        .map_err(|_| "catalog worker panicked")??;
    println!(
        "refresh_ms={:.3} entries={} visited={} issues={} truncated={}",
        start.elapsed().as_secs_f64() * 1000.0,
        catalog.entries.len(),
        catalog.visited,
        catalog.issues.len(),
        catalog.truncated
    );
    let second_start = std::time::Instant::now();
    let second = sei_apps::refresh_async(sei_apps::Cancellation::default(), None)?
        .join()
        .map_err(|_| "second catalog worker panicked")??;
    println!(
        "second_refresh_ms={:.3} entries_identical={}",
        second_start.elapsed().as_secs_f64() * 1000.0,
        catalog.entries == second.entries
    );
    if catalog.entries != second.entries {
        return Err("catalog changed between explicit refreshes".into());
    }
    let mut kinds = std::collections::BTreeMap::new();
    let mut operations = std::collections::BTreeMap::new();
    for issue in catalog.issues {
        *kinds.entry(format!("{:?}", issue.error.kind)).or_insert(0) += 1;
        *operations.entry(issue.error.operation).or_insert(0) += 1;
    }
    println!("issue_kinds={kinds:?}");
    println!("issue_operations={operations:?}");
    // Keep measurement process alive briefly, with no catalog polling/redraws.
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(())
}
