# Stage 4A — Dashboard acceptance

17 October 2026. Implemented in the existing Rust + iced workspace. Starting branch: `main`; HEAD: `f9f334bfb65bb57a6afc2047d47baad4738df52e`; initial `git status --short` was empty. No commit or push.

## Delivered

The default executable now opens a real iced 0.14 dashboard with native Windows chrome, the approved pale blue-gray management surfaces, slate text, lavender/pink/mint accents, an Overview/Applications rail, and visible local keyboard focus. Four resource cards switch between four columns and two columns at 1180 DIP. Initial client size is 1100 × 740; minimum is 760 × 520. Content scrolls while the rail and clock remain fixed. Tab/Shift+Tab traverse navigation, Enter/Space open the focused page, and PgUp/PgDn scroll. Navigation returns the content to its top.

CPU, physical memory, system-drive capacity/free space, battery and local time are real readings. Loading, first CPU interval, individual native errors, absent/unknown battery, and five-second paused-data states are explicit. Capacity and CPU bars use actual readings; no fabricated chart/history or health verdict is present. Network throughput and disk activity are explicitly unavailable/unmeasured.

Applications is the permitted functional navigation placeholder, with an explanation and working return action. It does not enumerate, select or launch applications. Stage 3A/3B services and their contracts are unchanged. The existing validation harness and all nine of its tests are retained; run it with `--validation` (plus its existing diagnostic/error-fixture options).

Run the dashboard:

```powershell
cargo run -p sei-desk --release --locked --offline
# Existing validation harness:
cargo run -p sei-desk --release --locked --offline -- --validation --diagnostics --error-fixture
```

## Files and architecture

| File | Change |
| --- | --- |
| `crates/sei-desk/src/main.rs` | Dashboard state, subscriptions, native window, local navigation/scrolling, validation-mode dispatch. |
| `crates/sei-desk/src/view.rs` | Reference-derived responsive cards, rail, clock, power section, placeholder and states. |
| `crates/sei-desk/src/monitor.rs` | Fixed sampler/freshness workers, bounded stream bridge, wakeable stop and owned joins; lifecycle test. |
| `crates/sei-desk/src/validation.rs` | Preserved old entry point, with explicit module paths, callable entry and reusable executor visibility. |
| `crates/sei-desk/src/validation_view.rs` | Preserved original validation view. Original `model.rs` is unchanged. |
| `crates/sei-windows/src/lib.rs` | Export the new read-only metrics module. |
| `crates/sei-windows/src/metrics.rs` | Narrow native snapshots, counter/capacity validation, battery decoding, four tests. |
| This report | Actual implementation, verification and remaining boundaries. |
| `README.md` | Current default dashboard instructions and explicit legacy validation route. |
| `tools/Test-Stage2A.ps1` | Pass `--validation` so the existing observer continues to target its original harness. |

No manifest, dependency feature, lockfile, approved mockup, service source, historical report or experiment was changed. No reset, clean, stash or deletion was used. All pre-existing tracked work was preserved; the initial tree had no untracked files.

One sampler thread performs synchronous Windows queries off the UI thread every two seconds, without overlapping scans. One independent freshness thread sends a local clock/pause pulse every second. The shared async channel is bounded (configured capacity three plus the two fixed sender reservations); backpressure drops an update rather than accumulating work or creating workers. A stable subscription identity preserves the single receiver across views. UI updates only consume owned values. The existing single-thread iced executor is reused.

The owner lives outside iced, signals both workers on exit and joins them after the event loop returns. Sleeping workers wake immediately. Native calls in progress must finish before joining; no hard deadline or forced thread termination is claimed. Sampling continues while minimized. No external process, shell command, COM enumeration or application launch is used for dashboard readings.

| Reading | Source and meaning |
| --- | --- |
| CPU | `GetSystemTimes`; differences of kernel/user/idle counters between valid samples. Kernel includes idle. First sample and invalid/reset intervals have no percentage. On hosts above 64 logical processors, this is the calling processor group's aggregate, explicitly labeled in the card. |
| Memory | `GlobalMemoryStatusEx`; physical total minus available; decimal GB and percent. |
| Disk | `GetDiskFreeSpaceExW` on the local drive prefix of `SystemRoot`; caller-visible total/available (quota-aware), decimal GB. Capacity is distinct from activity. Unexpected system-root formats fail explicitly. |
| Battery | `GetSystemPowerStatus`; optional charge percentage, charging/AC source, explicit absent and unknown cases. |
| Clock | `GetLocalTime`, refreshed independently every second. Windows local timezone; fixed 24-hour/ISO date formatting for now. |

Small `repr(C)` Kernel32 declarations are justified because the accepted windows-sys feature set does not expose all required queries and dependency changes are prohibited. Existing enabled bindings are reused for CPU/FILETIME/error codes. All unsafe calls are isolated in the Windows module, operate on initialized SDK-layout buffers with synchronous lifetimes, and carry safety comments. Layout tests check 64/12/16-byte structures. This is not a provider fork or a new native shell integration. Sources: [GetSystemTimes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemtimes), [SYSTEM_POWER_STATUS](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-system_power_status).

## Commands actually executed

Windows x64/MSVC; Rust/Cargo 1.99.0. Final gates used process-local `CARGO_INCREMENTAL=0` because this workspace drive reports unsupported incremental hard links; no warnings were suppressed in source.

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Final pass. |
| `cargo check --workspace --all-targets --locked --offline` | Pass. |
| `cargo test --workspace --locked --offline` | Final pass: 62 passed, 5 intentionally ignored native fixture tests. Includes 45 ordinary service tests, 11 app tests and 6 Windows-library tests. |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Final pass. |
| `cargo build --workspace --release --locked --offline` | Final pass. |
| `cargo build -p sei-apps --release --examples --locked --offline` | Pass. |
| `cargo test -p sei-apps --locked --offline -- --include-ignored` with `SEI_APPS_FIXTURE` set to this checkout's release `launch_fixture.exe` | Outside filesystem sandbox: 47 library + 1 consumer integration + 2 launch integration = 50 passed; no ignored tests. These overlap the ordinary tests, not 50 additional unique tests. |
| `git diff --check` | Pass. |
| `target/release/sei-desk.exe --diagnostics` | Actual Windows window opened, readings updated for several minutes, navigation/resizing/scrolling worked; native close exited 0 with `dashboard_workers_joined`. |

Initial development failures were diagnosed rather than hidden: the progress bar uses iced 0.14 `girth`, not `height`; a view style closure initially borrowed App beyond its lifetime; strict Clippy required the safety comment directly above GetLastError; a post-edit formatting check required another fmt. All were corrected before the final gates. The first enabled service run under the filesystem sandbox failed the existing shortcut rename at native.rs:376 (Windows error 5), with 46 other library tests passing. The identical unmodified service suite passed outside that sandbox, matching the Stage 3B documented environment restriction. No test was weakened. No service regression was found.

## Runtime observations and limitations

The exact workspace release executable was selected by its full path, separate from an installed application also named Sei-Desk. The launch helper initially targeted that installed application; its web UI and floating dock were excluded from dashboard evidence. The repository implementation remains Rust + iced only. Workspace captures live under the chat's visualization directory, outside the repository; runtime diagnostic output from the final session is under ignored `target/stage4a-runtime.log`.

Observed CPU percentages changed between two-second samples; physical memory was about 25.5 GB total, system-drive capacity about 262.4 GB, and the battery reported 76% plugged in on this host. These are observations, not fixtures. The actual battery-present path and native calls ran on Windows. Counter reset/zero intervals, invalid capacities, absent/unknown battery and stale-state behavior are unit-tested; actual permission denial, battery removal, a hung driver and prolonged recovery were not induced.

Visually checked normal 1100-DIP and compact 760-DIP client widths, a 760 × 520 client window with scrolling, and maximized four-column layout. Both mouse scrolling and final PgDown reached lower readings; Tab/Enter opened the Applications placeholder. Native titlebar close joined workers. The initial scrollbar overlapped card edges; spacing was fixed and the final window rechecked. These are functional observations, not exhaustive performance/leak qualification.

Not performed: Narrator/UIA acceptance, full 100–200% or mixed-DPI matrix, high contrast, multiple Windows builds, >64-CPU hardware, locale formatting preferences, long-duration resource/leak measurements or forced/native-hang shutdown qualification. The legacy Stage 2A PowerShell observer was updated to select validation mode and syntax-checked, but its full UI Automation suite was not rerun. UI Automation exposes native window chrome but not the iced content semantics here. Local keyboard focus and passing headless tests do not establish assistive-technology acceptance.

Network requires a read-only adapter/counter implementation with interface identity, reset/wrap/disconnect handling and timed deltas before throughput can be shown. Disk busy/transfer counters and real history are also deferred. No chart dependency was added.

Catalog integration remains precise future work: obtain a separately approved dependency on the existing sei-apps crate; own Consumer on an adapter thread; add bounded command/completion wakeups; perform start_pending/drain_ready/shutdown joins off the UI thread; carry operation IDs and snapshot generations to UI; retain cancellation, explicit selection, stale-result checks and accepted launch results; then implement loading/empty/error/selection/launch states after the accessibility disposition. Nothing here reimplements or relaxes the accepted services.

Native taskbar/Start replacement, AppBar reservation, global hotkeys/Windows-key interception/input hooks, Explorer recovery, privileged modifications and optimization actions remain absent and gated. Native Windows desktop controls are untouched by this implementation.

The independent visual finish review returned **ship for bounded Stage 4A**, with no material visual blocker in the supplied final normal/compact/keyboard-scroll captures. Its read-only documenter pass confirmed the incumbent palette and noted that the root README needed its new default/validation instructions; those instructions and the legacy observer's one-line argument were updated. Approved design assets remain unchanged. This review does not waive accessibility or certify untested dynamic/native failure states.

## Final Git status

HEAD and branch remain as recorded above. `git status --short`:

```text
 M README.md
 M crates/sei-desk/src/main.rs
 M crates/sei-desk/src/view.rs
 M crates/sei-windows/src/lib.rs
 M tools/Test-Stage2A.ps1
?? crates/sei-desk/src/monitor.rs
?? crates/sei-desk/src/validation.rs
?? crates/sei-desk/src/validation_view.rs
?? crates/sei-windows/src/metrics.rs
?? docs/implementation/STAGE-4A-DASHBOARD-ACCEPTANCE.md
```
