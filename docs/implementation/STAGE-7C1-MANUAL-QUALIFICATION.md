# Stage 7C.1 — Targeted Manual Qualification

2026-10-18, Asia/Calcutta. **Partial human-operated qualification.** The operator confirmed ordinary physical-keyboard and focus/dismissal sequences, ordinary maximized-window coexistence, visual launcher visibility and manager shutdown. A later response confirmed that the native taskbar appeared and application switching worked after exit. Full before/during/after taskbar coexistence remains incomplete: the taskbar did not appear before launch, and taskbar interaction during the session was not established. Numeric display/work-area and genuine fullscreen qualification remain open. This is not a release-readiness claim. **Production accessibility remains BLOCKED.**

## Baseline and preservation

Repository: `X:\Project\SeiDesk\sei-desk`. Starting/final branch: `main`. Starting/final HEAD: `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`.

Read the Stage 7C report and relevant desktop invocation, ID/generation routing, keyboard/Retry, dismissal, placement and ownership implementation. Before testing, recorded complete `git status --short` and inventoried 740 existing tracked/nonignored paths with SHA-256 hashes under ignored `target/stage7c1-baseline/status.txt` and `hashes.json`. All 740 original files remain byte-identical. Existing source, fixes, untracked code, historical reports, mockups, manifests, lockfiles and evidence were preserved. Earlier ignored evidence was not overwritten or removed.

Only new nonignored path is this report. New ignored artifacts are the uniquely named baseline directory, initially empty `target/stage7c1-profile` and `target/stage7c1-runtime.log`. No reset, clean, restore, stash, stage, commit, push, dependency installation, administrator privilege, system-setting change or Explorer manipulation occurred. No source change was warranted or made.

## Environment and evidence boundaries

- OS query this stage: `Microsoft Windows NT 10.0.26300.0`.
- Existing Stage 7B.1 release executable: `target/release/sei-desk.exe`, SHA-256 `3A6268B33FDD4C4188C608A57D18FF39C4E62BF580E7BB9A0FD7EE3DC5906FB2`. No rebuild/dependency resolution was needed for this manual-only stage.
- Started the executable with `--diagnostics` and process-local `LOCALAPPDATA` set to absolute `target/stage7c1-profile`. Actual user preferences were untouched.
- The human explicitly answered “Available now; guide me through the checks.” No Computer Use/Sky input, injected keyboard events, synthetic UI observations or remote-control actions were used in this stage. The agent launched the existing application and performed passive repository/process/log reads; those actions are separate from human physical input.
- CMake GUI was the instructed safe known application. The operator was explicitly told not to activate **Uninstall CMake**, not to use Configure/Generate/project controls and not to alter installed files to create a failure.
- Monitor count, scale values, topology, primary-display identity and a harmless genuine fullscreen setup were requested but not reported. They remain unknown; prior Stage 7C CIM failures are historical context, not a new hardware inventory.
- No new screenshot was supplied or captured in this stage. Manual PASS rows rest on the operator's text responses to the specified sequences, not independent visual measurement. Short bundled responses do not establish additional unasked details or numeric telemetry.

### Human evidence record

| ID | Guided procedure and actual response |
| --- | --- |
| H0 | Operator confirmed availability for physical checks. |
| H1 | Initial taskbar answer “none” was clarified, rather than treated as success. Operator said the check had not been completed. After instructions to move to the usual taskbar edge and switch an existing app/return to Codex, the operator selected “Taskbar did not appear or switching failed,” then specifically **“Native taskbar did not appear.”** This was before the isolated Sei-Desk session started. |
| H2 | Physical sequence: click manager, Ctrl+L, type `zzzz-no-app`, observe no-match, hold Enter about one second, Ctrl+A/Backspace, Escape. Operator replied **“works, no issues.”** This confirms the instructed sequence by self-report; valid-target held Enter was not part of it. |
| H3 | Physical sequence: Ctrl+L, type `cmake`, Down/Up, ensure safe CMake GUI selection, Ctrl+Enter, Alt+Enter, plain Enter once. Expected no modified-key launches, one CMake GUI launch and popup dismissal. Operator replied **“woeks.”** Recorded as affirmation of the bundle. No separate application-window count or screenshot was provided. |
| H4 | Physical/pointer sequence: Alt+Tab to manager, Ctrl+L, Alt+Tab to CMake, wait about five seconds, deliberately return to manager, reinvoke, click visible Dismiss. After a non-result “continue” message was explicitly not accepted as a pass, the outcome request received **“worked.”** Recorded as affirmation of the focus/reuse/dismissal bundle only; no display setup was reported. |
| H5 | Final instructions requested ordinary CMake maximization, visual launcher placement, manager closure with popup open, post-exit taskbar check and normal empty-CMake closure. The response arrived after the first report version: **“All worked; launcher visible; both Sei-Desk windows closed”** and **“Taskbar appeared and switching worked.”** This update records those results without replacing the earlier pre-launch taskbar observation. CMake's final closure was not separately confirmed. |
| L1 | Passive log read found `dashboard_workers_joined` at the end of `target/stage7c1-runtime.log`. Passive `Get-Process sei-desk` returned no process. This establishes a shutdown observation, not how the operator closed the manager or whether a popup was open at that moment. An exact process exit code was not retrieved and is not inferred. |

## Taskbar interaction — priority 1

| Scenario | Prerequisites | Procedure | Expected behavior | Actual observation / evidence | Status |
| --- | --- | --- | --- | --- | --- |
| Native taskbar visible before launch | Operator on ordinary desktop; no Sei-Desk session | Move pointer to usual taskbar edge without changing settings | Native taskbar visible/accessible | Operator explicitly reported it did not appear, H1 | FAIL — pre-launch prerequisite only |
| Baseline taskbar app switch / return to Codex | Visible usable native taskbar | Click ordinary existing taskbar item, then Codex | Both switches work | Cannot establish switches with the absent-taskbar baseline; H1 | NOT TESTED |
| Taskbar app activation with launcher open | Successful native-taskbar baseline, manager/popup | Click ordinary taskbar item and return through taskbar | Native taskbar works; no disruption | Prerequisite not met; not substituted with Alt+Tab | NOT TESTED |
| Open/dismiss while switching ordinary apps | Available manager and safe CMake GUI | Physical Alt+Tab, Ctrl+L, wait, reinvoke and visible Dismiss | No focus loop, duplicate popup or unintended manager closure | Operator affirmed bundle, H4 | PASS — human-reported application switching, not taskbar switching |
| Taskbar functional after normal exit | Completed shutdown and direct native taskbar observation | Recheck edge, activate a taskbar item | Native bar accessible and functional | Operator confirmed “Taskbar appeared and switching worked,” H5 | PASS — human-reported post-exit interaction |

The failed taskbar prerequisite predates launch and is **not attributed to Sei-Desk**. It does not diagnose Explorer, an auto-hide configuration, fullscreen state or a Windows fault. No taskbar settings or Explorer processes were changed. A passing source inspection or prior target-window screenshot cannot replace this missing taskbar evidence.

## Physical input — priority 2

| Scenario | Prerequisites | Procedure | Expected behavior | Actual observation / evidence | Status |
| --- | --- | --- | --- | --- | --- |
| Existing manager command | Physical keyboard, manager active | Press Ctrl+L | One launcher opens | H2/H3 affirmed | PASS — human-reported |
| Query entry / no-match | Physical keyboard, launcher | Type `zzzz-no-app` | Query and no-match feedback visible | H2 affirmed | PASS — human-reported |
| Query clearing | Same session | Ctrl+A, Backspace | Query clears safely | H2 affirmed | PASS — human-reported |
| Repeated Enter with no result | No-match query | Hold physical Enter briefly, release | No application launch; popup retained until dismissal | H2 affirmed | PASS — no-match case only |
| Repeated Enter on valid selection | Safe controlled valid-target repetition setup | Hold Enter while a valid target is selected; check duplicates | No unintended repeated launch | Not performed; no-match repetition does not prove this case | NOT TESTED |
| Arrow selection | Physical keyboard, `cmake` results | Down, Up; select safe GUI entry | Visible selection moves/reconciles | H3 affirmed | PASS — human-reported |
| Ctrl+Enter | Safe GUI selected | Physical Ctrl+Enter | No launch | H3 affirmed | PASS — human-reported |
| Alt+Enter | Safe GUI selected | Physical Alt+Enter | No launch | H3 affirmed | PASS — human-reported |
| Plain Enter once | Confirmed safe CMake GUI selection | Physical Enter once | GUI launch accepted; popup closes | H3 affirmed the bundle; exact window count not independently measured | PASS — human-reported single-press workflow |
| Escape | Launcher open after clearing query | Physical Escape | Popup only closes; manager stays | H2 affirmed | PASS — human-reported |
| Visible Dismiss | Reopened launcher | Human clicks Dismiss · Esc | Popup only closes | H4 affirmed | PASS — human-reported pointer action |
| Deliberate focus change and return | Manager, popup, safe peer | Alt+Tab away, wait, return deliberately | No persistent focus forcing or unexpected foreign restoration | H4 affirmed the bounded sequence | PASS — human-reported; no continuous foreground trace |
| Reinvocation | Existing popup | Return to manager and Ctrl+L again | Reuse one popup | H4 affirmed; native window identity not measured here | PASS — human-reported visible behavior |
| Manager close with popup open | Manager and companion open | Close manager normally | Both Sei-Desk windows disappear | Operator confirmed both windows closed, H5; passive join marker corroborates shutdown, L1 | PASS — human-reported + log |
| Launch-failure recovery | Safe controlled failure through actual launcher UI | Reject a fixture launch and exercise explicit Retry physically | Retain state; explicit matching Retry only | No safe manual failure setup was established; installed app files untouched | NOT TESTED |

These are physical-input self-reports from the available operator. Earlier Stage 7B.1/7C widget/native-fixture tests remain historical automated evidence and are not relabeled as manual passes. The operator's terse bundled confirmations are preserved above so reviewers can assess their precision.

## Display behavior — priority 3

| Scenario | Prerequisites | Procedure | Expected behavior | Actual observation / evidence | Status |
| --- | --- | --- | --- | --- | --- |
| Visual launcher visibility on manager's display | Manager and launcher visible to operator | Invoke and inspect all edges on the manager's display | Launcher wholly visible, no part offscreen | Operator confirmed the guided visibility check worked, H5; no screenshot or numeric bounds | PASS — human-reported visual check only |
| Numeric invoking-monitor/work-area containment | Identified monitor/work-area rectangles, scale and window bounds | Compare actual geometry | Same intended monitor and full usable-work-area containment | Visual visibility alone does not establish monitor identity, scale or numerical work-area boundaries | NOT TESTED |
| Each available display | Reported multi-display hardware/setup | Move manager to each existing display and invoke | Correct invocation monitor | Hardware count/topology unreported; no assumption of a single monitor | NOT TESTED |
| Negative-coordinate layout | Existing known negative-coordinate monitor arrangement | Invoke there without reconfiguring displays | Correct monitor-local placement | Setup not established | NOT TESTED |
| Mixed display scaling | Known original configuration and safe reversible setup | Test existing supported scales / restore if separately authorized | Correct size, placement and focus | Scale values unknown; no display settings changed | NOT TESTED |
| Native shell/input integration exclusions | Current production companion source | Inspect invocation, native placement and dismissal paths | No global hotkey/AppBar/taskbar replacement/Explorer route | Source review only; not physical display behavior. Existing local Ctrl+L and owned-window APIs retained | PASS — source scope check only |

## Fullscreen and maximized windows

| Scenario | Prerequisites | Procedure | Expected behavior | Actual observation / evidence | Status |
| --- | --- | --- | --- | --- | --- |
| Ordinary maximized app | Empty harmless CMake GUI | Maximize through titlebar; switch deliberately to manager/launcher | No unsolicited overlay or persistent focus forcing | Operator confirmed “All worked” for the guided sequence, H5 | PASS — human-reported maximized-window sequence |
| Genuine borderless fullscreen | Known harmless app/mode, safe exit | Enter/exit genuine borderless mode and observe | Safe ordinary-window coexistence | Suitable setup not reported or exercised | NOT TESTED |
| Exclusive fullscreen | Suitable harmless app and safe restoration | Enter/exit exclusive mode and observe | No unsupported overlay/input interference | Setup not established | NOT TESTED |

Maximized, borderless fullscreen and exclusive fullscreen remain separate cases. No maximized-window result is promoted to fullscreen evidence. No display setting change, hardware disconnect, fullscreen manipulation or Explorer action was performed by the agent.

## Defects, validation and remaining blockers

No concrete reproducible Sei-Desk defect was established, so no source fix or new regression test was made. The taskbar prerequisite did not meet expectations before launch; attribution remains unresolved and outside this stage's authorized repair scope. Post-exit taskbar interaction passed by human report, but that does not retroactively pass the pre-launch or during-session cases. Physical valid-target repeat, controlled manual failure recovery, complete taskbar coexistence, numeric monitor/work-area placement and genuine fullscreen remain open.

Commands/actions this stage:

- `git branch --show-current`, `git rev-parse HEAD`, complete `git status --short`, `git ls-files --cached --others --exclude-standard` and SHA-256 inventory: baseline recorded; 740 existing files unchanged.
- `Get-FileHash target/release/sei-desk.exe -Algorithm SHA256`: exact existing build identity recorded above.
- Read Stage 7C and relevant source; no edits.
- Start `target/release/sei-desk.exe --diagnostics` with new process-local isolated LOCALAPPDATA. Launch used normal filesystem access; no UAC/admin privilege or automated input.
- Passive `Get-Content target/stage7c1-runtime.log -Tail 1`: `dashboard_workers_joined`. Passive `Get-Process sei-desk`: no process returned. Exact exit code not available.
- `git diff --check`: passed, existing LF/CRLF notices only. New-report whitespace checked separately.

No Cargo tests, controlled renderers or broad build/Clippy checks were rerun for this source-unchanged manual stage. No earlier result is counted as a new run. No new screenshots are claimed.

Recommended next step: owner review of this partial evidence. Establish a usable native-taskbar baseline without assuming this app caused its earlier absence, then complete direct pre-launch and during-session taskbar switches; retain the confirmed post-exit result as evidence for that case only. Obtain explicit hardware/fullscreen setup details before those tests. Add attributable results without upgrading untested rows. CMake cleanup was instructed, but its final closure was not separately reported. **Production accessibility remains BLOCKED**; this stage supplies no Narrator/UIA, real-IME, high-contrast or release qualification and authorizes no provider/architecture work.

## Starting and final Git state

The exact starting short status is preserved at `target/stage7c1-baseline/status.txt`; it is the complete final list below with only this new Stage 7C.1 report entry omitted. Branch and HEAD are unchanged. All pre-existing 740 inventoried paths are byte-identical; no earlier evidence was overwritten or removed.

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
?? docs/implementation/STAGE-7C1-MANUAL-QUALIFICATION.md
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

Only this report was added to the nonignored tree. No stage, commit or push. Stopped for owner review; no additional stage or excluded integration began.
