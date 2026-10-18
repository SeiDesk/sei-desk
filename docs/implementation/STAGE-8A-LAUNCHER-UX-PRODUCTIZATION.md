# Stage 8A — Launcher UX Productization

2026-10-18, Asia/Calcutta. Completed one focused pass and corrected one reproducible empty-results recovery defect. Rust + iced, the companion/manager ownership model, local Ctrl+L, existing palette and unrelated behavior remain unchanged. **Production accessibility remains BLOCKED.** No commit or push.

## Starting and final repository state

Repository: `X:\Project\SeiDesk\sei-desk`. Starting/final branch: `main`. Starting/final HEAD: `af7db2f0a9b1fc2540071a3d2f41c9f4ca7c139c`.

Starting complete `git status --short`: **empty — clean working tree**. Read the Stage 7A, 7B, 7B.1, 7C, 7C.1 and 7C.2 records and current launcher, search guard, selection/feedback, routing and tests. Their historical observations are context, not independently requalified here.

The brief's `docs/implementation/SEI-DESK-UI-UX-SPEC.md` is absent. Existing `docs/design/SEI-DESK-UI-UX-SPEC.md`, especially §8, and approved `docs/design/mockups/03-start-launcher-states.png` remain design authority. Root PRODUCT.md/DESIGN.md are absent; none was invented. The incumbent pale manager and slate/mint popup were retained.

Saved hashes for all 743 baseline tracked/nonignored files under ignored `target/stage8a-ccff256ea20a47b7b76eb0637fe34c07/hashes.json`; `status.txt` records the zero-length starting short status. Final comparison: exactly the two intended source/test paths changed, with 741 original paths byte-identical. Earlier reports, evidence, mockups, source outside the fix, manifests, lockfiles, Git attributes and scripts were preserved. No reset, clean, restore, stash, staging, commit or push occurred.

## UX audit and narrow change

| Area | Finding/disposition |
| --- | --- |
| Empty/no-match recovery | Confirmed defect. §8 specifies Enter with no results has no action and retains useful guidance. The popup instead generated “Select a current application before launching.” After choosing All applications, that stale message remained beside a valid selected result. Corrected at the no-key submission branch. |
| Search focus, typing and clearing | Existing focused search, native editing, query bounds and clear-query selection reconciliation remain. The new regression checks that empty Enter preserves query/focus, and clearing recovers a valid selection. Live automated checks confirmed these states. No new shortcut or search behavior added. |
| Selection and Enter affordance | Existing semibold selected name, mint outline, separate Enter cue and bounded reveal remain. No further defect established in this focused pass. |
| Hierarchy, contrast and long names | Existing slate/mint palette, compact spacing, clipped title/full hover tooltip and readable selected metadata retained. No cosmetic token, typography, geometry or artwork change warranted. Prior narrow-render and contrast evidence is historical; no new broad contrast/DPI qualification claimed. |
| Dismissal/reopening | Existing visible Dismiss and Escape routes retained; current automated live run exercised both and explicit reopening. No shell or ownership change. |
| Real failure/Retry | Preserve service errors, failed launches and exact-key explicit Retry. Empty-results Enter is not a launch failure. The existing error-clearing regression now uses a valid selection with unavailable-service delivery, retaining meaningful failure coverage. |

### Reproduction and before/after

1. Use a new isolated profile with no favorites and open the companion.
2. Press Enter on empty Favorites. **Before:** a selection error replaces the result-count feedback, despite useful no-favorites guidance already being present.
3. Choose All applications. **Before:** valid selected results appear while the obsolete selection error remains.
4. **After:** Enter without a current key returns without mutation or service submission. Empty-state guidance and focus remain; browsing to applications shows the normal count/selection. A nonempty no-match query likewise retains its query and guidance after Enter.

Production change: `crates/sei-desk/src/desktop.rs:601`, within `Desktop::action`, removes only the two-line local-error assignment from the `model.key() == None` branch. Existing selection validity, busy/pending guards, composition handling, service provenance and window lifetime logic are unchanged. No worker, queue, allocation strategy or dependency was added. Actual valid-target delivery and launch failures retain their existing feedback.

Test changes: `crates/sei-desk/src/desktop/tests.rs`:

- Added `enter_without_results_preserves_empty_state_and_browse_recovery` (line 46). It exercises empty Favorites, repeated Submit, All applications recovery, nonempty no-match Enter and clear-query recovery. It asserts no local error/pending launch, retained search focus/query, valid recovered selection and unchanged manager query/key. This regression failed before the fix at `local_error.is_none()` and passed afterward.
- Updated the existing query-clears-error test to `changing_query_clears_local_launch_delivery_error` (line 34). A valid selection with no service handle creates a real local delivery error, then a new query clears it. The old test manufactured its error by pressing Enter with no results, which intentionally changes in this correction. No failure coverage was removed merely to obtain a pass.

## Validation commands and actual results

Used existing locked/offline dependencies with `CARGO_INCREMENTAL=0`. Flags below are the exact constraints added to the requested Cargo checks; no dependency installation occurred.

| Command | Result |
| --- | --- |
| `cargo test -p sei-desk --locked --offline enter_without_results` | Before fix: failed, 0 passed/1 failed, assertion on obsolete local error. After fix: 1 passed. |
| `cargo test -p sei-desk --locked --offline changing_query_clears_local_launch_delivery_error` | 1 passed |
| `cargo test -p sei-desk --locked --offline desktop::tests` | 10 passed, including existing window routing/lifecycle, stale-callback and actual companion widget Enter tests |
| `cargo test -p sei-desk --locked --offline actual_search_widget` | 1 passed; existing manager widget modifier/repeat policy |
| `cargo fmt --all` then `cargo fmt --all --check` | Passed; hash comparison found no unrelated formatting changes |
| `cargo check --workspace --locked --offline` | Passed |
| `cargo test --workspace --locked --offline --quiet` | 137 passed, 0 failed, 6 ignored: 45 service + 74 desktop + 18 Windows. Five existing native opt-in tests and one fixture-render test remain ignored by default; not newly passed here. |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Passed |
| `cargo build --workspace --release --locked --offline` | Passed; rebuilt for the current UI verification |
| `git diff --check` | Passed; LF/CRLF conversion notices only. New report whitespace checked separately. |

Broad validation used ordinary filesystem access for the existing atomic preference fixtures. No administrator privilege/UAC, dependency download or new environment workaround was used. Test groups overlap; focused counts must not be added to the workspace total.

## Current live evidence and review

**No human manual qualification was performed in Stage 8A.** The following are actual running Windows UI observations driven by Computer Use/Sky automation, not physical-keyboard, Narrator or shell qualification. Earlier Stage 7C.1/7C.2 human reports were read but not independently repeated or upgraded.

Evidence root: `X:\Project\SeiDesk\sei-desk\target\stage8a-ccff256ea20a47b7b76eb0637fe34c07` (ignored, uniquely named; earlier artifacts untouched).

- `before-empty-enter.jpg`: pre-fix empty Favorites shows the erroneous selection message.
- `before-browse-recovery.jpg`: pre-fix valid selected applications still show that error.
- `after-empty-enter.jpg`: rebuilt app retains empty-state feedback and focused search after Enter.
- `after-browse-recovery.jpg`: All applications recovers selected results/count without stale feedback.
- `after-no-match-enter.jpg`: query `zzzz-no-app` and approved no-results guidance remain after Enter.
- `after-clear.jpg`: Ctrl+A/Backspace recovers a valid selection with focused search.
- `after-reopen.jpg`: Escape closed only the companion; manager local Ctrl+L opened a fresh popup with initial empty Favorites/search focus. Visible Dismiss also closed only the popup.
- `before-runtime.log`, `after-runtime.log`: each isolated manager session returned exit code 0 and recorded `dashboard_workers_joined`. Separate `before-profile/` and `after-profile/` LOCALAPPDATA directories protected actual user preferences.

No application was launched during these checks; catalog entries were only displayed/selected. The pre-fix manager/popup IDs were 1312800/196626. The rebuilt manager/popup were 2097810/198856; after Escape, reopening created popup 329926. Settled inventories confirmed popup-only closure before manager exit.

A fresh read-only Impeccable finish reviewer returned **SHIP**, scoped only to this correction, after examining source, regression and six complete 580×440 before/after captures. No material finding or visual expansion was requested. A read-only documenter confirmed that palette, typography, layout and manager architecture remain unchanged and the behavior now matches §8; no design-system files were written. No web detector ran for the native surface. A fresh narrow-layout capture was not required for an unchanged layout; historical narrow-render evidence remains historical.

## Exact files and final status

Modified:

- `crates/sei-desk/src/desktop.rs` — no-results Submit no longer invents a selection error.
- `crates/sei-desk/src/desktop/tests.rs` — new recovery regression and retained query-clears-delivery-error coverage.

New: `docs/implementation/STAGE-8A-LAUNCHER-UX-PRODUCTIZATION.md`.

Starting complete short status: empty. Final complete `git status --short`:

```text
 M crates/sei-desk/src/desktop.rs
 M crates/sei-desk/src/desktop/tests.rs
?? docs/implementation/STAGE-8A-LAUNCHER-UX-PRODUCTIZATION.md
```

Branch/HEAD remain `main` / `af7db2f0a9b1fc2540071a3d2f41c9f4ca7c139c`. All 741 unrelated original files are byte-identical to this stage's baseline. No stage, commit or push.

## Limitations and recommendation

This pass fixes empty-results recovery only. Existing artwork/plate-parity differences, full keyboard access to clipped names, numeric work-area/mixed-DPI/fullscreen qualification, real IME, high contrast and production semantic accessibility remain outside this correction. Visual clarity and synthetic event tests do not establish Narrator/UI Automation support. **Production accessibility remains BLOCKED.**

Recommended next stage, subject to owner review: a bounded human confirmation of the corrected empty-Favorites/no-match/browse/clear sequence in this build, while retaining the broader platform/accessibility gates. No new feature, architecture change, shell integration or automatic next stage is proposed or started. Stopped after Stage 8A and this report.
