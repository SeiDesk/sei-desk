# Stage 6D — End-to-end usability and edge-case verification

2026-10-17, Windows x64/MSVC. **Audit-only outcome: no new concrete, reproducible defect found in this verification pass.** No implementation or test changes were justified. This is scoped workflow evidence, not whole-product release acceptance. Production accessibility remains **BLOCKED**.

## Baseline, scope and preservation

Repository: `X:\Project\SeiDesk\sei-desk`. Starting and final branch: `main`. Starting and final HEAD: `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`.

Read the Stage 4A–4D, 5A, 6A, 6B and 6C reports and inspected the integrated update/navigation/input paths in `crates/sei-desk/src/main.rs`, current view, monitor/history, launcher model/browser, preferences/worker, runtime coordinator and service/Consumer/native tests. Earlier stages' evidence was context, not a substitute for the checks recorded below.

Starting `git status --short`:

```text
 M README.md
 M crates/sei-desk/src/applications.rs
 M crates/sei-desk/src/applications/preferences.rs
 M crates/sei-desk/src/applications/preferences/tests.rs
 M crates/sei-desk/src/main.rs
 M crates/sei-desk/src/traffic_chart.rs
 M crates/sei-desk/src/view.rs
 M tools/Test-Stage2A.ps1
?? crates/sei-desk/src/applications/search.rs
?? docs/implementation/STAGE-5A-STABILIZATION-ACCEPTANCE.md
?? docs/implementation/STAGE-5B-ACCESSIBILITY-ARCHITECTURE-DECISION.md
?? docs/implementation/STAGE-5C-OWNER-DECISION-PACKET.md
?? docs/implementation/STAGE-5D-ACCESSIBILITY-CANDIDATE-PREFLIGHT.md
?? docs/implementation/STAGE-5E-UPSTREAM-FIRST-OWNER-DECISION.md
?? docs/implementation/STAGE-6A-DASHBOARD-RELIABILITY-REPORT.md
?? docs/implementation/STAGE-6B-LAUNCHER-ACCEPTANCE.md
?? docs/implementation/STAGE-6C-VISUAL-REFINEMENT-REPORT.md
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

Before running checks, a SHA256 inventory covered **730 pre-existing tracked/nonignored untracked files**. Inventory and entry status are retained in ignored `target/stage6d-baseline/hashes.json` and `status.txt`. Final comparison confirms **all 730 unchanged**, including every Stage 5A/6B fix, Stage 6C visual change, manifest, lockfile, existing test, report, script and evidence file. The sole new nonignored file is this report. No reset, clean, restore, stash, staging, commit or push occurred.

## Runtime setup and directly observed workflows

Used the current workspace release binary twice, with `--diagnostics`, and automated native app interaction through Computer Use. For these two processes only, `LOCALAPPDATA` was set to resolved workspace `target/stage6d-profile/`. The preferences implementation reads that variable; native catalog discovery still uses Windows known folders. This isolated the favorite experiment from the user's saved configuration. No installed shortcut, target or system setting was changed to create a failure.

| Workflow | Actual observation | Result / boundary |
|---|---|---|
| Navigate while monitoring | Overview initially showed CPU 10.3%, physical memory 18.5/25.5 GB and live network rates/history. After launcher interactions, returning showed new CPU/network values, fresh update age and 30 measured intervals. | PASS for ordinary observation. Monitoring continued while Applications was open; no apparent stale replay or frozen UI. Natural traffic bursts were not classified as stale-data defects. |
| Minimize/restore and resize | Used native Minimize, restored the same window, then native Size to reduce outer width from 1102 to 772 px at 772 px height. Overview retained readable content and continued updating; history remained at its bounded live window. | PASS at these observed sizes and one minimize/restore cycle. No DPI or long soak claim. |
| Installed application search/selection | `cmake` returned exactly two results: **CMake (cmake-gui)** and Uninstall CMake. The selected item was the GUI application. List→Grid retained that identity. Reverse sorting moved its position without retargeting it; pinned grouping/filtering retained it. | PASS. The uninstaller was never activated. |
| Rapid queries and clearing | Sent ordinary key events for `zzz`, `paint`, `cmake`, using Escape between queries. Final selection remained the valid CMake result. Separately observed `zzz` in All/List: zero results, explicit no-match guidance, disabled launch/pin. Escape restored 78 results with a visible valid selection. | PASS for these short sequences; not a timing/load benchmark. |
| Pin and restart | Pinned CMake; UI changed to Unpin selected and `Pinned · Shared Start menu`. Isolated `pins-v1.txt` contained exactly its label and stable token `c21c9b183449728c342fe0a119acef5d6bad749f966a6df1ce1c2ab4f133760b`. Closed and restarted the same isolated profile; CMake was still pinned. | PASS, actual GUI save/reload plus file-content check. The file contains an identity token, not a launch command. |
| Session-only presentation | Before exit, chose Grid, Name Z–A, Group Pinned and Show Pinned. After restart, Applications opened with **List, Name A–Z, Ungrouped, All**, and empty query, while the CMake favorite survived. | PASS: persistent favorite and session choices remain distinct. |
| Unpin and empty favorites | After restart selected CMake and chose Unpin. Isolated file returned to its v1 header with no records; Show Pinned displayed “No pinned applications yet” and disabled launch/pin. | PASS for intended identity removal and immediate persisted bytes. No third GUI restart was needed to claim the already-observed pin reload; unpin reload is covered by store tests, not a third runtime observation. |
| Enter policy and safe launch | With CMake selected and search actually focused, Ctrl+Enter left the launch state unchanged. Plain Enter opened a visible **CMake 4.3.4** window from `C:\Program Files\CMake\bin\cmake-gui.exe`. Inspected its empty project view and closed it without configuring, generating or saving anything. Sei-Desk displayed process acceptance for CMake. | PASS for this selected target and Ctrl modifier. Actual window observation supplements, rather than changes, the application's deliberately limited process-acceptance wording. |
| Repeated refresh and page switching | Double activation of Refresh completed accepted sequential refreshes through catalog generation 3, retained the pinned identity and 78 entries/83 reported source issues. Switched back to Overview and saw fresh monitoring data. | No duplicate-launch/freeze symptom. Refresh was fast enough that the captured UI already showed completion; exact page-switch-while-pending overlap was **not established** by this GUI sequence. Busy rejection and pending cancellation are covered by controlled tests below. |
| Startup and shutdown | Both instances started normally, reported explicit initial catalog loading, then usable partial discovery. Both closed with Alt+F4, process exit code 0 and exactly one `dashboard_workers_joined` marker per session. | PASS for ordinary owned shutdown. No forced kill, hung native API or crash-recovery qualification. |

No screenshot evidence files were retained: there was no reproduced issue requiring a capture bundle. Native state images were used transiently to observe the actual windows. Temporary diagnostics were inspected for catalog/launch transitions and clean joins, then the two newly generated session logs were removed from ignored `target` after recording the outcomes here. No pre-existing evidence was removed. The isolated profile remains an ignored test artifact.

## Controlled edge cases and source boundaries

These tests ran during this stage. They establish the specified model/service/widget contracts; they are not falsely labeled as physical human or production failure-injection observations.

| Case | Existing implementation/test evidence | Outcome |
|---|---|---|
| Stale/out-of-order monitoring | `main.rs::tests::out_of_order_sample_cannot_replace_rates_or_steal_navigation_focus` feeds new samples then an old value 999; retains latest CPU/rates/history and Applications focus. `paused_readings_and_navigation_are_independent` covers paused state alongside navigation. | PASS in workspace run. |
| Unavailable/baseline/recovery and bounded history | `traffic/tests.rs` covers failures/baselines as gaps, valid recovery, cohort reset, nonfinite/stale points, real elapsed geometry and 32-point/60-second retention. `sei-windows` tests cover reset/wrap, failure, disconnect/reconnect, invalid capacity and unknown battery. | PASS. No physical adapter disable, battery change or native-call fault was induced. |
| Refresh failure and stale completion | `applications/runtime/tests.rs::refresh_failure_keeps_last_good_catalog_and_rejects_stale_generation_launch`, cancellation/pre-entry race tests, and Consumer tests cover late/replayed completions and last-good catalog retention. | PASS. Production catalog had reported source issues but not an induced whole-refresh failure. |
| Missing/changed target and captured identity | Runtime and Consumer tests reject missing/stale selection, capture the selected request before later selection changes and refuse overlapping launches. Existing Windows fixture tests exercise removed/locked targets, stale/duplicate shortcuts and structured arguments. | PASS, with all opt-in fixture tests explicitly run. No real installed target was deleted or replaced. |
| Modified/repeated Enter | `main.rs::tests::actual_search_widget_respects_modified_and_repeated_enter` lays out the actual search-containing view, focuses the real input and dispatches widget plus global events. Expected Submit/Activate counts for plain/Ctrl/Alt/Logo/repeat remain `[1,0,0,0,0]`. | PASS. Ctrl additionally observed live; Alt/Logo/repeat are synthetic widget evidence. No Windows-key shortcut was injected into the desktop. |
| Bounds, cancellation and joins | Runtime queue-capacity/latest-frame, both-lane completion, service panic/backpressure and blocked cancellable shutdown tests; Consumer pending cancellation/owned join tests; monitor undrained-channel shutdown tests. | PASS. Source retains bounded commands, one-slot publications, two Consumer service lanes and owned handles. Arbitrarily nonreturning native calls are not proven time-bounded. |
| Favorite errors/recovery | Existing preferences/worker tests cover strict format, atomic replacement/reload, interrupted temp/lock files, native replacement failure, conflict detection, malformed/future/oversized data, bounded recovery and shutdown flushing. | PASS. GUI pin/unpin/restart supplements these isolated tests. |

The inspected paths remain the existing `main.rs` message reducer/local event policy; `applications.rs` selection/generation reconciliation; `applications/browser.rs` presentation and pin transactions; `applications/search.rs` submit guard; `applications/runtime.rs` bounded ownership; `applications/preferences.rs` and `preferences/worker.rs`; `sei-apps/src/consumer.rs` and native catalog/launch modules; `monitor.rs`, `traffic.rs` and `sei-windows` metrics/network. No speculative queue, polling or lifecycle redesign was introduced.

## Validation commands and actual results

Used process-local `CARGO_INCREMENTAL=0` and existing locked/offline dependencies. Persistence/workspace/controlled-launch checks and isolated GUI saves used normal filesystem access because restricted Windows atomic-replacement fixture access is a known earlier-stage limitation. No administrator/UAC GUI action, dependency installation or test weakening occurred.

| Command | Outcome |
|---|---|
| `cargo fmt --all --check` | PASS, exit 0; no formatting edits. |
| `cargo check --workspace --all-targets --locked --offline` | PASS, exit 0. |
| `cargo test --workspace --locked --offline --quiet` | PASS: **120 passed, 0 failed, 5 intentionally ignored** (45 service + 59 dashboard/model/widget/worker + 16 Windows); doc tests 0. |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | PASS, exit 0. |
| `cargo build --workspace --release --locked --offline` | PASS, exit 0; current binary used for both sessions. |
| `cargo build -p sei-apps --release --examples --locked --offline` | PASS, exit 0. |
| Resolve `SEI_APPS_FIXTURE` to workspace `target/release/examples/launch_fixture.exe`; `cargo test -p sei-apps --locked --offline -- --include-ignored` | PASS: **47 unit + 1 Consumer Windows + 2 Windows launch = 50 passed**, none ignored. Only the suite's controlled fixture is launched by those tests. |
| Set process-local `LOCALAPPDATA` to resolved `target/stage6d-profile`; run `target/release/sei-desk.exe --diagnostics` twice | Both PASS: exit 0 and owned-worker join marker. |
| `git diff --check` | PASS, exit 0. Existing LF-to-CRLF notices are not whitespace failures. |

No validation command failed in Stage 6D. The full and opt-in runs overlap; their counts must not be added as unique tests. No new regression test was added because no new defect was established, and existing tests were retained unchanged.

## Final state and remaining limits

**Exact Stage 6D addition:** `docs/implementation/STAGE-6D-END-TO-END-ACCEPTANCE.md` only. The tracked diff is identical to entry. Final `git status --short`:

```text
 M README.md
 M crates/sei-desk/src/applications.rs
 M crates/sei-desk/src/applications/preferences.rs
 M crates/sei-desk/src/applications/preferences/tests.rs
 M crates/sei-desk/src/main.rs
 M crates/sei-desk/src/traffic_chart.rs
 M crates/sei-desk/src/view.rs
 M tools/Test-Stage2A.ps1
?? crates/sei-desk/src/applications/search.rs
?? docs/implementation/STAGE-5A-STABILIZATION-ACCEPTANCE.md
?? docs/implementation/STAGE-5B-ACCESSIBILITY-ARCHITECTURE-DECISION.md
?? docs/implementation/STAGE-5C-OWNER-DECISION-PACKET.md
?? docs/implementation/STAGE-5D-ACCESSIBILITY-CANDIDATE-PREFLIGHT.md
?? docs/implementation/STAGE-5E-UPSTREAM-FIRST-OWNER-DECISION.md
?? docs/implementation/STAGE-6A-DASHBOARD-RELIABILITY-REPORT.md
?? docs/implementation/STAGE-6B-LAUNCHER-ACCEPTANCE.md
?? docs/implementation/STAGE-6C-VISUAL-REFINEMENT-REPORT.md
?? docs/implementation/STAGE-6D-END-TO-END-ACCEPTANCE.md
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

No dependencies, manifests, lockfiles, source, tests, shell/taskbar behavior, Windows-key policy, accessibility provider or architecture were changed. Rust + iced, actual monitoring semantics, stable launch identities, bounded queues, persisted favorites and session-only view choices are preserved.

Remaining untested conditions include physical keyboard/held-key behavior, Narrator, real IME, DPI/mixed displays, high contrast, hardware removal/failure, a production whole-refresh failure, exact pending-operation/page-switch overlap, permanently hung native/filesystem calls, long stress/soak and crash/power-loss recovery. Controlled cases provide useful contract coverage, not proof of all those conditions. Accessibility and release gates remain unchanged and blocked independently of these usability checks.

**No new reproducible in-scope defect was found. Stop after this verification pass and await review; do not infer release readiness or start another stage.**
