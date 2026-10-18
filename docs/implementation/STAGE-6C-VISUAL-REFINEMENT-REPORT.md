# Stage 6C — Visual refinement and UI consistency

2026-10-17, Windows x64/MSVC. **Completed a narrow refinement of the existing Rust + iced management interface.** No redesign, additional feature or architecture change. Production accessibility remains **BLOCKED**.

## Baseline and authority

Repository: `X:\Project\SeiDesk\sei-desk`. Starting/final branch: `main`. Starting/final HEAD: `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`.

Read the UI/UX specification, inspected approved plates 05 and 06 at their native image size, and reviewed Stage 4A–4D, 5A, 6A and 6B acceptance reports. Inspected current `view.rs`, `traffic_chart.rs`, theme, responsive branches, launcher focus/input wiring and existing tests. No dedicated pixel-golden test suite exists; runtime captures and the existing real-widget/model tests provide the available evidence.

The specification's light lavender/blue-gray **management windows**, slate text, mint selection, restrained accents and native chrome govern these views. Its graphite treatment applies to separate desktop overlays; it does not authorize changing these windows into a dark theme. Stage 4D's text-first grid, reading-order navigation and session-only view/sort/group/filter contract remain intact. Approved artwork and historical records were not changed. Existing absent PRODUCT.md/DESIGN.md and aspirational shell/icon/context-menu features were not made a side project.

Initial full working-tree status, grouped by `git status --short`:

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

A durable SHA256 inventory of **729 pre-existing tracked/nonignored untracked files**, starting status and copies of view/main were saved before editing under ignored `target/stage6c-baseline/`. Existing files and all user work were protected.

## Concrete visual problems and changes

| Baseline observation | Narrow correction | Authority / limit |
|---|---|---|
| Enabled browsing controls, Refresh, Pin and Launch all used mint, competing with selected rows. | Launch retains mint; secondary actions use the existing panel tint, hover canvas and pressed edge tint. Disabled text remains readable. Focus borders remain independent. | Specification §§4, 11, 13: restrained emphasis and distinct control states. Actions/enabling rules unchanged. |
| Selection outline disappeared when search had focus; list and grid selected labels had ordinary weight. | Persistent 1 DIP focus-token selection outline and semibold selected name; 2 DIP border when results also hold keyboard focus. | §§5, 11, 14: mint plus outline, selected name weight, selection independent of focus. No selection logic change. |
| Search used pure white outside the established surface palette. | Panel-tinted resting field, canvas-tinted focused field, existing dark focus border, 12 DIP padding and shared 8 DIP radius. | §§4–5 and plate 06. Search ID, state, callbacks and Stage 6B guard retained. |
| Page headings were 30 DIP regular, metric values 27 DIP regular and section headings 17 DIP regular. Network lacked the other cards' label treatment. | 24 DIP semibold page headings/metrics, 18 DIP semibold section headings, aligned 14 DIP semibold metric labels. Small tint markers are consistent across all four cards. | §5 typography, plate 05 hierarchy. Existing font family retained; no downloads or font dependency. |
| Metric panels used 18 DIP padding, 205 DIP height and 14 DIP gaps, leaving generous unused space while pushing history below the normal viewport. | Shared 16 DIP padding, 184 DIP metric height and 12 DIP gaps; 8 DIP internal spacing. Normal and narrow views expose more of the history while keeping captions and real values readable. | §5 compact spacing; §10 readable responsive cards. Existing two/four-column threshold and scrolling retained. |
| The plot had only a zero baseline against its large pale panel. | One quiet half-scale guide separates the plot visually without changing bar positions, rates, peak scale, history, retention or gap meaning. | §10 readable history. Existing legend, scale text, endpoints and measured-interval count retained. No data or series added. |

Exact Stage 6C changed paths:

- `crates/sei-desk/src/view.rs` — visual hierarchy, local shared constants/helper, compact spacing and control/selection/search styling. Stage 6B's existing search-wrapper wiring remains in place.
- `crates/sei-desk/src/traffic_chart.rs` — one additional guide quad in drawing only.
- `docs/implementation/STAGE-6C-VISUAL-REFINEMENT-REPORT.md` — this new report.

No monitoring, launch, selection, preference, worker, refresh or cancellation implementation was edited. No test was removed, rewritten or weakened. No new test merely mirrors styling constants.

## Before/after visual evidence

Verified local artifact directory:

`C:/Users/swani/.codex/visualizations/2026/10/17/01a14960-e87d-70c1-8db4-20816aebcadc/`

The following filenames are relative to that absolute directory. Files contain native JPEG screenshot bytes, with matching `.jpg` extensions; no image generation or editing was used.

| State | Before | After | Captured outer window size |
|---|---|---|---|
| Dashboard normal | `stage6c-before-overview-normal.jpg` | `stage6c-after-overview-normal.jpg` | 1102 × 772 px |
| Applications normal, loaded All/List | `stage6c-before-apps-normal.jpg` | `stage6c-after-apps-normal.jpg` | 1102 × 772 px |
| Dashboard maximized | `stage6c-before-overview-max.jpg` | `stage6c-after-overview-max.jpg` | 1920 × 1080 px |
| Applications maximized, loaded All/List | `stage6c-before-apps-max.jpg` | `stage6c-after-apps-max.jpg` | 1920 × 1080 px |
| Dashboard narrow | `stage6c-before-overview-narrow.jpg` | `stage6c-after-overview-narrow.jpg` | 772 × 772 px |
| Applications narrow, loaded All/List | `stage6c-before-apps-narrow.jpg` | `stage6c-after-apps-narrow.jpg` | 772 × 772 px |
| Applications narrow, All/Grid | `stage6c-before-grid-narrow.jpg` | `stage6c-after-grid-narrow.jpg` | 772 × 772 px |

Comparisons retain the same view/query/presentation/size, not frozen sample values: timestamps, measured rates and history naturally differ between real sessions. Normal is the existing 1100 × 740 client size with native chrome. Narrow sizing used the native Size menu; a corner-drag attempt had left the window unchanged and was superseded by the verified 772 px capture. An early capture selector picked a 16 px icon; it was corrected to the largest returned screenshot and the affected baseline capture was replaced before review. Saved before/after views were checked for validity by the fresh reviewer. These are captured pixels, not DPI qualification.

Additional after-state captures: `stage6c-after-search.jpg`, `stage6c-after-empty.jpg`, `stage6c-after-refresh.jpg`, `stage6c-after-selection.jpg` and `stage6c-after-query-no-matches.jpg`. They show search filtering, no-match guidance and disabled launch/pin, refresh feedback, and keyboard selection in the narrow grid. Maximized captures include an unrelated desktop pet near the lower-right corner; blue pointer halos are automation feedback. Neither is product styling or part of the refinement.

## Interaction and regression checks

Two actual dashboard sessions used the workspace release binary with `--diagnostics`. Baseline and refined sessions both exited with code 0 and `dashboard_workers_joined`. Refined metric values and network history continued updating. Applications loaded **78 supported entries with 83 reported source issues**, displayed those issues honestly, and Refresh completed from catalog generation 1 to 2. Search `m` produced 29 results; `microsoft` produced the single matching installed app; `zzz` and `developer` produced explicit no-match states. Escape restored the catalog; Down moved selection from 64Gram to Adobe Premiere Pro 2026. List/Grid switching retained the selected identity. No GUI launch or favorite write was performed.

Existing tests confirm favorites round-trip, atomic replacement, invalid/interrupted file recovery, conflict/error handling, bounded workers, list/grid selection, stale refresh rejection and exact launch identity. The actual laid-out search-widget regression still verifies plain Enter versus modified/repeated Enter. Favorites were validated through these isolated fixtures rather than changing the user's saved pins. View choices remain session-only. The existing `Pinned ·` caption distinguishes favorites from selection; no new decorative favorite icon was introduced.

Error and unavailable/stale metric branches retain their original explicit content and semantics. Error mapping/recovery tests passed; no forced production launch error, hardware failure or stale-worker injection was performed in the screenshot session. Installed visible labels including Adobe Premiere Pro 2026 fit at narrow width; extreme 100+ character titles, translated labels and tooltip coverage were not newly demonstrated. The attempted `developer` query had no supported match and is recorded as such, not as long-name evidence. Native automation's Unicode `type_text` call did not populate the iced field; ordinary key events did. This is not evidence of physical IME behavior.

## Commands and actual results

Cargo used existing locked/offline dependencies and process-local `CARGO_INCREMENTAL=0`. Workspace tests ran with normal filesystem access for the existing Windows atomic-replacement fixtures, whose restricted-sandbox limitation was established earlier; no administrator/UAC GUI action was used.

| Command | Actual outcome |
|---|---|
| `cargo fmt --all` | Applied Rust formatting after the visual edits. |
| Initial `cargo check -p sei-desk --all-targets --locked --offline` and release build | FAILED E0277: the new metric-height constant was initially `u16`, unsupported by `Into<Length>`. Corrected it to `u32`; no dependency or behavior workaround. |
| `cargo fmt --all --check` | PASS, exit 0. |
| `cargo check --workspace --all-targets --locked --offline` | PASS, exit 0. |
| `cargo test --workspace --locked --offline --quiet` | PASS: 45 service + 59 dashboard/model/widget/worker + 16 Windows = **120 passed**, 0 failed, **5 intentionally ignored** process fixtures. Doc tests 0. |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | PASS, exit 0. |
| `cargo build --workspace --release --locked --offline` | PASS, exit 0 after correcting the constant type. |
| `target/release/sei-desk.exe --diagnostics` | Baseline and refined sessions exited 0 with workers joined. Logs: ignored `target/stage6c-before-runtime.log` and `target/stage6c-after-runtime.log`. |
| `git diff --check` | PASS. LF-to-CRLF notices are working-copy notices, not whitespace failures. |

Inspection-only rg commands using an unexpanded PowerShell wildcard and an initial `chart.rs` guess failed to find their paths; directory discovery identified the actual registry path and `traffic_chart.rs`. They were not product failures. `git diff --no-index` returned its normal difference result when comparing the protected baseline view with the edited view.

Fresh read-only Impeccable finish review returned **ship**, with TYPE/MATERIAL/GROUND matching the incumbent specification, all required viewport captures valid, and no material fix. The read-only documenter confirmed this acceptance report suffices and no approved design-record update is warranted. No web detector was applied to native Rust/iced. These handoffs are limited to this existing-system refinement; they do not authorize design migration or repair historical spec/implementation differences.

## Integrity, final status and limits

Final baseline SHA256 comparison: **only `view.rs` and `traffic_chart.rs` differ among the 729 pre-existing files; all other 727 match exactly**. This includes the complete pre-existing Stage 5A/6B source/test fixes, manifests, lockfiles, prior reports, scripts and evidence. The only new nonignored path is this report. No dependency, shell/taskbar, Windows-key policy, provider, architecture or data-semantic change occurred. No stage, commit, push, reset, clean, restore or stash was performed.

Final `git status --short`:

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

No material clipping was observed in the captured live content at these three sizes. The management window still scrolls to reach history, power and application details at shorter heights; this was preserved. Unsupported icon artwork, spatial grid navigation and shell surfaces remain outside the accepted Stage 4 implementation. DPI/mixed monitors, forced colors, physical keyboard, Narrator, real IME, localization, extreme titles, forced runtime error appearance and exhaustive intermediate-width combinations remain unqualified. This stage neither removes those limits nor changes existing release/accessibility gates.
