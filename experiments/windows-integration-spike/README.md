# Isolated Windows integration spike

This standalone Rust binary is a disposable native Win32 harness, **not the Sei-Desk application**. It has no iced dependency and implements no production taskbar, launcher, dock, dashboard or optimizer. Existing designs remain in `docs/design/`. Read the [feasibility report](../../docs/implementation/STAGE-1-WINDOWS-INTEGRATION-REPORT.md) and [matrix](../../docs/implementation/STAGE-1-TEST-MATRIX.md) before interpreting results.

## Build

Windows x64 with Rust/MSVC and Windows SDK/linker. Recorded toolchain: Rust/Cargo 1.99.0. Rust 2024 edition; dependencies pinned/locked to Microsoft `windows-sys 0.61.2` and `windows-link 0.2.1`. No other GUI/runtime dependency. Build on Windows; non-Windows execution explicitly fails rather than supplies fake Windows results.

From the repository root:

```powershell
cargo fmt --check --manifest-path experiments/windows-integration-spike/Cargo.toml
cargo check --locked --manifest-path experiments/windows-integration-spike/Cargo.toml
cargo test --locked --manifest-path experiments/windows-integration-spike/Cargo.toml
cargo clippy --locked --manifest-path experiments/windows-integration-spike/Cargo.toml --all-targets -- -D warnings
cargo build --release --locked --manifest-path experiments/windows-integration-spike/Cargo.toml
```

Add `--offline` when the two dependencies are cached, as they were on this host. The `target/` directory is ignored. There is no root Cargo workspace or production scaffold.

## Run one bounded probe

Use an ordinary user terminal. Do not elevate a rejected probe to conceal a deployment limitation. Each invocation accepts one mode and an optional **1–30 second** observation duration (default 5); invalid modes/durations fail before registration. Example:

```powershell
./experiments/windows-integration-spike/target/release/sei-windows-integration-spike.exe snapshot
./experiments/windows-integration-spike/target/release/sei-windows-integration-spike.exe observe 10
```

| Mode | Purpose / expected result | Resources and cleanup |
| --- | --- | --- |
| `snapshot` | Read monitor/work rectangles, native taskbar presence, candidate window counts, foreground geometry, animation preference, own token elevation; no titles/paths | No HWND/hook/registration created |
| `observe` | Bounded foreground/minimize WinEvent registration and aggregate delivery counts; no polling | Unshown top-level HWND, timer, three out-of-context WinEvent hooks, console cancellation handler; explicit same-thread cleanup |
| `poll` | Compare nominal 50 ms physical-cursor sampling against event-only baseline; count one-pixel strip entries | Same as observe, cursor queries on timer; no overlay or input interception; cancellations invalidate geometry |
| `raw` | Mouse-only INPUTSINK registration, count WM_INPUT and sample current cursor on receipt | Same as observe plus raw mouse registration; fixed header read/default message cleanup; `RIDEV_REMOVE` at end; no keyboard/NOLEGACY |
| `hotkey` | Temporary Ctrl+Alt+Q registration; count delivery only, never show/activate anything | Owned HWND and registered non-Windows shortcut; unregister before HWND destruction; failure reported |
| `tray` | Temporary owned native icon ADD/version-4/DELETE round trip | Borrowed shared system icon, owner HWND, tray identity; delete before owner destruction. Callback count only: no context menu implemented |
| `window` | Show, minimize and restore only the identifiable owned fixture without activation | Tool window with a clear test title; ordinary Z-order, never topmost; destroy after timer |
| `appbar` | Guarded **additional** bottom reservation, approximately 58 DIP, native taskbar untouched; log approved rectangle and baseline recovery | Requires visible native taskbar and notification state accepting interaction; register/query/set; normal remove before window destruction; cancel on later shell/display changes instead of implementing production relayout |
| `close` | Native fixture posts WM_CLOSE to itself; verify WM_QUIT and normal cleanup | Observer resources; no fake Windows mock |
| `panic` | Intentional panic after bounded observer run; **expected exit 101** and successful Drop cleanup | Observer only, no reservation/tray/hotkey; unwind proof only, not abort/crash cleanup |

The test window class/title identify it as `SeiDesk.Stage1.BoundedProbe` / `SEI-DESK STAGE 1 TEMPORARY TEST`. Close its window or press Ctrl+C/Break in its console for graceful cancellation. Ctrl+C handler posts quit to the owner thread; it does not destroy a window from the handler thread. The physical Ctrl+C path still needs manual validation. Console-close/logoff/forced termination may bypass Rust destructors.

The duration bounds the observation loop using both a timer and a monotonic deadline under event traffic. It is **not a hard timeout for synchronous setup/cleanup calls into an unresponsive Explorer or a stuck OS**. Run AppBar only on a controlled healthy desktop. This is a limitation, not a successful crash-recovery guarantee. No fixture starts a worker process, installs a service/hook/startup task or changes native taskbar/system settings. Raw input ownership is process-wide per device class: do not embed this mode in an iced application without coordinating registrations.

## Cleanup and safe interpretation

Normal return and Rust unwind call the same owner-thread cleanup: remove AppBar/icon/hotkey/raw input/WinEvents, kill timer, destroy window, unregister class and console handler. Successful registrations are recorded immediately; partial setup failures unwind already-owned resources. Explicit cleanup results are logged; normal-run cleanup failures return a nonzero status. After normal runs all monitor/work rectangles are compared with the baseline; unexpected changes fail the probe and require inspection rather than a system-setting “repair.” A concurrent external display change can legitimately invalidate that comparison.

`Drop` **does not run** on `TerminateProcess`, `panic=abort`, access violation or power loss. Windows documents automatic WinEvent removal when the client thread ends; this does not prove AppBar work-area/tray recovery. The Stage 1 forced-stop experiment held only observer resources and targeted an exact child process object. **Do not force-kill an AppBar or tray test on the user's desktop.** Those tests need a disposable VM, known external recovery and further authorization if Explorer would be intentionally restarted. Never terminate Explorer, hide its taskbar or call SPI_SETWORKAREA to “fix” a test.

No keyboard hook, Windows-key suppression, input injection or third-party tray hosting exists in this source. Snapshot full-monitor geometry/notification busy state is a heuristic observation, not a full-screen oracle. The raw/poll sensor does not implement production drag/full-screen eligibility or reveal a dock; no edge entry in an idle run proves nothing about reliable crossing detection. No accessibility or iced behavior is validated by this native fixture.

Two unit tests cover **pure** physical-coordinate geometry (negative origin, exactly one pixel, narrow width), not real monitor or input behavior. CPU is a native process-time delta over the loop; memory is an endpoint working set/private-commit reading. Message counts are not hardware wakeups or latency measurements. Failed/blocked results and manual follow-ups are retained in the matrix; do not turn them into passing product claims.
