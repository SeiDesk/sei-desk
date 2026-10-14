# Stage 2A — Windows accessibility and baseline report

Date: 14 October 2026. Read with [architecture](STAGE-2A-ARCHITECTURE.md), [run/test instructions](../../README.md) and [evidence index](evidence-stage2a/README.md). Existing Stage 1 documents/evidence and product designs are not replaced.

## Disposition

The minimal Rust + iced window builds and runs on the available Windows host. **Client accessibility is blocked:** the correct titled iced HWND exposes a native UIA Window with Window/Transform patterns, but no client descendants in either raw or control view. The text field, Validate action, reduced-motion checkbox and inline error/Dismiss action have no observed UIA names, roles, values, states or actions. Local focus rings and unit-tested keyboard policy do not remedy that missing semantic tree.

Do not begin the production launcher/taskbar/dock/dashboard/optimizer UI on the assumption that stable iced provides these semantics. Keep the Rust + iced choice; the next gate is a small maintained UIA/AccessKit integration for this existing validation window, not a new frontend or an unreviewed draft fork. A screen-reader usable client slice must pass before essential product UI expands. This is an explicit accessibility decision, not a conformance claim.

## Environment and method

Actual Windows NT `10.0.26300.0`, x64/MSVC, Rust/Cargo `1.99.0`, 16 visible logical processors. One primary 1920 × 1080 monitor is recorded in the Stage 2 environment snapshot. Runtime tests used ordinary inherited rights within the restricted session; no UAC/RunAs request or runtime elevation retry occurred. Stage 1's proposed 19045/26100/26200 qualification families, mixed DPI and the declared Rust 1.88 minimum have not been separately tested here.

Selected GUI: stable iced **0.14.0**, resolved iced_winit **0.14.1**, iced_widget **0.14.2**, winit **0.30.13**, tiny-skia software renderer. The exact root lockfile and per-run binary SHA256s are retained. Official research: [iced release list](https://github.com/iced-rs/iced/releases), [open accessibility issue #552](https://github.com/iced-rs/iced/issues/552), [draft AccessKit PR #3111](https://github.com/iced-rs/iced/pull/3111). Those references motivate testing; the runtime result below is the actual evidence for this client.

Windows PowerShell 5.1 loaded the built-in `UIAutomationClient` and `UIAutomationTypes` assemblies. The test selects the exact child PID, Window control type and expected title; its HWND is confirmed against `iced::window::run`'s diagnostic raw handle. Raw/control views are traversed separately with 80-node/depth-6 bounds. These are real Windows UIA queries, not mock widgets, HTML accessibility trees or inferred support from API declarations. [Microsoft UIA tree overview](https://learn.microsoft.com/en-us/dotnet/framework/ui-automation/ui-automation-tree-overview).

Windows SDK Inspect x64 (10.0.26100.0) and Narrator are installed. Their graphical/speech workflows were **not executed**. Native computer interaction/audio evaluation was unavailable in this tool session. The programmatic UIA client was available and used; its result must not be mislabeled as a Narrator pass. UIA error-event notifications and screen-reader speech were not independently monitored.

## Executed findings

| Requirement/check | Status | Observed evidence / limit |
| --- | --- | --- |
| Application opens and paints | Passed on this host | Correct titled client window, actual pastel pixels captured; ordinary decorated window with no desktop customization |
| Actual UIA root selection | Passed in corrected runs | Own PID/title/type and iced HWND agree. Native Window class, enabled=true, HasKeyboardFocus=true; Window/Transform patterns only |
| Client text input exposure | Failed | 0 Edit elements and no Value/Text pattern or label association in client raw/control views |
| Client action names/roles/states | Failed | 0 Validate buttons, 0 CheckBox elements, no Invoke/Toggle or client focus nodes. Native WindowPattern.Close is not the client's Close button |
| Inline error accessibility | Missing implementation; speech not run | Seeded error is visibly rendered, but error text and Dismiss action do not appear in UIA. No live-region/notification provider is implemented; no claim that an event listener or Narrator heard an error |
| Visible local focus | Passed for initial/seeded state only | Client screenshot shows Input outline in normal mode, Validate outline in seeded error mode. Actual Tab focus-ring transitions were not delivered by the automated input fixture |
| Keyboard traversal/error state policy | Passed as unit tests | Pure model tests cover forward/reverse/conditional controls, error retention, dismissal and terminal close. This is not physical Windows keyboard validation |
| Automated Windows key delivery to this window | Blocked | Guarded developer SendInput probe returned `0`, expected `2`, on its first Tab pair; remaining commands stopped. UIPI/session cause is not established. No higher-privilege retry |
| Physical Tab/Shift+Tab/Enter/Space/Escape | Not run | Requires an interactive human keyboard run; native injection failure is not evidence that physical input is broken or working |
| Text caret/selection after edit | Unit/source check only | Editing no longer re-focuses the input (iced focus moves caret to end). Fixed sample/caret injection did not run successfully; manual edit-in-middle still required |
| IME/repeat/modifier policy | Passed as pure policy tests; manual not run | Composition guards, captured Enter de-duplication and ignored Ctrl/Alt/Windows/repeated activation checked; actual IME/Sticky Keys/AltGr not validated |
| Normal window shutdown | Passed | UIA WindowPattern.Close reached app close action; diagnostics show graceful exit; corrected process fixture obtained exit 0 and no remaining owned child |
| Native focus restoration | Denied on this host | Guarded `SetForegroundWindow` returned denial; app still closed normally. No successful foreign-window/control restoration is claimed |
| Restoration ownership/identity policy | Passed as unit test | No restore when foreground ownership, validity or process/thread identity fails. HWND reuse and real external focus-switch scenarios still need manual checks |
| Reduced-motion preference | Partially validated | Native query and visible status indicate Windows requests reduced motion; model OR/unknown fallback tests pass. No animations exist; OS preference was never toggled |
| High contrast, DPI, narration, minimized/occluded lifecycle | Not run | Manual target-release desktop checks remain required |

The UIA field is an iced focus ID, not an AutomationId or accessible name. Drawing text beside it does not automatically create a UIA label relation. Likewise, a local button focus outline cannot expose keyboard focus/selection to assistive technology without provider nodes. The outer window title may be available to a screen reader; that is insufficient to operate the custom client UI.

### Invalid and blocked test-fixture runs retained

The [evidence index](evidence-stage2a/README.md) labels each run explicitly:

- Run 01 selected `Process.MainWindowHandle`, which pointed at `Winit Thread Event Target`. Its tree/captures and claimed injection sequence are **invalid client evidence**. It was closed; do not infer a screen-reader or keyboard result from it.
- Run 02 encountered a diagnostic-read fixture error before validation; its exact child was stopped. This was not an app/UIA success.
- Run 03 used iced's actual HWND and observed the native Window-only tree, but WScript key requests produced no app transitions. Those requests were not treated as successful keyboard operation. Its old full-frame capture includes window border/shadow and is not the final client-only image.
- Run 04 established SendInput count 0 but the PowerShell native-stderr handling prematurely ended the fixture. Its app closed normally; the corrected run classified this as Blocked instead of losing the resource measurement.
- Run 05 retained the exact .NET Process, confirmed the correct HWND, recorded blocked native input, a 30-second baseline and exit 0. Run 06 separately seeded the actual inline error and confirmed its missing UIA client nodes. Run 07 checks the final normal build.

No failed/invalid run was deleted or relabeled as a passing client test. Screenshots named `error-focus` in early runs do not prove an error was produced; the genuine error-rendering evidence is Run 06's explicitly seeded fixture. The fixture flag is not enabled in the normal application.

## Smallest viable remedy

Integrate a maintained Windows UIA adapter/AccessKit path at the iced semantic/layout boundary while retaining Rust + iced. Start with this window's stable IDs and only its real controls: TextInput (Name, Value/Text, selection/caret where supported), Button (Invoke), CheckBox (Toggle), enabled/focus states, error text and a polite status/notification event. Synchronize physical bounds after DPI/layout changes, dispatch UIA actions to the UI thread, and remove nodes before controls disappear or the HWND closes. UIA callbacks cannot block the GUI or invent focus that the widget does not hold. [UIA provider overview](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-providersoverview), [AccessKit](https://accesskit.dev/).

Re-check then-current stable iced support before adding integration. A native-window wrapper alone will not supply missing custom-widget semantics. Tooltips, internal focus IDs, static labels, screenshots, MessageBox workarounds and mouse-only fallback do not close the client accessibility gate. Do not silently add a GUI framework, copy a draft fork or claim complete Text/Selection support without testing. If a maintained semantic bridge cannot be delivered within the selected architecture, keep essential client rollout blocked and escalate that architecture decision before more UI is built.

## Reproducible manual accessibility procedure

Build/run from the [root instructions](../../README.md), ordinary user rights. Do not start Explorer changes, hooks or shell experiments. For UIA inspect, run Inspect in UI Automation mode and select the titled **visible client** (not winit's internal event window); verify PID/native HWND against diagnostics if uncertain.

1. Open the normal window. Confirm a visible input caret/focus outline; type sample text, move caret into the middle, edit and select. Confirm the caret does not jump to the end after each edit.
2. Clear text, Tab to Validate, press Enter. Expect the inline error without focus relocation or cleared input. Tab to Motion, Space to toggle, Tab to Dismiss error, Enter to dismiss; expect focus back on Sample text. Repeat in reverse with Shift+Tab and confirm no hidden Dismiss stop when the error is absent.
3. With an error present, Escape dismisses it; a later Escape closes. Repeat Close button and Alt+F4. Verify process exits and Windows remains usable. Switch to another app before an external close request; verify no stale foreground restoration steals focus. Record denied/skipped restoration separately from success.
4. With Narrator, navigate input/action/checkbox/status. Inspect Name, ControlType, enabled/focus/toggle/value states and supported patterns. Do not award a pass because the window title is announced. Current recorded build is expected to fail client semantics; this must be re-tested after a provider remedy.
5. Trigger validation and observe/record a polite error announcement while focus remains on its initiating control. Confirm the disappearing Dismiss control leaves neither keyboard nor UIA focus on a removed node. Record auditory result and UIA event observations separately.
6. Test a prepared reduced-motion/high-contrast environment, 100–200% mixed DPI, keyboard-only operation, IME composition, Sticky Keys and AltGr. Do not alter persistent OS settings from the harness; use prepared test environments. Confirm no animations, usable focus/contrast and no composition-induced close/submit.
7. Close the app and Inspect/Narrator instances you opened for the test; do not terminate pre-existing accessibility tools. Save exact build/revision/tool versions, observations and failures. These manual checks are currently Not run.

The supplied developer script is reproducible automation for process/UIA/baseline checks. Its optional fixed injected-key sequence is not a physical-keyboard or screen-reader substitute; on this host it is blocked and stops before sending further input.

## Release idle CPU and memory baseline

Method: one release process at a time; after UIA/capture preparation and 3 s settling, native-backed `.TotalProcessorTime` delta over independent `.Stopwatch` wall time. Percentage = CPU seconds / wall seconds × 100 for **one logical core**, not divided by the host's 16 processors. `.Refresh()` precedes endpoint working-set/private-commit and CPU reads; peak working set includes startup. PowerShell/UIA/probe helper costs are outside the measured app PID. Fields, exact binary hashes and exit results are in each `run.json`.

| Recorded case | Observation | CPU delta / one-core % | Working set end / private commit end / peak bytes |
| --- | --- | --- | --- |
| Run 05, normal input-focused, input injection blocked | 30.013563 s | 0 s / 0% at measurement resolution | 18,382,848 / 4,734,976 / 18,432,000 |
| Run 06, seeded error, logical Validate focus, no caret editing | 30.004293 s | 0 s / 0% at measurement resolution | 18,169,856 / 4,743,168 / 18,219,008 |
| Run 07, final normal build, input-focused | 30.003132 s | 0.171875 s / 0.572857% | 18,214,912 / 4,763,648 / 18,255,872 |

These are short observed baselines, not a zero-CPU promise or a statistical performance acceptance result. Zero delta means no measurable increment in this counter/sample, not proof of no execution/redraws/wakeups. No ETW, power/GPU, high-input-rate, hidden/minimized-window, screen-reader-attached or cross-renderer profile was run. The software renderer's font/cache/working-set behavior is part of the recorded process, unlike Stage 1's smaller native-only harness. Full target-build qualification and repeated longer samples remain open.

## Quality gates and scope

Root workspace formatting, Cargo check, 11 unit tests, strict all-target Clippy, release application build and developer-example build passed; [quality-gates.txt](evidence-stage2a/quality-gates.txt) records outputs. Formatting/lint tooling used a filesystem sandbox exception as in Stage 1; GUI/input tests did not use that exception or administrator elevation. Windows path/hardlink warnings from the sandbox are retained and are distinct from Rust warnings/errors. Local links, new-file whitespace, Git diff and Stage 1 preservation hashes are checked at completion.

Only the minimal workspace/app/adapter, developer procedure, Stage 2 documents and evidence are added. The 29 original Stage 1 files are preserved byte-for-byte, designs/assets remain intact, and nothing is committed. No AppBar reservation, Windows-key interception, persistent keyboard hook, tray host, native taskbar change, background service or full product feature is added.
