//! Bounded native probes, not a Sei-Desk UI or shell replacement.
#[cfg(windows)]
mod native;

fn main() {
    #[cfg(windows)]
    if let Err(error) = native::run() {
        eprintln!("probe_failed: {error}");
        std::process::exit(1);
    }
    #[cfg(not(windows))]
    {
        eprintln!("Windows required; no simulated Windows result is provided.");
        std::process::exit(1);
    }
}
