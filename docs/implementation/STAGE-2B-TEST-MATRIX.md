# Stage 2B — test matrix and reproducible Windows procedure

14 October 2026. **Gate: blocked, not passed.** No runtime bridge is installed; these are baseline regression results and pending acceptance tests. Read the [report](STAGE-2B-ACCESSIBILITY-REPORT.md) and [evidence index](evidence-stage2b/README.md). Executed host: Windows NT 26300 x64/MSVC, Rust/Cargo 1.99.0. Windows 10/11 target release qualification is not inferred from this host.

## Recorded status

| ID | Test | Status | Evidence / remaining requirement |
| --- | --- | --- | --- |
| R01 | Released provider/integration research | Research only | AccessKit 0.25.1 / Windows 0.35.1 / winit adapter 0.34.1 researched; no maintained released iced 0.14 semantic adapter found; no candidate compiled |
| Q01 | Formatting | Passed after sandbox exception | Initial sandbox Access denied retained; retry `cargo fmt --all --check` exit 0 |
| Q02 | Compilation | Passed | `cargo check --workspace --locked --offline`, exit 0; cache hard-link warnings retained |
| Q03 | Existing state/error/policy tests | Passed | 9 app + 2 Windows adapter tests; no new provider tests |
| Q04 | Strict Clippy | Passed | Workspace, all targets, `-D warnings`, exit 0 |
| Q05 | Release builds | Passed | Existing app and developer example build, exit 0 |
| U01 | Correct process/window selection, raw and control inspection | Passed | Own PID/title/type; root HWND independently confirmed by iced diagnostics in normal/error runs |
| U02 | Sample input Name/Edit role/value/focus/label relationship | Failed | Actual client tree has no input element or descendants |
| U03 | Validate Name/Button/enabled/Invoke | Failed | No client Validate node |
| U04 | Motion Name/CheckBox/enabled/checked/Toggle | Failed | No client CheckBox node |
| U05 | Error text and conditional Dismiss node | Failed | Seeded error rendered; neither feedback nor Dismiss exposed in raw/control views |
| U06 | Read/set input value through UIA | Blocked | No input Value/Text provider to target; not executed as an action |
| U07 | UIA Validate/Dismiss Invoke and checkbox Toggle | Blocked | No client action patterns; native WindowPattern.Close does not substitute |
| U08 | Error event notification without focus relocation | Blocked | No semantic error/provider integration; no UIA event listener or speech pass |
| U09 | Disappearing node/stale action rejection | Blocked | No Dismiss accessible node existed; disappearance/lifetime correctness cannot be claimed |
| U10 | Caret/selection ranges | Blocked | No Text/Selection provider; released iced operation lacks a current selection getter; no support claimed |
| K01 | Existing optional synthetic keyboard sequence | Blocked | Fixed native SendInput probe result recorded separately; failed injection is not a physical-keyboard result |
| K02 | Physical Tab/Shift+Tab/Enter/Space/Escape | Not run | Interactive human validation unavailable in this session |
| K03 | Physical typing, edit-in-middle, IME/selection | Not run | Unit/source checks do not validate real widget input |
| N01 | Narrator names/roles/value/actions/error speech | Not run | Essential gate test; graphical/speech workflow unavailable, installation alone proves nothing |
| L01 | Normal native-window close/process exit | Passed | WindowPattern.Close; graceful app exit 0/no owned child in normal/error runs |
| L02 | Guarded external focus restoration | Denied, not passed | Recorded restoration outcome; Windows remains responsible for activation policy |
| L03 | Close followed by stale UIA client/control actions | Blocked | No client provider; cannot prove lifetime handling for a future bridge |
| L04 | External application focus switch and return | Not run | Requires an interactive desktop sequence |
| D01 | Non-default DPI, movement, resize, hit-test bounds | Not run | No semantic bounds exist to compare; mixed DPI remains unqualified |
| D02 | High contrast and prepared reduced-motion environment | Not run | Motion policy unit tests pass; no OS setting changed or contrast conformance claimed |
| P01 | Idle app CPU/memory | Measured, no acceptance threshold | Per-run release process counters over 30 s; not provider overhead because no bridge installed |

Q01–Q05 outputs: [quality-gates.txt](evidence-stage2b/quality-gates.txt). U/L/P evidence: [normal](evidence-stage2b/normal/run.json), [error](evidence-stage2b/error/run.json), [keyboard](evidence-stage2b/keyboard/run.json). Each directory contains independent raw/control trees, own-client screenshot where captured, lifecycle trace and process result. Script success is not semantic or Narrator success.

## Re-run the unchanged Stage 2A checks

From repository root, ordinary user desktop. Preserve existing output: each destination must be new/empty. Use a new suffix for subsequent runs. Close each test app before the next run; do not run keyboard/focus tests concurrently.

```powershell
cargo fmt --all --check
cargo check --workspace --locked --offline
cargo test --workspace --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo build --release --locked --offline -p sei-desk
cargo build --release --locked --offline -p sei-windows --example stage2a_probe
powershell.exe -NoProfile -File tools/Test-Stage2A.ps1 -IdleSeconds 30 -OutputDirectory docs/implementation/evidence-stage2b/repeat-normal
powershell.exe -NoProfile -File tools/Test-Stage2A.ps1 -ErrorFixture -IdleSeconds 30 -OutputDirectory docs/implementation/evidence-stage2b/repeat-error
# Optional injection: stops on a failed foreground/input guard. No elevation retry.
powershell.exe -NoProfile -File tools/Test-Stage2A.ps1 -Keyboard -IdleSeconds 30 -OutputDirectory docs/implementation/evidence-stage2b/repeat-keyboard
git diff --check
```

Offline requires cached dependencies. The build-tool filesystem exception used here is separate from administrator privileges; do not RunAs/elevate the GUI or keyboard fixture to turn a blocked test into a pass.

## UI Automation acceptance procedure after an integration is selected

This procedure describes tests to run against the **actual validation window**. On the current unmodified app, stop at missing client nodes and record failure. Do not attempt equivalent mouse clicks and report them as UIA operations. Use built-in Windows PowerShell 5.1 `UIAutomationClient`/`UIAutomationTypes` or SDK Inspect UI Automation mode. Record versions, PID, actual HWND, binary hash and environment. Use a dedicated test desktop; provider calls can hang, and tree node-count limits are not API timeouts.

1. Run `cargo run --locked -p sei-desk -- --diagnostics` from one terminal. Read its own `validation_hwnd` and identify its PID. In Inspect, select the visible titled window; confirm PID and native HWND. Do not use `Process.MainWindowHandle` alone: winit may return its internal event window.
2. Inspect both Raw View and Control View. Record stable identities, parent/child order, Name, ControlType, IsEnabled, IsKeyboardFocusable, HasKeyboardFocus, supported patterns and BoundingRectangle. Input must be Edit named Sample text with the correct editable value/label relation; Validate/Dismiss/Close must be Buttons; Motion must be a named CheckBox. Compare rectangles with actual client pixels at current scale.
3. In a separate PowerShell client, identify the confirmed root with `AutomationElement.FromHandle`. The following is a **fail-closed input check**, not an executed success in this stage. Enter the HWND from the running diagnostic, never an unrelated window:

```powershell
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
$validationHandle = [IntPtr]([long](Read-Host 'Own validation_hwnd from diagnostics'))
$root = [System.Windows.Automation.AutomationElement]::FromHandle($validationHandle)
if ($root.Current.Name -notmatch '^Sei-Desk . Stage 2A validation$') { throw 'Wrong window' }
$scope = [System.Windows.Automation.TreeScope]::Descendants
$condition = [System.Windows.Automation.PropertyCondition]::new(
    [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
    [System.Windows.Automation.ControlType]::Edit)
$sampleControl = $root.FindFirst($scope, $condition)
if ($null -eq $sampleControl -or $sampleControl.Current.Name -ne 'Sample text') {
    throw 'Missing correctly named accessible input; stop and record failure'
}
if ($sampleControl.Current.ProcessId -ne $root.Current.ProcessId) { throw 'Wrong process' }
$value = $sampleControl.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)
if ($value.Current.IsReadOnly) { throw 'Expected editable value' }
$value.SetValue('sample')
# Bounded test-client wait for an asynchronous UI-thread update; no production polling.
$deadline = [DateTime]::UtcNow.AddSeconds(2)
while ($value.Current.Value -ne 'sample' -and [DateTime]::UtcNow -lt $deadline) {
    Start-Sleep -Milliseconds 50
}
if ($value.Current.Value -ne 'sample') { throw 'Value not synchronized' }
```

4. Discover buttons using **both Name and Button control type** within the same root and verify exact PID, enabled state and InvokePattern before invoking. Set input empty through ValuePattern, invoke Validate; require real inline error, exposed feedback, stable initiating focus and an independently captured error event. Then find/invoke Dismiss error. Re-query raw/control views to prove removal; invoke the retained stale element and require a safe failure/no unintended action. Use a bounded client wait for asynchronous UI updates, never a production polling loop. No nodes/patterns means Blocked, not a fallback action pass.
5. Discover Motion using Name and CheckBox role. Capture TogglePattern.ToggleState, call Toggle once, then verify both the model-rendered checkbox and semantic state changed. Toggle back; verify it cannot override Windows' reduced-motion request. Confirm focus behavior does not change unexpectedly for an external automation action.
6. If TextPattern/TextPattern2/selection is advertised, compare ranges, caret, Unicode/IME text and screen rectangles with actual edit-in-middle behavior. Test selecting/replacing text. If a pattern is unavailable, record it as unsupported; do not infer it from ValuePattern or the ability to set a string. Assess essential Narrator editing separately.
7. Move/resize at 100% and at least one prepared non-default DPI setting; compare hit tests and bounds after each layout change. Repeat with the error inserted/removed and after monitor movement. No new semantic/control nodes should remain at old positions.
8. Focus another application, query/operate only supported UIA actions on this validation window, then return. Record native foreground and semantic focus separately; the provider must not steal foreground merely to answer a query. Follow the existing guarded close policy.
9. Close normally. Re-query the old root and retained input/action nodes; require element unavailable or equivalent safe disconnection. Repeat open/close and in-flight action cases; verify no deadlock, crash, duplicate provider, remaining process or input/settings side effect. Current baseline's process exit alone does not prove future provider cleanup.

## Physical keyboard and Narrator acceptance

Use the [Stage 2A physical procedure](STAGE-2A-ACCESSIBILITY-REPORT.md#reproducible-manual-accessibility-procedure) on an actual interactive Windows desktop. Tab/Shift+Tab must follow visual order with visible focus; type/select/edit in the middle; exercise keyboard Validate, error dismissal, Motion and Close. Repeat after switching applications. Error announcement must preserve focus; removed Dismiss must not retain keyboard or accessibility focus.

With Windows Narrator, record spoken names, roles, checked/editable/focus state and value while navigating **and operating** the essential controls. Record error speech separately from UIA event observation. Test scan-mode and ordinary edit navigation as applicable. A static tree or narrated window title is insufficient. Test high contrast/reduced motion and non-default DPI in a prepared environment if available, without changing persistent settings from the harness. Mark unavailable scenarios Not run/Blocked with the reason; failed synthetic input proves neither physical failure nor success.

Stage 2B may pass only after required semantic actions, non-disruptive error behavior, disappearance/cleanup, regression checks and actual Narrator operation pass. These acceptance tests remain open; no production UI milestone is authorized by this matrix.
