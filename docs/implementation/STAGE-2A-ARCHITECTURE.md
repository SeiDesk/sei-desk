# Stage 2A — minimal iced foundation

14 October 2026. This stage creates one ordinary Windows validation window. It builds on the [Stage 1 report](STAGE-1-WINDOWS-INTEGRATION-REPORT.md), [risk register](STAGE-1-RISK-REGISTER.md), [matrix](STAGE-1-TEST-MATRIX.md) and existing [design package](../design/README.md). Stage 1's uncommitted files and evidence are preserved; its experiment remains standalone. This is not the production desktop shell.

## Decision and workspace

Use stable iced **0.14.0**, confirmed against the [official release list](https://github.com/iced-rs/iced/releases), with a committed-ready root Cargo lockfile. Build/run evidence uses Rust/Cargo 1.99.0, Windows x64/MSVC, Windows NT build 26300. Rust 1.88 is the declared minimum matching iced; that minimum compiler has not been separately tested. Stage 1's proposed Windows 10/11 build families still need qualification; this host is not evidence for every supported release.

```text
Cargo.toml / Cargo.lock                 root workspace, resolver 3
crates/sei-desk/
  src/main.rs                          iced entry, local events/tasks/lifecycle
  src/model.rs                         pure validation state and unit tests
  src/view.rs                          opaque pastel UI and visible focus
crates/sei-windows/
  src/lib.rs                           motion query, guarded focus request
  examples/stage2a_probe.rs             opt-in developer input/client-bounds probe
tools/Test-Stage2A.ps1                  Windows UIA client/process measurement
experiments/windows-integration-spike/  excluded; unchanged standalone Stage 1
```

The proposed two-crate structure was described before creation. There is no separate metrics/domain/service crate: those have no immediate purpose in this window. `sei-desk` has only iced and the local Windows adapter as direct dependencies. The adapter uses Microsoft `windows-sys 0.61.2`; input/GDI features are developer-only dependencies for the example. winit's separate transitive binding version is retained rather than patched unnecessarily.

iced defaults are disabled. `tiny-skia` provides a software renderer without adding wgpu/GPU initialization to this validation stage; this is a baseline choice, not a final renderer performance decision. `crisp` matches the reference's clean edges. `thread-pool` enables iced's task/subscription executor; a 1-worker executor avoids a default pool sized to every CPU. `advanced` exposes iced's IME event types for composition guards, not a new widget/GUI framework. No additional async runtime, telemetry, database, tray host or background service is added. Cross-platform target-support packages may appear in Cargo.lock through iced/winit; they do not introduce a web frontend.

## Minimal window and state flow

`iced::application` owns one decorated, resizable normal-level opaque window (640 × 500 DIP, minimum 560 × 480). The window uses Segoe UI and the existing reference-derived lavender canvas, slate text, mint controls, blush feedback and blue focus border. There is no custom titlebar, topmost mode, desktop reservation or dock. [Design specification](../design/SEI-DESK-UI-UX-SPEC.md) remains intact.

The only functional content is Sample text, Validate, an in-memory Prefer reduced motion checkbox, optional inline error/Dismiss error, and Close. Validation checks non-whitespace text and reports a local status. It launches no applications and performs no system changes. The error fixture is an explicit developer option which seeds that same validation state at boot; it is not evidence that an OS key produced the error.

Local input/event → `Message` → pure `Model::apply` → view/task. Model actions have no iced or Win32 dependency. Closing is terminal: late messages cannot modify/reopen the state. A validation failure retains the text and initiating logical focus. Editing clears stale feedback without moving the caret to the end. Escape clears an error first; a later Escape closes. If Dismiss error removes the focused control, focus returns to Sample text.

Tab order follows the visual layout: Input → Validate → Motion → Dismiss error (only while present) → Close → Input. Shift+Tab reverses it. Enter/Space activates an action; text input owns ordinary typing, caret movement and its single Enter-submit path. Repeated activation and Ctrl/Alt/Windows combinations are ignored by the local shortcut handler. Composition state blocks validation/navigation/Escape-close until composition ends; actual IME behavior remains a manual test. Native Alt+F4 still follows the normal close request.

iced 0.14.x buttons and checkbox do not provide the required built-in focus traversal. This small fixture keeps a logical focus target and draws a 2-DIP outline; iced's focus operation explicitly focuses/unfocuses the actual text field. The operation is never re-issued on ordinary edit/commit because it moves the caret to the end. A generation check rejects stale asynchronous input-focus queries after pointer/traversal changes. Local focus is **not** an accessible UIA focus node; the [accessibility report](STAGE-2A-ACCESSIBILITY-REPORT.md) records that blocker.

## Native boundary and shutdown

The app does not import Win32 functions. `sei-windows` exposes:

- `motion_preference()`: read-only `SPI_GETCLIENTAREAANIMATION`, returning a typed preference/error. Query at startup and on window reactivation, never continuously. Unknown preference conservatively reduces motion. The checkbox can request less motion and cannot override a system request. No element animates in this fixture regardless of the preference.
- `ForegroundBookmark`: capture the prior foreground before creating the window; on explicit close only, re-check current foreground process, target validity and original process/thread association before a fallible `SetForegroundWindow` request. Return ObservedForeground, Requested, Denied or Skipped. No HWND/control-focus guarantee, input attachment, lock-policy change or elevation workaround.

The bookmark owns no foreign window. Same-process/thread HWND reuse remains a limitation of this short-lived validation bookmark; a production launcher would need stronger lifetime tracking. A successful API request is not a promise to restore an arbitrary foreign child control. On focus loss or validation errors, no native foreground request runs. If Windows denies restoration, closing still completes and Windows retains ordinary activation policy. Actual runs on this host reported denial; no successful foreign restoration is claimed.

The titlebar Close, local Close and Escape converge on one application close action. `exit_on_close_request=false` lets that action run before `iced::exit`. iced owns and destroys its window/event loop and drops the executor on return. There are no app-owned hooks, shell registrations, sampling timers, worker services or retry queues to clean. Bootstrap/runtime iced errors go to stderr and a failure exit code; they are distinct from recoverable inline validation feedback. Diagnostics are opt-in and log lifecycle/focus/status only, never typed content or foreign window identity. An own-window raw HWND is emitted only in diagnostics for test-target verification.

The native example is a separate **developer executable**, not linked into the app. It accepts only a fixed list of sample key commands/client-bounds query and requires exact PID, title, HWND and native foreground checks. It never activates a target or sends Windows-key shortcuts, installs no hooks and does no polling. Input injection still has a focus-change race and may be blocked by UIPI/session policy; do not use it while interacting with other windows. A partial Shift sequence attempts a release. It is not a production input API.

## Test procedure and evidence boundaries

See the [root README](../../README.md) for build/run commands and [accessibility report](STAGE-2A-ACCESSIBILITY-REPORT.md) for the manual checklist and evidence decisions. `tools/Test-Stage2A.ps1` uses Windows PowerShell's built-in .NET UI Automation client, process counters and pixel capture. It is developer tooling, not another application frontend. Output directories must be new/empty; previous evidence is not overwritten.

The script starts and retains an exact `System.Diagnostics.Process`, drains output asynchronously, selects a titled UIA Window for that PID, and checks its HWND against iced's diagnostic handle after exit. Do not select `Process.MainWindowHandle` blindly: the first test selected winit's internal event window. Each raw/control tree traversal is bounded to 80 nodes/depth 6. These count limits are not a timeout guarantee for a hung OS/provider call. Run in a healthy test desktop and retain ownership of the child if manually cancelling the script.

It optionally requests fixed test keys only while both UIA and native foreground guards match. An API return count is insufficient: expected state transitions must appear in app diagnostics. This host's native SendInput attempt returned 0, so keyboard runtime validation is blocked, not passed. Error rendering/UIA is independently inspected with a seeded error fixture. Pixel capture uses the guarded native **client** rectangle so shadow/border areas do not copy another app. The early wrong-root/full-frame captures are invalid evidence and explicitly labeled in the evidence index.

After the bounded observation, the script uses the native window's UIA WindowPattern.Close, waits for process exit, captures exit code/output and disposes its process object. If graceful close fails, it force-stops **only its owned child** and reports failure. It never kills Explorer or another user process. Closing UIA test patterns is a native-frame lifecycle test, not proof of accessible client controls. Narrator speech and physical keyboard/IME/mixed-DPI checks remain separate manual tests.

Unit tests cover error/value/focus preservation, correcting input, traversal including conditional error controls, Escape versus close, terminal close state, motion precedence/unknown fallback, composition guards, captured Enter avoiding double-submit, ignored modifier/repeat activation, native ownership gating and error formatting. These are pure state/policy tests, not Windows accessibility conformance or foreground activation successes.

## Resource policy and next gate

No application timer or continuous polling subscription exists. `event::listen_with` filters redraw events rather than turning them into model updates. iced may still schedule caret redraws while its text field has focus; that is renderer/widget behavior and must be included in a focused-input baseline. System motion is event-driven; debug output only changes on events. The executor has one idle-blocking worker. No shared system-information service is created early; when monitoring is authorized, one app-owned sampler should serve all future surfaces as Stage 1 recommends.

The report records actual release process CPU/working-set/private-commit measurements over explicit intervals, including the window/focus/fixture condition. They are short host baselines, not budgets or cross-renderer benchmarks; renderer choice and hidden-window behavior require further profiling. No arbitrary zero-CPU promise is made.

**Disposition:** the ordinary minimal window is the Stage 2A foundation. Essential client accessibility remains blocked by the observed missing UIA tree. Keep full launcher/taskbar/dock/dashboard/optimizer implementation closed. The next smallest remedy is an accessible iced vertical slice using a maintained UIA/AccessKit bridge tied to widget semantic IDs, physical layout, actions/focus and state-change/error events. Prove text/value, invoke, toggle and focus semantics plus Narrator before extending the full UI. Do not adopt a draft fork or another GUI framework silently.

## Git change summary

All Stage 2A work is uncommitted on `main`. Added root workspace/lockfile/ignore/run instructions, two crates (including the isolated developer example), one PowerShell test procedure, the two Stage 2A reports and a separate `evidence-stage2a/` directory. No tracked design/reference file changed. The 29 pre-existing uncommitted Stage 1 documents, evidence and experiment files were checked against their pre-edit SHA256s: zero alterations. No Stage 1 file is staged, overwritten or included in a commit. See [preservation evidence](evidence-stage2a/preservation.json).
