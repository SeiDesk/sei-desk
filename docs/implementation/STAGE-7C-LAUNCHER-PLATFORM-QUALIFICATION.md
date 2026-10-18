# Stage 7C — Launcher Platform Qualification

2026-10-18 (Asia/Calcutta). **Bounded qualification pass completed; platform qualification remains partial.** No reproducible defect requiring a source change was established. This stage adds only this report to the nonignored working tree. Production accessibility remains **BLOCKED**. Passing scoped rows below does not establish release readiness or pass unavailable hardware/manual scenarios.

## Baseline and environment

Repository: `X:\Project\SeiDesk\sei-desk`. Starting and final branch: `main`. Starting and final HEAD: `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`.

Read the Stage 7A, 7B and 7B.1 reports and relevant launcher, desktop routing, native placement, service/runtime, keyboard and lifecycle tests. Source and earlier reports distinguish conceptual APIs, controlled tests and live observations; this report retains those distinctions.

Before tests, saved the complete starting `git status --short` and inventory/hashes for all 739 existing tracked/nonignored files under ignored `target/stage7c-baseline/status.txt` and `hashes.json`. The baseline contains all Stage 5–7B.1 modifications, untracked code, reports and evidence shown in the final status below, except this new report. All 739 original files remained byte-identical. No prior report, mockup, source, manifest, lockfile, script or evidence was edited. No earlier ignored evidence was deleted. New ignored logs/profile/snapshots are separately named Stage 7C artifacts.

Environment facts:

- OS query returned `Microsoft Windows NT 10.0.26300.0`.
- Existing release executable used: `target/release/sei-desk.exe`, SHA-256 `3A6268B33FDD4C4188C608A57D18FF39C4E62BF580E7BB9A0FD7EE3DC5906FB2`. It was built in Stage 7B.1; source is unchanged in this stage.
- All live Sei-Desk work used a new, initially empty, process-local `LOCALAPPDATA=target/stage7c-profile`. Real preferences were untouched.
- Safe unrelated focus peer: a newly opened installed CMake GUI 4.3.4, with no project loaded. Its normal exterior capture was 614×674; maximized capture 1920×1080. These are captured-window dimensions, not proof of monitor work area, primary-display identity, scale or topology. Restored to its normal size before closing; no Configure/Generate or application setting action performed.
- Read-only `WmiMonitorConnectionParams` and `Win32_VideoController` inventory queries both failed: “Access to a CIM resource was not available to the client.” No permission/system change or administrator attempt followed. Display count, topology, physical scale, primary-display identity and safe reconnection setup remain unverified.
- UI operations used Computer Use/Sky automation. These are live Windows observations driven by injected input, **not physical keyboard evidence**. No human-operator qualification was performed.

## Evidence index

All live screenshots below are under `C:\Users\swani\.codex\visualizations\2026\10\17\01a14960-e87d-70c1-8db4-20816aebcadc` (the existing writable artifact root; captures are from this Stage 7C run).

| ID | Evidence |
| --- | --- |
| L1 | `stage7c-open.jpg`: actual opaque 580×440 popup with search focus and empty-favorites state. Manager HWND 264532; initial popup 6817876. Named manager control opened it; reinvocation retained 6817876. |
| L2 | Window inventories: Escape removed 6817876; reopening created 1313260; visible Dismiss removed that popup. Subsequent explicit invocations created 15337772, 2820248 and finally 788910 during the bounded checks. |
| L3 | `stage7c-foreign-focus.jpg`: CMake GUI 17107212 remained visible with its focused source field after explicit activation and a five-second settling interval while the companion existed. `stage7c-maximized-peer.jpg`: same harmless peer maximized; no Sei-Desk overlay appeared in its captured content. No native foreground trace was installed. |
| L4 | `stage7c-manager-resized.jpg` records the initial 1102×772 manager capture after an attempted pointer resize that did not change its observed dimensions. The subsequent native Size command produced `stage7c-manager-size-command.jpg`, 1201×849. `stage7c-popup-after-resize.jpg` records a fresh 580×440 popup after that actual size change. A titlebar move was attempted; absolute movement was not measured. |
| L5 | Minimize manager with native system menu while 2820248 is open; wait five seconds; inventory contains manager only. Restore manager: `stage7c-minimize-recovery.jpg` shows “Launcher: Open the launcher from the active management window.” Explicit invocation recovered popup 788910. |
| L6 | Close manager 264532 with popup 788910 open. Process returned exit code 0; ignored `target/stage7c-runtime.log` ends with `dashboard_workers_joined`. Final inventory contains neither Sei-Desk window nor the test CMake window. |
| T1 | `desktop::tests`, 9 passed: reuse, old-window messages, manager isolation/destruction, stale service feedback, focus-generation changes, unconfirmed-focus/invalid-placement failure, geometry-query ticket race, actual companion widget Enter policy and query-error recovery. |
| T2 | `applications::runtime::tests`, 11 passed: bounded queues/latest-state mailbox, pending launch/refresh closure, stale completion rejection, cancellation, worker failure/shutdown, identity capture and explicit Retry. |
| T3 | `sei-windows` launcher tests, 2 passed: negative origins/scales and invalid/undersized geometry are pure tests, not hardware observations. |
| T4 | Existing `sei-apps` suite with opt-in native fixtures, 50 passed: 47 unit tests plus 3 integration tests. Includes real CreateProcess invalid-PE failure and test-owned missing/stale/locked target cases; installed application files/permissions are untouched. |
| T5 | `actual_search_widget_respects_modified_and_repeated_enter`, 1 passed. Together with T1 actual companion widget path, plain/Ctrl/Alt/Logo/repeated Enter submit counts are `[1,0,0,0,0]`. Synthetic widget/event evidence only. |
| S1 | `crates/sei-desk/src/desktop.rs`: window routing/lifetime, explicit opening and focus requests, pending deadlines, actual focus observation generations, periodic and move/rescale geometry checks, close/exit policy. |
| S2 | `crates/sei-windows/src/launcher.rs:74–162`: live owned-HWND validation, monitor/work-area/DPI queries, minimized-manager rejection, stale rectangle validation, `SWP_NOACTIVATE | SWP_NOZORDER`, no shell writes. |
| S3 | `crates/sei-desk/src/main.rs:630–637`: event-loop return drops monitor, applications and pins owners before diagnostic join marker. Runtime/consumer ownership tests verify their joins. Existing ForegroundBookmark restoration is referenced by `validation.rs`, not the production companion path. |

## Complete qualification matrix

Statuses are **PASS**, **FAIL**, **BLOCKED**, or **NOT TESTED** for the stated procedure/evidence class only. A pure/model test PASS does not pass the corresponding live hardware case. No row is upgraded from skipped/unavailable evidence.

### A. Window lifecycle and focus

| Scenario | Prerequisites | Procedure | Expected result | Actual result / evidence | Status |
| --- | --- | --- | --- | --- | --- |
| A1 Named manager invocation | Running foreground manager, isolated profile | Click existing Open launcher command | One focusable popup | 6817876 opened; actual view/search focus observed, L1 | PASS — live automated |
| A2 Reinvoke | Popup already shown | Deliberately return to manager and use local Ctrl+L | Reuse, no duplicate | Same 6817876 remained; T1 also checks outstanding callbacks, L1/L2 | PASS — live automated + test |
| A3 Escape dismissal | Shown popup | Send Escape to popup | Only popup closes | 6817876 disappeared; manager remained, L2 | PASS — live automated |
| A4 Visible Dismiss | Reopened popup | Activate named Dismiss control | Only popup closes | 1313260 disappeared; manager remained, L2 | PASS — live automated |
| A5 Manager close with popup | Recovered popup 788910 | Close manager through normal window command | Exit, both windows gone, owners join | Exit 0, no owned window, join marker, L6/S3/T1 | PASS — live automated + tests |
| A6 Manager/popup/foreign focus changes | Newly opened harmless CMake GUI | Activate peer while popup exists; settle; return deliberately through manager | No forced repeated focus or stale foreign restoration | CMake remained usable/visible; ordinary manager/popup routing recovered, L3. No authoritative continuous foreground trace | PASS — limited live observation |
| A7 No repeated focus-forcing path | Current source/tests | Inspect gain-focus sites and timeout/reuse branches; rerun focus-generation test | Only deliberate requests, bounded timeout, no restoration loop | S1/S3, T1; no production bookmark restoration call | PASS — source/automated only |
| A8 Actual Windows focus denial | Safe OS-denial setup or authorized human procedure | Deny one real request and observe recovery | Manager usable; no retries | No controlled OS-denial setup used. T1 injects unconfirmed observation only | NOT TESTED — live denial |

### B. Display geometry and DPI

| Scenario | Prerequisites | Procedure | Expected result | Actual result / evidence | Status |
| --- | --- | --- | --- | --- | --- |
| B1 Ordinary display opening | Available interactive desktop | Open/reopen and inspect actual popup | Usable unclipped popup | 580×440 view observed before/after resize, L1/L4 | PASS — limited live layout |
| B2 Primary monitor / work-area containment | Verified monitor identity, rectangle and scale | Compare actual bounds to invocation monitor work area | Fully contained on intended monitor | CIM inventory denied; native numeric monitor/bounds telemetry not captured. Window-only screenshots do not prove this | NOT TESTED — live numeric containment |
| B3 Negative-coordinate arrangement | Verified second display and safe existing layout | Invoke on negative-origin monitor | Correct monitor-local containment | Required hardware/topology not established; T3 is pure negative-coordinate coverage only | NOT TESTED — hardware |
| B4 Move manager and reopen | Measurable window movement | Titlebar move then invoke | Same intended monitor, safe placement | Move attempted; absolute position not measured. Invocation continued, but movement/containment not qualified | NOT TESTED — absolute movement |
| B5 Resize manager and reopen | Ordinary manager window controls | Native Size command, then invoke | Manager remains usable; popup valid | Manager exterior changed 1102×772 → 1201×849; fresh popup 580×440, L4 | PASS — live automated resize |
| B6 Change display scale | Known original scale, reversible authorized setup | Change scale, invoke, restore | Correct recomputation and containment | No scale change performed; original scale/safe setup unavailable | NOT TESTED |
| B7 Remove/reconnect display | Safe reversible hardware setup | Disconnect/reconnect and invoke | No stale placement or stranded popup | No connection/topology change performed | NOT TESTED |
| B8 Invalid/undersized geometry | Existing pure tests | Run launcher geometry tests | Return error, never an unsafe placement | 2 tests passed, T3 | PASS — pure tests only |
| B9 Stale geometry callback | Existing reducer regression | Deliver old query reply after reuse/newer inspection | Old callback ignored | T1 geometry-ticket regression passed | PASS — controlled model only |
| B10 Manager minimize/recovery | Open companion and ordinary manager controls | Minimize, wait, restore, explicitly reopen | Popup fails closed, manager usable | Popup gone, banner on restore, recovery 788910, L5 | PASS — live automated |
| B11 Display-change implementation boundary | Current source | Inspect move/rescale and freshness paths | Bounded fresh geometry checks, fail closed | S1/S2: one automatic outstanding check, exact tickets, fresh work-area validation, no-activate placement | PASS — source inspection only |

### C. Fullscreen and taskbar coexistence

| Scenario | Prerequisites | Procedure | Expected result | Actual result / evidence | Status |
| --- | --- | --- | --- | --- | --- |
| C1 Ordinary maximized peer | Newly opened harmless CMake GUI | Maximize peer while companion exists; observe | No unsolicited topmost popup/focus forcing | Peer maximized normally; captured peer content unobstructed; restored to 614×674 and closed, L3 | PASS — limited live observation |
| C2 Exclusive fullscreen | Suitable harmless app and reversible setup | Enter/exit exclusive fullscreen | No unsolicited overlay or input interception | No suitable app/setup established or exercised | NOT TESTED |
| C3 Borderless fullscreen | Suitable harmless app with known mode | Enter/exit actual borderless fullscreen | Safe ordinary-window coexistence | Maximized CMake is not evidence of borderless fullscreen | NOT TESTED |
| C4 Native taskbar enabled/functional | Direct taskbar interaction/state evidence | Observe taskbar; activate a taskbar item without changing settings | Native taskbar remains functional | Connector supplied target-window captures, not an authoritative taskbar state/click result. No taskbar action/settings change performed | NOT TESTED — functional taskbar gate |
| C5 No AppBar/taskbar/Explorer/global-hotkey integration | Production launcher source | Trace actual companion calls; search shell/input entry points | No such production companion operations | S1/S2: ordinary Level::Normal; typed handle, only owned-window placement. No AppBar, Explorer manipulation, taskbar replacement or RegisterHotKey/hook/tray route in companion | PASS — source scope check |

### D. Failure and recovery

| Scenario | Prerequisites | Procedure | Expected result | Actual result / evidence | Status |
| --- | --- | --- | --- | --- | --- |
| D1 Dismiss during pending launch | Existing gated service fixture | Hold accepted work, dismiss, reopen, release | One launch; old completion cannot close replacement | T2 pending-launch regression passed | PASS — controlled service/reducer |
| D2 Dismiss during pending refresh | Existing gated refresh fixture | Hold refresh, dismiss/reopen, release failure | No old error in manager/replacement | T2 failed-refresh replacement regression passed | PASS — controlled service/reducer |
| D3 Stale window/focus/placement callbacks | Existing ID/generation tests | Deliver old messages to replacement | No mutation/replacement dismissal | T1 old-window, focus-generation and geometry-ticket tests passed | PASS — controlled model |
| D4 Ordinary Windows launch failure | Test-owned executable fixtures | Run invalid PE, removed/stale/locked target fixtures | Structured rejection; no wrong target or installed-app modification | T4: real native failures passed; exact invalid-PE check accepts OS 193/216 without recording which branch this machine returned | PASS — actual Windows fixture operations |
| D5 Failure UI and explicit retry | Controlled launcher/service fixture | Fail selected launch, retain state, attempt ordinary Enter, explicit/stale Retry | Explicit matching Retry only; no implicit duplicate | T2 companion retry regression passed. No live UI launch failure injected into installed apps | PASS — controlled service/reducer |
| D6 Native pending-work UI dismissal | Safe slow native operation with UI | Dismiss live popup while actual native launch/refresh pending | No stale window effect | Not forced against installed apps; deterministic coverage is D1/D2 only | NOT TESTED — live pending native case |
| D7 Clean shutdown / worker termination | Running manager + popup; existing owner tests | Normal manager close, inspect return/log; rerun worker lifecycle cases | No resident UI or detached owned workers | L6/T1/T2/T4/S3. Process exit 0 and join marker | PASS — live + automated |
| D8 Focus denial/stale work area/display-change handling review | Current source/tests | Inspect live handle/PID/minimized/work-area checks and deadlines | Errors fail closed, no stale HWND cast or focus loop | S1/S2/T1/T3; real manager-minimize case L5 also passed | PASS — source/test boundary; not hardware denial/removal |

### E. Accessibility and input limitations

| Scenario | Prerequisites | Procedure | Expected result | Actual result / evidence | Status |
| --- | --- | --- | --- | --- | --- |
| E1 Modified/repeated Enter | Existing laid-out widgets and outer routing tests | Run manager/companion event-path tests | One plain submission; zero modified/repeat submissions | T1/T5, `[1,0,0,0,0]` | PASS — automated only |
| E2 Stale identities/state | Existing consumer/runtime/desktop fixtures | Rerun captured identity/generation, busy, close/replacement tests | No different target or obsolete callback effect | T1/T2/T4 passed | PASS — automated/native fixtures |
| E3 Physical keyboard | Authorized operator and physical input | Repeat ordinary/repeat/modifier workflows manually | Same bounded policy with real input | No operator test performed; Sky input is synthetic | NOT TESTED |
| E4 Narrator/UIA / production accessibility | Applicable semantic/provider and authorized human/platform gates | Complete actual qualification evidence | All required gates pass before production claim | Existing Stage 5E architecture decision and failed source preflight remain operative; Stage 7C adds no provider or semantic qualification | BLOCKED |
| E5 Real IME / high contrast / mixed-DPI input | Real operator, supported IME/hardware and safe setup | Execute applicable manual scenarios | Correct composition/focus/layout | Not performed; no state change to manufacture a test | NOT TESTED |
| E6 Explorer restart / crash recovery | Separate safe authorized procedure | Exercise actual restart/crash recovery | Safe restoration | Explicitly not attempted; no arbitrary kill or Explorer manipulation | NOT TESTED |

## Commands and exact outcomes

All Cargo commands used existing locked/offline dependencies with `CARGO_INCREMENTAL=0`.

| Command | Result |
| --- | --- |
| `git branch --show-current`; `git rev-parse HEAD`; `git status --short` | Recorded starting/final state; unchanged branch/HEAD |
| `git ls-files --cached --others --exclude-standard` + SHA-256 inventory | 739 baseline paths; all 739 unchanged at completion |
| `cargo test -p sei-desk --locked --offline desktop::tests` | 9 passed, 0 failed |
| `cargo test -p sei-desk --locked --offline companion` | 6 passed, 0 failed; overlaps T1/T2 |
| `cargo test -p sei-windows --locked --offline launcher::tests` | 2 passed, 0 failed |
| `cargo test -p sei-desk --locked --offline applications::runtime::tests` | 11 passed, 0 failed |
| `cargo test -p sei-desk --locked --offline actual_search_widget` | 1 passed, 0 failed |
| Set `SEI_APPS_FIXTURE` to absolute `target/release/examples/launch_fixture.exe`; `cargo test -p sei-apps --locked --offline -- --include-ignored` | 50 passed, 0 failed, 0 ignored; only controlled fixture targets. Run with ordinary filesystem access, no UAC/admin privilege |
| `cargo fmt --all --check` | Passed; no formatting writes |
| `Get-CimInstance -Namespace root/wmi -ClassName WmiMonitorConnectionParams`; `Get-CimInstance Win32_VideoController` | Both returned CIM-resource access errors; no display facts inferred |
| New isolated LOCALAPPDATA, `target/release/sei-desk.exe --diagnostics` | Actual UI run; normal exit 0, join marker |
| `git diff --check` | Passed; existing LF/CRLF notices only |

The test groups overlap; they are not additive independent qualification counts. No source change was made, so workspace check/full workspace tests/Clippy/release rebuild were not rerun merely to repeat Stage 7B.1. Its 136-test broad validation is historical and is not reported as a new Stage 7C run. No new test or renderer capture was added; existing tests supplied focused evidence. No dependency resolution, installation, system setting change or administrator privilege was used.

## Findings and next step

No concrete reproducible defect was established within the tested scope; there is no source correction or new regression test to report. The initial pointer resize did not change observed size and is not claimed as a passing resize; the subsequent native Size-command result supplies that row's actual evidence.

Outstanding risks are the NOT TESTED/BLOCKED rows, especially primary/work-area numeric containment, negative-coordinate and mixed-DPI topology changes, real OS focus denial, exclusive/borderless fullscreen, taskbar functional interaction, physical input and production accessibility. Source checks/pure tests do not resolve those hardware/platform risks. Arrange a known safe display/taskbar/fullscreen test setup and authorized human input checks for owner review; do not implement a dock, global input feature or architecture experiment as a substitute. Production accessibility remains **BLOCKED**.

## Exact changed path and final Git status

Only new nonignored path: `docs/implementation/STAGE-7C-LAUNCHER-PLATFORM-QUALIFICATION.md`. All source, manifests, lockfiles, earlier reports, approved mockups, untracked evidence and pre-existing fixes are unchanged. Baseline complete short status is the following list with only the Stage 7C report entry omitted; the original exact listing is also preserved in `target/stage7c-baseline/status.txt`.

Final complete `git status --short`:

```text
 M README.md
 M crates/sei-desk/src/applications.rs
 M crates/sei-desk/src/applications/preferences.rs
 M crates/sei-desk/src/applications/preferences/tests.rs
 M crates/sei-desk/src/applications/runtime.rs
 M crates/sei-desk/src/applications/runtime/tests.rs
 M crates/sei-desk/src/applications/tests.rs
 M crates/sei-desk/src/main.rs
 M crates/sei-desk/src/traffic_chart.rs
 M crates/sei-desk/src/view.rs
 M crates/sei-windows/src/lib.rs
 M tools/Test-Stage2A.ps1
?? crates/sei-desk/src/applications/search.rs
?? crates/sei-desk/src/desktop.rs
?? crates/sei-desk/src/desktop/
?? crates/sei-desk/src/launcher.rs
?? crates/sei-desk/src/launcher/
?? crates/sei-windows/src/launcher.rs
?? docs/implementation/STAGE-5A-STABILIZATION-ACCEPTANCE.md
?? docs/implementation/STAGE-5B-ACCESSIBILITY-ARCHITECTURE-DECISION.md
?? docs/implementation/STAGE-5C-OWNER-DECISION-PACKET.md
?? docs/implementation/STAGE-5D-ACCESSIBILITY-CANDIDATE-PREFLIGHT.md
?? docs/implementation/STAGE-5E-UPSTREAM-FIRST-OWNER-DECISION.md
?? docs/implementation/STAGE-6A-DASHBOARD-RELIABILITY-REPORT.md
?? docs/implementation/STAGE-6B-LAUNCHER-ACCEPTANCE.md
?? docs/implementation/STAGE-6C-VISUAL-REFINEMENT-REPORT.md
?? docs/implementation/STAGE-6D-END-TO-END-ACCEPTANCE.md
?? docs/implementation/STAGE-7A-DESKTOP-FEASIBILITY.md
?? docs/implementation/STAGE-7B-LAUNCHER-IMPLEMENTATION.md
?? docs/implementation/STAGE-7B1-LAUNCHER-UX-ACCEPTANCE.md
?? docs/implementation/STAGE-7C-LAUNCHER-PLATFORM-QUALIFICATION.md
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

No stage, commit or push. Stopped after the bounded qualification report; no Stage 7D or excluded implementation began.
