# Stage 8A.1 — Human Confirmation of Launcher Recovery

2026-10-18, Asia/Calcutta. **All seven requested steps passed by explicit human report.** The operator used the actual physical keyboard for the key sequences and the visible control for dismissal. No regression was reported or reproduced; no source changes, tests or builds were performed in this confirmation stage. **Production accessibility remains BLOCKED.** This is not Narrator/UI Automation qualification or general release readiness.

## Baseline and preservation

Repository: `X:\Project\SeiDesk\sei-desk`. Starting/final branch: `main`. Starting/final HEAD: `af7db2f0a9b1fc2540071a3d2f41c9f4ca7c139c`.

Read `STAGE-8A-LAUNCHER-UX-PRODUCTIZATION.md`, the Stage 8A source/test diff, and the existing launcher specification at `docs/design/SEI-DESK-UI-UX-SPEC.md` §8. Its no-results Enter rule is the authority for this check. Stage 8A's prior test/UI results are historical context, not new runs here.

Starting complete `git status --short`:

```text
 M crates/sei-desk/src/desktop.rs
 M crates/sei-desk/src/desktop/tests.rs
?? docs/implementation/STAGE-8A-LAUNCHER-UX-PRODUCTIZATION.md
```

Recorded starting status and SHA-256 inventory of all 744 existing tracked/nonignored files in the new ignored directory:

`X:\Project\SeiDesk\sei-desk\target\stage8a1-9f268f92ac464a8ab8e8d020a8daa06b`

Files: `status.txt`, `hashes.json`, isolated `profile/`, and `runtime.log`. All 744 baseline files remain byte-identical, including the uncommitted Stage 8A fix, tests, report and earlier evidence. Only this report was added to the nonignored tree. No reset, clean, restore, stash, staging, commit or push occurred. No Windows setting, Explorer process, system configuration or administrator privilege was used.

## Executable and operation boundaries

Used the existing rebuilt Stage 8A `target/release/sei-desk.exe`, SHA-256:

`107EDE794E3AEA6F76AC4DE2C1CED5F4A0766FD9C5BB9BCD3766DB34F4C73EA3`

The agent launched it with `--diagnostics`, setting process-local LOCALAPPDATA to this run's new empty `profile/` directory. Real user preferences were untouched. The human performed the guided interaction; the agent sent no Computer Use/Sky, injected or synthetic UI input. No application launch was requested during the sequence.

No screenshot was required, supplied or captured. Manual results below are attributable text confirmations, not independently measured visual/keyboard telemetry. The runtime exit/log evidence is separate from those human observations.

## Human confirmation results

| Step | Procedure and expected behavior | Actual operator response | Result |
| --- | --- | --- | --- |
| 1 | In manager, press physical Ctrl+L; companion opens | **“Yes”** to whether physical Ctrl+L opened the companion | PASS — human-reported |
| 2 | With Favorites empty, press Enter; empty guidance remains, no selection error | **“Yes—guidance remained; no selection error”** | PASS — human-reported |
| 3 | Select All applications; valid results and selection appear without stale error | **“Yes—valid results/selection; no stale error”** | PASS — human-reported |
| 4 | Focus search, type `zzzz-no-app`, press physical Enter; query and no-results guidance remain | **“Yes—query and guidance remained”** | PASS — human-reported |
| 5 | Ctrl+A, Backspace; valid results/selection return and search remains usable. Type `zzzz-no-app` again to verify typing focus | **“Yes—results restored and search typing worked”** | PASS — human-reported |
| 6 | Press physical Escape; only companion closes, manager stays | **“Yes—only the companion closed”** | PASS — human-reported |
| 7 | Return deliberately to manager, Ctrl+L, activate visible Dismiss · Esc; popup closes while manager remains | **“Yes—reopened; Dismiss closed only the popup”** | PASS — human-reported |

The All applications control was permitted through physical Tab twice/Enter from search or a manual click; the operator's response does not distinguish which activation method was used. Dismiss activation is confirmation of the visible control, not a claim that every control was reached by keyboard alone.

### Session cleanup and independent runtime evidence

After step 7, the operator initially answered **“Not yet”** to manager closure. That was not treated as a shutdown pass. After a follow-up request to close the larger manager normally, the response was **“Yes—closed normally.”** The running command then completed with **exit code 0**, and `runtime.log` ends with **`dashboard_workers_joined`**.

This corroborates normal session shutdown. It does not replace the human evidence for the seven interaction steps or broaden their scope.

## Validation and file integrity

Commands/actions actually performed:

- `git branch --show-current`, `git rev-parse HEAD`, complete `git status --short`: recorded before/after; branch and HEAD unchanged.
- `git diff -- crates/sei-desk/src/desktop.rs crates/sei-desk/src/desktop/tests.rs`: inspected the existing Stage 8A changes without altering them.
- `git ls-files --cached --others --exclude-standard` plus SHA-256 inventory: 744 baseline paths; all unchanged at completion.
- `Get-FileHash target/release/sei-desk.exe -Algorithm SHA256`: recorded the executable identity above; no rebuild.
- Isolated-profile `target/release/sei-desk.exe --diagnostics`: human-operated session; command exit 0, worker-join marker observed.
- `git diff --check`: passed; existing LF/CRLF notices only. New-report whitespace checked separately.

No Cargo tests, formatting, workspace checks, Clippy, build, provider experiment or architecture work was run in this stage. No source or regression-test change was necessary because no concrete regression was established.

## Final state and limitations

Final complete `git status --short`:

```text
 M crates/sei-desk/src/desktop.rs
 M crates/sei-desk/src/desktop/tests.rs
?? docs/implementation/STAGE-8A-LAUNCHER-UX-PRODUCTIZATION.md
?? docs/implementation/STAGE-8A1-HUMAN-CONFIRMATION.md
```

The only new nonignored file is `docs/implementation/STAGE-8A1-HUMAN-CONFIRMATION.md`. All pre-existing changes and evidence remain intact.

This short human session confirms the requested empty-Favorites/no-match/browse/clear/dismiss/reopen recovery sequence on the rebuilt application. It does not establish exact timing, all keyboard configurations, launch-failure recovery, shell/taskbar behavior, fullscreen, mixed DPI, high contrast, real IME, Narrator/UIA or general release readiness. **Production accessibility remains BLOCKED.**

Stopped after this report for owner review. No next stage, commit or push.
