# Sei-Desk

Stage 4D adds persistent pinned favorites, sorting, grouping and list/text-grid presentations to Applications. Search, refresh and explicit launches still use the existing Start menu catalog and Consumer services. The native Rust + iced dashboard retains CPU, physical memory, system-drive capacity, battery, local time, live receive/transmit rates and bounded traffic history. The existing [design package](docs/design/README.md) remains the visual reference. See the [Stage 4D acceptance report](docs/implementation/STAGE-4D-LAUNCHER-USABILITY-ACCEPTANCE.md) for persistence and usability checks, [Stage 4C](docs/implementation/STAGE-4C-APPLICATIONS-ACCEPTANCE.md) for launch integration, [Stage 4B](docs/implementation/STAGE-4B-NETWORK-MONITORING-ACCEPTANCE.md) for network policy, and [Stage 4A](docs/implementation/STAGE-4A-DASHBOARD-ACCEPTANCE.md) for the dashboard baseline. Earlier validation code, services, experiments and evidence are preserved; production accessibility remains gated.

## Build and run

Windows x64, Rust/MSVC and a Windows SDK/linker are required. iced is pinned to stable `0.14.0`; the root lockfile pins its resolved dependencies. Rust 1.88 is the declared minimum inherited from iced; execution here uses 1.99.0 (the minimum compiler has not been separately tested).

```powershell
cargo run --locked -p sei-desk
# Optional dashboard metric diagnostics:
cargo run --locked -p sei-desk -- --diagnostics
# Preserved Stage 2A harness, with optional seeded validation error:
cargo run --locked -p sei-desk -- --validation --diagnostics --error-fixture
```

In the dashboard, Tab/Shift+Tab traverse navigation and Applications controls, Enter/Space activate the focused action, and PgUp/PgDn scroll the page. Applications search matches names and executable filenames; Up/Down selects a result and Enter launches it. Escape clears transient application search/selection state without closing the dashboard. Refresh retains the last good catalog on failure. Native titlebar Close/Alt+F4 exits and joins owned workers; launched applications remain independent. The layout uses two or four metric columns and scrolls at compact sizes; no system modifications are available.

Applications' Show control switches All/Pinned; View, Sort and Group cycle the labeled choices. Up/Down follows reading order in both list and grid, including unavailable saved pins. Pin selected/Unpin selected saves the current application; Remove saved pin forgets an unavailable identity, which cannot launch. Favorites are stored in `%LOCALAPPDATA%\SeiDesk\pins-v1.txt` as versioned identity digests and labels, without launch commands. Changes appear after a successful background save. Configuration errors preserve the prior file and show recovery guidance; unsupported versions are read-only. Presentation, sorting and grouping are session choices. Launch feedback reports process acceptance, which does not establish that a window opened.

In `--validation` mode, Tab/Shift+Tab traverse Sample text, Validate, Prefer reduced motion, Dismiss error when present, and Close. Enter validates text or activates the local focused action; Space activates an action outside the text field. Escape dismisses an inline error first, then closes the window. The motion checkbox is in-memory only and cannot override a system request to reduce motion. Neither window contains animated elements.

The UI has visible local focus and textual feedback. **UI Automation/screen-reader conformance is not established by those features.** Read the [Stage 2A accessibility report](docs/implementation/STAGE-2A-ACCESSIBILITY-REPORT.md) and Stage 4A limitations; production launcher and shell accessibility gates remain closed.

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

The Stage 2A script explicitly launches the retained validation mode; it does not test dashboard accessibility. See its [architecture and boundaries](docs/implementation/STAGE-2A-ARCHITECTURE.md). The Stage 1 experiment stays standalone/excluded with its own Cargo commands and evidence. The dashboard activates read-only monitoring only: no AppBar, global shortcut/hook, tray host or taskbar/Start replacement is introduced.
