//! Test-owned child only. Writes received arguments/cwd to a test-owned file,
//! then exits. Never used as a catalog entry outside explicit test fixtures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let output = args.next().ok_or("test output path required")?;
    let rest: Vec<_> = args.collect();
    std::fs::write(output, format!("{rest:?}\n{:?}", std::env::current_dir()?))?;
    Ok(())
}
