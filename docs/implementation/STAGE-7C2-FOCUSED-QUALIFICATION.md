# Stage 7C.2 — Focused Launcher Edge-Case Qualification

2026-10-18, Asia/Calcutta. **Focused checks passed at the human-reported observation level:** one visible CMake GUI after the instructed physical Enter hold, and normal native-taskbar switching before/during/after the launcher session. No reproducible source defect was established. Normal manager shutdown was additionally corroborated by process exit code 0 and `dashboard_workers_joined`.

This is one bounded run, not a release-readiness review, an instrumented key-repeat/launch-count proof or broad Windows qualification. **Production accessibility remains BLOCKED.**

## Baseline, preservation and executable

Repository: `X:\Project\SeiDesk\sei-desk`. Starting and final branch: `main`. Starting and final HEAD: `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`.

Before testing, recorded complete `git status --short` and SHA-256 inventory of all 741 existing tracked/nonignored files. Read Stage 7C, Stage 7C.1 and the current invocation, selection/key submission, repeated-Enter guard, dismissal and shutdown paths. Relevant implementation remains `applications/search.rs::Search::update`, `desktop.rs` window event routing and launch/pending guards, launcher view/selection state, and `main.rs` owner drops after the event loop. Inspection is context, not passing manual evidence.

All generated baseline/log/profile artifacts are under the new uniquely named ignored directory:

`X:\Project\SeiDesk\sei-desk\target\stage7c2-68303d73aebb4742b548261e84383819`

- `status.txt`: exact starting Git status.
- `hashes.json`: 741 baseline paths and hashes.
- `profile/`: new isolated LOCALAPPDATA, protecting actual preferences.
- `runtime.log`: diagnostics for this run only.

Reused `target/release/sei-desk.exe`, SHA-256 `3A6268B33FDD4C4188C608A57D18FF39C4E62BF580E7BB9A0FD7EE3DC5906FB2`, the existing Stage 7B.1 release executable. No rebuild or dependency resolution occurred. No source, manifest, lockfile, earlier report, artwork or evidence was changed; final hash comparison confirms all 741 original files unchanged. Only this report is a new nonignored file. No reset, clean, restore, stash, staging, commit or push occurred.

## Evidence and operation boundaries

The human performed taskbar clicks and physical keyboard interaction. The agent started the existing executable with `--diagnostics` and process-local LOCALAPPDATA, guided the checks, and read repository/log/process state. No Computer Use, injected UI input, synthetic keyboard event or screenshot automation was used to perform the physical checks. No screenshots were supplied; PASS rows rely on the operator's responses, with the exact material responses retained below. There is no recorded key-event trace, timed repeat interval or exact native launch-request count.

Only the existing safe CMake GUI target was permitted. Instructions explicitly excluded **Uninstall CMake**, Configure, Generate, project controls and destructive actions. No installed files were modified to create a failure. No settings, Explorer process, system configuration, administrator privilege, installation, global hotkey or new integration was used.

### Human/result record

| ID | Observation and attribution |
| --- | --- |
| H1 | Before starting Sei-Desk, the human inspected the native taskbar and used actual taskbar items to activate an existing application and return to Codex. Response: **“Visible and usable; both taskbar switches worked.”** This satisfied this run's conditional taskbar prerequisite. |
| H2 | First launch attempt was not accepted as repetition evidence. Operator reported **“non was open, i pressed ENter, now 1 is open”**, then clarified **“Tapped Enter once; launcher closed.”** This establishes a zero-to-one single-press observation only. |
| H3 | Operator was then instructed to close the empty CMake GUI normally, return to manager, invoke with Ctrl+L, search `cmake`, verify CMake (cmake-gui), hold physical Enter about one second and release. In response to the hold-result question: **“1 window open, launcher closed as soon as cmake opened.”** One visible window and dismissal are reported for that instructed hold procedure. The operator did not separately restate the pre-hold window count or measured hold duration. |
| H4 | Native taskbar switches with launcher closed: **“Worked normally.”** No Alt+Tab substitution was instructed or accepted. |
| H5 | Native taskbar switches with launcher open, including checking for unsolicited overlay/persistent focus stealing: **“Worked normally.”** |
| H6 | Native taskbar switching after visible Dismiss: **“Worked normally.”** |
| H7 | Initial post-exit answer was **“Worked normally,”** but the launch session was still running and diagnostics continued. It was not accepted as proof that the manager had exited. Agent instructed closing the larger Overview/Applications manager rather than only the small launcher. Operator clarified **“Manager was open; now closed; post-exit taskbar switches worked.”** This corrected, repeated check supplies the post-exit result. |
| L1 | After H7 correction, the running command completed with **exit code 0**. `runtime.log` ends with **`dashboard_workers_joined`**. Before that correction the session remained active; an empty passive `Get-Process` result was not relied on as proof of exit when session/log evidence contradicted it. |

## Qualification matrix

| Scenario | Prerequisites | Procedure | Expected behavior | Actual outcome / evidence | Status |
| --- | --- | --- | --- | --- | --- |
| Pre-launch native taskbar visibility and usability | Sei-Desk not running; human present | Inspect usual edge; click existing taskbar app, then Codex taskbar item | Bar accessible; both switches work | Explicit H1 confirmation before this run started | PASS — human-reported |
| Safe target and initial single press | Existing CMake GUI result; no CMake GUI initially open | Invoke via local Ctrl+L, search, select safe GUI, tap Enter | One visible GUI; popup dismisses | H2 reports zero before, one afterward and explicitly a tap | PASS — single-press observation only |
| Short physical Enter hold on valid selection | Same safe target; instructed closure of earlier empty GUI; physical keyboard | Reinvoke/search/verify GUI, hold Enter about one second, release and observe | Popup dismisses normally; target does not visibly open more than once | H3 reports one window and immediate dismissal on GUI opening; no duplicate reported | PASS — human-reported one hold procedure, with evidence limits below |
| Taskbar switching, launcher closed | H1 passed; manager and safe peer | Click CMake taskbar item and return through manager taskbar item | Normal native switches | H4 | PASS — human-reported |
| Taskbar switching, launcher open | H1 passed; popup opened with Ctrl+L | Activate ordinary taskbar item, return deliberately to manager through taskbar | No taskbar disruption, unsolicited overlay or persistent focus forcing | H5 | PASS — human-reported bounded interval |
| Taskbar switching after Dismiss | Open popup, deliberate manager return | Bring popup forward with local command if needed, click Dismiss, repeat native taskbar switches | Manager remains; switches normal | H6 | PASS — human-reported |
| Normal manager exit | Running larger manager | Close manager normally; wait for command completion | Process exits; owned workers join | H7 clarification, exit 0 and L1 join marker | PASS — human report + runtime evidence |
| Post-exit native taskbar switching | Confirmed manager exit | Activate CMake and return to Codex through taskbar | Native taskbar still usable | H7 corrected/repeated result after manager closure; L1 | PASS — human-reported |
| Duplicate launch reproduction | A clearly observed duplicate | Repeat safely and trace narrow submission path | Reproducible defect before any correction | No duplicate was reported in H3; no defect established | NOT TESTED — no failing reproduction to pursue |
| Quantitative repeat/launch-event measurement | Instrumented or timed procedure | Measure hold duration, generated repeat events and all native launch requests | Establish exact event/request counts | No such instrumentation or timing used | NOT TESTED |

### Interpretation and limitations

The observed result closes the requested visible valid-target repetition check for this instructed physical hold, not every repeat configuration or repeated-launch timing. H3 does not independently prove exact hold duration, the OS repeat delay/rate, the precise pre-hold count or a single native launch request. No source-only assertion was used to fill those measurements. The initial tapped Enter was explicitly separated from the later hold procedure.

All required conditional taskbar interaction cases were reported performed through actual native taskbar items. H1 justified proceeding; the original Stage 7C.1 **“Native taskbar did not appear”** pre-launch observation remains unchanged in that historical report. A successful new baseline neither reinterprets the old failure nor diagnoses its cause. The premature first post-exit answer was reconciled with active-session evidence and repeated after actual manager closure.

No fullscreen, multi-monitor, display scaling, numeric monitor/work-area, UIA/Narrator, IME, high-contrast, crash-recovery or real focus-denial scenario was executed or newly passed here. Those remain at their previously documented status. No taskbar replacement, AppBar reservation or fullscreen guarantee is inferred.

## Defect/fix disposition and validation

**No concrete reproducible source defect was established.** The manager remaining open was clarified as an incomplete closure step; it closed normally when the larger manager was actually closed. No minimal fix or regression test was necessary. No Cargo test suite, formatting tool, workspace check, Clippy run or build was rerun or implied.

Exact commands/actions and outcomes:

- `git branch --show-current`, `git rev-parse HEAD`, complete `git status --short`: recorded before testing and at completion; branch/HEAD unchanged.
- `git ls-files --cached --others --exclude-standard` and `Get-FileHash -Algorithm SHA256`: 741 original paths inventoried; all unchanged. Release-executable hash recorded above.
- `git check-ignore target/stage7c2-68303d73aebb4742b548261e84383819/status.txt`: confirmed generated directory is ignored.
- Create isolated `profile/`, set process-local LOCALAPPDATA, run `target/release/sei-desk.exe --diagnostics` redirected to this run's `runtime.log`: human-operated session started; final command exit 0.
- Passive log reads and command-session polling: monitoring continued before actual manager closure; final `dashboard_workers_joined` after closure. Passive process listing alone did not establish exit.
- `git diff --check`: passed, existing LF/CRLF notices only; new-report whitespace checked separately.

Recommended next step: owner review of these two focused human-reported results and their evidence precision. Retain the broader Stage 7C/7C.1 limitations. Additional platform/manual qualification would require an explicit future task; no further stage starts automatically. **Production accessibility remains BLOCKED.**

## Final Git state

Only new nonignored file: `docs/implementation/STAGE-7C2-FOCUSED-QUALIFICATION.md`. Complete starting status is saved in this run's `status.txt`; it equals the complete final list below with only the Stage 7C.2 report entry omitted. All 741 original paths remain byte-identical; all pre-existing tracked and untracked work is preserved.

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
?? docs/implementation/STAGE-7C2-FOCUSED-QUALIFICATION.md
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

No stage, commit or push. Stopped after this report for owner review.
