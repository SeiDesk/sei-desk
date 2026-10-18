# Stage 6B — Launcher usability and reliability

Assessed 2026-10-17 in `X:\Project\SeiDesk\sei-desk`, Windows x64/MSVC. **One reproduced input-routing defect fixed.** This is a narrowly scoped reliability result, not accessibility qualification or release approval.

## Baseline and scope

Starting and final branch: `main`. Starting and final HEAD: `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`.

Complete entry `git status --short`:

```text
 M README.md
 M crates/sei-desk/src/applications/preferences.rs
 M crates/sei-desk/src/applications/preferences/tests.rs
 M crates/sei-desk/src/main.rs
 M tools/Test-Stage2A.ps1
?? docs/implementation/STAGE-5A-STABILIZATION-ACCEPTANCE.md
?? docs/implementation/STAGE-5B-ACCESSIBILITY-ARCHITECTURE-DECISION.md
?? docs/implementation/STAGE-5C-OWNER-DECISION-PACKET.md
?? docs/implementation/STAGE-5D-ACCESSIBILITY-CANDIDATE-PREFLIGHT.md
?? docs/implementation/STAGE-5E-UPSTREAM-FIRST-OWNER-DECISION.md
?? docs/implementation/STAGE-6A-DASHBOARD-RELIABILITY-REPORT.md
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

The entry inventory covered 727 tracked/nonignored untracked files. All existing changes were retained, including Stage 5A recovery limits/tests and selection scrolling. No reset, clean, restore, stash, staging, commit or push was performed. Read the Stage 4C, 4D, 5A and 6A reports and the launcher model/view, browser projection, preferences store/worker, coordinator, Consumer, native launch contracts and their tests.

The owner clarified criterion 6: **“Preserve session-only view choices; audit favorites persistence.”** View, sort, grouping and filter choices therefore remain session-only under the existing Stage 4D contract. Favorites alone persist. This stage adds no persistence policy or architecture decision.

## Inspected paths and acceptance evidence

Paths below are repository-relative. Existing tests were retained; the new real-widget regression is identified separately.

| Criterion | Data path and evidence | Finding |
|---|---|---|
| Search/clear and visible selection | `crates/sei-desk/src/applications.rs`: `edit`, filtering and selection reconciliation; `applications/browser.rs`: projection/offset; `main.rs`: Query/Clear/catalog-generation scrolling. Existing query/reorder, pin reconciliation and reveal tests passed. Runtime search and Escape retained the selected Android Studio row visibly. | No additional defect established; preserve Stage 5A scrolling fix. |
| Empty/boundary/changing results | Model `move_selection`, browser reading order and identity reconciliation. Existing `keyboard_boundaries_empty_results_and_unicode_query_bounds`, both-presentation navigation and changing-query tests passed. | No new defect established. |
| Enter and shortcuts | `view.rs`: actual search TextInput submit callback; `main.rs`: `local_event`, `apps_action`, composition and focus policy. Real-view regression reproduced modified/repeated Enter bypass below. Existing captured-input, composition and Tab/focus tests passed. Runtime Escape cleared the query. | One verified defect fixed. |
| Refresh/cancellation | `applications/runtime.rs`: sequenced commands, latest frame, cancellation entry receipt; model rejects stale revisions; `crates/sei-apps/src/consumer.rs`: operation IDs and generation checks. Existing tests cover cancelled late data, pre-entry cancellation race, stale generations, failed refresh retaining the last catalog and out-of-order completions. | No new defect established. |
| Failures and UI responsiveness | Model error mapping preserves Windows code and gives user-facing guidance; native error mapping includes access-denied/elevation-required errors. Coordinator dispatches service calls to owned worker lanes. Error, panic, backpressure and failed-save tests passed. | No new defect established. A native API that never returns is not proven bounded in time. |
| Favorites recovery | `applications/preferences.rs`: strict bounded format, lock/expected-content conflict check, synced temporary file and atomic replacement; `preferences/worker.rs`: confirmed save receipts and one-slot publication. Existing round-trip, interrupted write/lock, replacement failure, malformed/future/oversized file, multi-instance conflict and Stage 5A bounded recovery tests passed. | No new defect established. Session-only view choices intentionally preserved. |
| Identity/launch target | UI carries a stable ID plus catalog generation; runtime validates against the current Consumer snapshot. Consumer `request_launch` freezes the selected request before dispatch. Pins store identity tokens, not commands. Tests cover removed targets, stale selection, pending selection changes, duplicates and missing pins. | No silent fallback target found in these paths. No executable-content authenticity guarantee is claimed. |
| Bounds/lifecycle | Catalog limits, query cap, fixed pin/file/recovery limits, bounded channels, coalesced frames and owned shutdown reviewed. Queue-capacity, undrained publication, cancellation and join tests passed; GUI exited with `dashboard_workers_joined`. | No new unbounded queue or detached work defect established. |

## Verified defect and correction

The application's global keyboard handler rejects Ctrl/Alt/Windows-modified Enter and auto-repeat. However, pinned `iced_widget` 0.14.2 `src/text_input.rs` handles Enter in its `Widget::update` key branch by publishing `on_submit` and capturing the event without checking those conditions. The search field registered that callback. Its captured event never reached the global handler's ordinary activation path, so the global policy alone could not protect search submissions.

`tests::actual_search_widget_respects_modified_and_repeated_enter` in `crates/sei-desk/src/main.rs` constructs the actual `view::content`, lays it out with the existing renderer, focuses the real `applications-search` widget, sends modifier/key events through its widget tree, then also evaluates the global event path. It counts Submit/Activate messages without launching applications. Before the correction, plain Enter, Ctrl+Enter, Alt+Enter, Logo+Enter and repeated Enter produced **[1, 1, 1, 1, 1]**, failing the required **[1, 0, 0, 0, 0]**. The same regression now passes.

New `applications/search.rs` wraps only the existing search TextInput and captures modified/repeated Enter before its submit callback. It delegates the original widget's state tag/state, layout, focus operations, remaining events, drawing and mouse interaction. `view.rs` retains its text, ID, styles and normal submit callback. Keeping that callback preserves direct search submission rather than depending on an asynchronously observed focus value. No visual tokens, geometry, shortcut policy, shell behavior or provider were changed. The guard adds no worker, queue, allocation per event or polling.

Stage 6B exact changed paths:

1. `crates/sei-desk/src/applications.rs` — declare the search module.
2. `crates/sei-desk/src/applications/search.rs` — narrow event guard (new).
3. `crates/sei-desk/src/view.rs` — wrap the existing styled search input.
4. `crates/sei-desk/src/main.rs` — add the real-view regression; earlier Stage 5A scrolling changes retained.
5. `docs/implementation/STAGE-6B-LAUNCHER-ACCEPTANCE.md` — this report (new).

## Commands and actual outcomes

Cargo commands used process-local `CARGO_INCREMENTAL=0`, locked/offline existing dependencies. Persistence and wider tests ran with normal filesystem access because the restricted environment has a known Windows atomic-replacement fixture limitation; no UAC/admin GUI action or test weakening was used.

| Command/check | Result |
|---|---|
| `cargo test -p sei-desk --locked --offline actual_search_widget_respects_modified_and_repeated_enter` | Initial test setup failed compilation with E0277 (task-level focus helper used where a widget operation was required); corrected the test helper. Then RED: one failure, actual [1,1,1,1,1]. After the guard: GREEN, 1 passed, 58 filtered out. |
| `cargo test -p sei-desk --locked --offline applications::` | PASS: 37 passed, 22 filtered out. |
| `cargo fmt --all` | Applied Rust formatting to the new guard/wrapper/test. |
| `cargo fmt --all --check` | PASS, exit 0. |
| `cargo check --workspace --all-targets --locked --offline` | PASS, exit 0. |
| `cargo test --workspace --locked --offline` | PASS: 120 passed, 0 failed, 5 intentionally ignored; includes 59 dashboard/model/worker tests and 16 Windows tests. Doc tests 0. |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | PASS, exit 0. |
| `cargo build -p sei-apps --release --examples --locked --offline` | PASS, exit 0. |
| Set `SEI_APPS_FIXTURE` to resolved workspace `target/release/examples/launch_fixture.exe`; `cargo test -p sei-apps --locked --offline -- --include-ignored` | PASS: 47 unit + 1 Consumer Windows + 2 Windows launch tests = 50 passed, none ignored. Only the controlled fixture is launched by these tests. |
| `cargo build --workspace --release --locked --offline` | PASS, exit 0. |
| Workspace `target/release/sei-desk.exe --diagnostics` | Opened Applications, observed catalog 78 supported entries with 83 reported source issues; typed `n` (54 results, visible selection), Escape cleared to 78 results retaining visible identity. Closed with Alt+F4: exit 0, `dashboard_workers_joined`. No application launched or favorite changed through the GUI. |
| `git diff --check` | PASS. Git's LF-to-CRLF notices are not whitespace failures. |

The fixture and focused runs overlap workspace tests; counts must not be summed as unique tests. Synthetic widget events and automated GUI input do not establish physical-keyboard, real-IME or Narrator acceptance. Captures are local review artifacts `stage6b-search.png` and `stage6b-cleared.png` under `C:/Users/swani/.codex/visualizations/2026/10/17/01a14960-e87d-70c1-8db4-20816aebcadc/`; runtime log is ignored `target/stage6b-runtime.log`. The normal window was inspected; no new compact-size, DPI or exhaustive visual qualification is claimed. Native review used no web detector. Fresh read-only finish review returned **ship**, limited to this input correction, with no material fixes.

Two inspection-only commands also needed correction: PowerShell did not expand a wildcard source path supplied to rg, and an abbreviated Stage 4D report filename did not exist. Directory-based rg and the actual `STAGE-4D-LAUNCHER-USABILITY-ACCEPTANCE.md` path were then used. These were not product/test failures.

## Resources, limitations and preservation

Catalog traversal is capped at 8,192 nodes; query at 256 Unicode scalars; favorites at 256 entries with 1,024-byte labels and a 600,000-byte file limit. Existing invalid-file recovery remains bounded to 32 slots with identical-byte reuse. Coordinator commands are channel-capacity 4 plus the sender reservation (tested maximum 5); latest publications coalesce to one frame. Consumer owns at most the refresh and launch lanes. Favorites use one file worker, one queued command and one latest state; accepted saves finish before its worker is joined after UI exit. Stop/cancel and owned joins prevent deliberately detached work. These are memory/work bounds, not a promise that arbitrary blocking Windows or filesystem calls have a hard deadline.

No new defect was established in persistence, generation/cancellation reconciliation or identity capture. Remaining risks needing a concrete reproduction include native calls that never return, hostile/external filesystem changes between validation and process creation, long-running stress, device/IME variation and unavailable physical operator checks. The narrow regression proves message routing for the tested event combinations, not all possible input methods.

The original entry hash inventory was session-memory evidence and was unavailable after continuation; this report does not claim a fresh byte-for-byte comparison against that lost inventory. Preservation was checked against the recorded entry status, Stage 5A/6A reports and final source diff: only the five Stage 6B paths above were edited/added here; the other pre-existing tracked modifications and reports/evidence were left in place. A durable pre-report SHA256 inventory of 728 tracked/nonignored untracked files was saved in ignored `target/stage6b-pre-report-integrity.json`; final comparison found **0 differences across all 728 files** after writing this report and review. Manifests and lockfiles have no Git diff. Historical reports, evidence, service code, tests and Stage 5A fixes were not rewritten. The read-only documentation review found no Stage 6B design-system drift and required no design-record edits.

Final `git status --short`:

```text
 M README.md
 M crates/sei-desk/src/applications.rs
 M crates/sei-desk/src/applications/preferences.rs
 M crates/sei-desk/src/applications/preferences/tests.rs
 M crates/sei-desk/src/main.rs
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
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

Recommended next step: review the narrow correction and this evidence; obtain physical keyboard/IME operator evidence separately when available. No further stage, dependency change, provider experiment, redesign, native-shell change, commit or push was performed. Production accessibility remains **BLOCKED** under the existing owner decision.
