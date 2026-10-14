# Sei-Desk

Stage 2A adds one small Windows validation window using Rust + iced. It does not implement the planned desktop shell. The existing [design package](docs/design/README.md) remains the product visual reference. The uncommitted [Stage 1 investigation](docs/implementation/STAGE-1-WINDOWS-INTEGRATION-REPORT.md) and isolated experiment are preserved.

## Build and run

Windows x64, Rust/MSVC and a Windows SDK/linker are required. iced is pinned to stable `0.14.0`; the root lockfile pins its resolved dependencies. Rust 1.88 is the declared minimum inherited from iced; execution here uses 1.99.0 (the minimum compiler has not been separately tested).

```powershell
cargo run --locked -p sei-desk
# Optional local event diagnostics; no typed values or raw key logging:
cargo run --locked -p sei-desk -- --diagnostics
# Seed an inline validation error for accessibility inspection without injected keys:
cargo run --locked -p sei-desk -- --error-fixture
```

Use Tab/Shift+Tab to traverse Sample text, Validate, Prefer reduced motion, Dismiss error when present, and Close. Enter validates text or activates the local focused action; Space activates an action outside the text field. Escape dismisses an inline error first, then closes the window. The native titlebar Close/Alt+F4 also exits the process. The motion checkbox is in-memory only and cannot override a system request to reduce motion. The window contains no animated elements.

The UI has visible local focus and textual feedback. **UI Automation/screen-reader conformance is not established by those features.** Read the [Stage 2A accessibility report](docs/implementation/STAGE-2A-ACCESSIBILITY-REPORT.md) for actual observations and the blocking disposition before building the full UI.

## Tests

```powershell
cargo fmt --all --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo build --release --locked -p sei-desk
cargo build --release --locked -p sei-windows --example stage2a_probe

# Uses Windows' UI Automation client, native process counters and bounded cleanup.
# Use Windows PowerShell 5.1 (UIAutomationClient is included); output must be new/empty.
powershell.exe -NoProfile -File tools/Test-Stage2A.ps1 -IdleSeconds 30 -OutputDirectory docs/implementation/evidence-stage2a/manual-baseline

# Optional: inject fixed sample keys only while the owned test app has focus.
# Stop interacting with other windows during this short sequence. It stops on focus loss.
powershell.exe -NoProfile -File tools/Test-Stage2A.ps1 -Keyboard -IdleSeconds 30 -OutputDirectory docs/implementation/evidence-stage2a/manual-keyboard

# Error rendering/UIA inspection; seeded state is not a keyboard-delivery test:
powershell.exe -NoProfile -File tools/Test-Stage2A.ps1 -ErrorFixture -IdleSeconds 30 -OutputDirectory docs/implementation/evidence-stage2a/manual-error
```

`--offline` can be added to Cargo commands when dependencies are cached. The developer PowerShell script is not application GUI code or an alternative frontend. It reports missing UIA controls separately from process lifecycle success, does not certify Narrator speech, and never modifies system settings. The exact child is closed/waited in `finally`; forced-stop fallback is reported as a graceful-exit failure. Do not interpret an exit-0 test script as accessibility conformance.

See [architecture and boundaries](docs/implementation/STAGE-2A-ARCHITECTURE.md), including the remaining manual Windows tests. The Stage 1 experiment stays standalone/excluded and retains its own Cargo commands and evidence. No AppBar, global shortcut/hook, tray host, taskbar replacement, metrics service or production module is activated by this application.
