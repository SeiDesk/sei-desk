# Stage 1 — test matrix

Date: 14 October 2026. Actual host/environment and interpretation are in the [report](STAGE-1-WINDOWS-INTEGRATION-REPORT.md). Status values are **Passed**, **Failed**, **Blocked**, **Not run**, **Research only**. A narrow probe pass does not validate the entire corresponding product feature. [Evidence index](evidence/README.md).

## Executed and researched checks

| ID | Check and expectation | Status | Actual result / evidence |
| --- | --- | --- | --- |
| E01 | Clean design-only baseline, branch/history and toolchain | Passed | `main` at `0400d17`; no Cargo/application code initially; Windows/x64 tooling recorded in [environment](evidence/environment.txt) |
| E02 | PMv2, monitors/work areas, native taskbar and app-window query calls | Passed | One monitor; API-returned candidate counts and minimized/maximized observations only; [snapshot](evidence/snapshot.txt) |
| E03 | Own non-activating window minimize/restore | Passed | Both IsIconic assertions true, foreground unchanged, cleanup succeeded; [window](evidence/window.txt). Foreign activation not covered |
| E04 | Normal bounded observer lifetime removes owned resources | Passed | Three WinEvent removals/window/class/console handler succeeded; work area matched; [observe](evidence/observe.txt) |
| E05 | Fixture's WM_CLOSE flows through WM_QUIT and cleanup | Passed | WM_QUIT observed, cleanup succeeded; [close](evidence/close.txt). This is a real native message test, not a physical key test |
| E06 | Deliberate panic unwinds owner guards | Passed | Expected exit 101 and successful cleanup; [panic](evidence/panic.txt). Abort/forced cleanup not implied |
| E07 | Force-stop exact observer child; process and its hidden HWND disappear | Passed | Parent retained exact process object, killed/waited it; subsequent snapshot found no spike HWND; [forced observer](evidence/forced-observer.txt). No AppBar/tray/hotkey owned |
| E08 | Force-exit automatically unhooks WinEvents | Research only | Microsoft documents client-thread lifetime cleanup; E07 did not independently instrument hook removal |
| E09 | AppBar safety admission refuses unsuitable desktop state | Passed | Busy state 2 caused refusal; receiver cleaned; [appbar](evidence/appbar.txt) |
| E10 | AppBar registration/reservation/removal round trip | Blocked | E09 prevented ABM_NEW; no reservation was created. Normal coexistence and recovery remain open |
| E11 | Ctrl+Alt+Q RegisterHotKey returns success | Failed | Error 5/access denied at ordinary rights; [hotkey](evidence/hotkey.txt). Cause not established, no elevation retry |
| E12 | Registered shortcut delivery/conflicts/unregister | Blocked | E11 prevented registration; no key injection or Windows-key suppression |
| E13 | Own notification icon NIM_ADD returns success | Failed | BOOL false; [tray](evidence/tray.txt); no trustworthy extra error code claimed |
| E14 | Version-4 tray/menu/keyboard callbacks/delete | Blocked | E13 prevented ownership; fixture only counts callbacks, no menu implementation |
| E15 | Raw-mouse INPUTSINK registration/removal | Passed | Both returned success; [raw](evidence/raw.txt). No raw events or edge entry observed in idle sample |
| E16 | Coarse timer samples physical cursor and cleans up | Passed | 479 reads/30.054 s and work-area match; [poll](evidence/poll.txt). Crossing/latency not tested |
| E17 | System client-area animation preference query | Passed | Query succeeded, false/disabled; no system setting changed; snapshot logs |
| E18 | Negative-origin, one-pixel and narrow-width math | Passed | Two Rust unit tests. Pure synthetic geometry inputs, not actual display/DPI proof |
| E19 | Short native CPU and memory measurement path | Passed | Native process counters/monotonic timing returned recorded values; report explains resolution and scope; no performance acceptance threshold exists |
| E20 | Bare Win tap replacement without breaking shell shortcuts | Research only | API/hook constraints researched; no keyboard hook or suppression implemented |
| E21 | Released iced 0.14.0 accessibility integration | Research only | Local cached source/manifests audited; upstream open issue/draft PR; [source audit](evidence/source-audit.txt). No Narrator/UIA execution |
| E22 | Explorer recreation / live re-registration | Not run | Explorer untouched; synthetic TaskbarCreated is not counted as real restart evidence |
| E23 | cargo check / release build / two geometry tests | Passed | Windows/MSVC, offline locked dependencies; [quality gates](evidence/quality-gates.txt) |
| E24 | cargo fmt --check / strict Clippy | Passed | Filesystem sandbox initially denied canonicalization; checks passed through build-tool-only sandbox exception; no runtime probe elevated |
| E25 | Pre-fix 30-second raw probe ends after one deadline period | Failed | Observed 60.009 s; cleanup succeeded; preserved [failed duration log](evidence/raw-pre-fix-lifetime.txt). Implementation corrected, not relabeled as passing |
| E26 | Rebuilt deadline behavior, mode/exit-path regression | Passed | Parent stopwatch: observe/raw/poll 1 s and 2 s; normal close, expected panic 101, successful cleanup. [Regression log](evidence/lifetime-regression.txt). Repeated raw 30 finished at 30.020 s / 30.122 s parent wall time in [raw.txt](evidence/raw.txt) |

## Qualification environments

| Environment | Required baseline | Status |
| --- | --- | --- |
| Windows 10 22H2 x64 | `10.0.19045.*`, exact current revision/servicing eligibility recorded | Not run |
| Windows 11 24H2 x64 | `10.0.26100.*`, exact revision/edition recorded | Not run |
| Windows 11 25H2 x64 | `10.0.26200.*`, exact revision/edition recorded | Not run |
| Current restricted Windows x64 host | NT 26300, one 96-DPI monitor; not a support target | Passed for the narrow E-series checks marked Passed only |
| Multi-monitor physical test station | Negative and stacked origins; mixed 100/125/150/175/200% DPI, portrait display | Not run |

## Reproducible manual validation still required

Use ordinary rights and the harness [build/run instructions](../../experiments/windows-integration-spike/README.md). Run one probe at a time, save output/exit code and before/after monitor/native taskbar state. Durations are 1–30 seconds; repeat bounded runs rather than leave a listener running. Do not change persistent settings, hide native taskbar, trigger power actions or restart Explorer from the harness. Future-feature tests below cannot pass using the current fixture alone; the required missing fixture/integration is stated explicitly.

| ID | Scenario / steps | Expected evidence and present limitation | Status |
| --- | --- | --- | --- |
| M01 | On each target build, take `snapshot`; run `appbar 5` on a normal desktop accepted by admission; compare during/after work rectangles and native taskbar | Temporary identifiable bottom window; additional space above native taskbar; no native policy change; ABM_REMOVE and baseline restoration. Do not bypass busy-state admission | Blocked on current host |
| M02 | Repeat M01 with native taskbar on supported configurations and another independently installed AppBar | Approved non-overlapping rectangles; correct bottom reservation and no taskbar hiding. No settings are changed by this probe | Not run |
| M03 | During bounded AppBar probe, physically change topology/resolution in a prepared lab; inspect cleanup | Current probe should cancel safely. Production relayout is not implemented and must be separately proven later | Not run |
| M04 | In observer `30`, press Ctrl+C / Ctrl+Break; repeat close via test-window Close button | WM_QUIT/owner-thread cleanup, no leftover receiver or subscriptions. Console-close/logoff termination is not equivalent to this path | Not run |
| M05 | In a disposable VM with external snapshot recovery, use a future guarded AppBar/tray fixture for abort/force-exit; capture exact PID/HWND and native work area | Destructor must be absent; external shell resources recover without manual desktop edits. Do not force-kill a registered AppBar on the user's desktop; until a reliable recovery plan exists, this test stays blocked | Blocked |
| M06 | Observe a naturally occurring or separately authorized externally managed Explorer restart while future AppBar/tray fixture is alive | Actual shell disappearance/recreation, TaskbarCreated receipt, bounded idempotent re-registration and usable native taskbar. Current harness has no recovery implementation; never kill Explorer or fake a restart by broadcast in this stage | Blocked |
| M07 | From normal desktop run `hotkey 30`, press Ctrl+Alt+Q, repeat while another utility owns it; repeat with AltGr layouts and assistive tools | One counter event per deliberate invocation, honest registration conflict and successful unregister/re-register. No focus action is implemented. Current registration denied | Blocked on current host |
| M08 | Review bare Win L/R tap/repeat/chords with a future separately consented, bounded nonsuppressing observation fixture first | Preserve native Win+L/D/E, native Start and games; no suppression evidence from this stage. A suppressing fixture requires a separate recovery/consent decision | Research only |
| M09 | Future user-click activation fixture: select another app, then minimized, owned/modal, packaged, elevated and hung targets; switch away before restoration | Correct target/generation, denied requests reported, no focus theft or AttachThreadInput tricks. Current own-window test cannot establish this | Not run |
| M10 | Browser F11, borderless and exclusive game, presentation, maximized app; alt-tab with future non-activating overlay fixture | Ordinary maximized behavior distinguished from suppression; no stale/queued overlay on return; native shortcuts preserved. Geometry snapshot alone cannot pass | Not run |
| M11 | Negative/stacked/portrait monitors and mixed DPI; move launcher across monitors, change primary, unplug target, reconnect | Physical coordinates correct, suggested DPI sizing applied, stale sensor disabled, fallback to valid primary; no cross-monitor animation spill. Current single-monitor snapshot cannot pass | Not run |
| M12 | Future own tray icon/menu fixture: keyboard select/right click, Escape, Enter, normal exit; inspect overflow after exit | Version-4 callbacks, menu focus restoration and no stale icon; native third-party tray remains accessible. Current fixture has no menu | Not run |
| M13 | `raw 30` and `poll 30`, physically enter/leave one-pixel center strip repeatedly; move across stacked boundary, fast diagonals and pointer warp | Count known physical crossings; compare missed events and latency with a recorded input/observation method. No events must be swallowed; polling misses are a valid failure, not a reason to enlarge the strip | Not run |
| M14 | Future dock fixture: hold mouse button/window drag across edge; slide outside panel; foreground full-screen and maximized tabs | No hidden-dock reveal during external drag/full-screen; owned slider capture protected; no focus theft. Current sensor counts only, not dock behavior | Not run |
| M15 | Tiny iced 0.14.x semantic slice with Narrator and Inspect/Accessibility Insights: input, results, taskbar group/overflow, slider, menu, error and confirmation | Names/roles/patterns/values/selection/focus and polite errors; hidden controls excluded; focus Cancel; external app switch does not dismiss/apply confirmation. No iced slice/provider exists yet | Blocked pending accessibility integration |
| M16 | Future iced slice with prepared reduced-motion/high-contrast environments; keyboard only, IME, Sticky Keys and screen reader | No positional motion when system/app prefers reduced motion; dismissal timing unchanged; legible focus/opaque fallback; no invisible-edge dependency | Not run |
| M17 | Three 60-second idle/active profiles per release build, one bounded run at a time, various mouse report rates and screen reader attached | Native counters plus optional ETW scheduling/peak memory; record settings/hardware and remove temporary traces. Compare integration versus renderer overhead; no fabricated CPU budget | Not run |
| M18 | Future event bridge flood/close test: foreground/name/location bursts while UI stops or queue fills | Bounded queue, coalescing/resync, no callback blocking or HWND use after destruction; no worker survives quit | Not run |
| M19 | Lock, UAC and session changes in disposable prepared environment with future surface fixture | Suppress ordinary overlays and sensors; never display above secure desktop; no queued replay or stale topmost/focus state | Not run |

No test is marked Passed on the basis of a mock or source-level API availability. Native WinEvent forced-exit cleanup is documented separately from observed HWND disappearance. The build tool exception was not used to retry denied runtime APIs.
