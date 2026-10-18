# Stage 7B.1 — Launcher UX Refinement & Internal Acceptance

## Result

Completed focused launcher refinement and internal acceptance checks on 2026-10-17. The approved plate 03 and existing UI/UX specification remain the design authority. No management-window redesign, application-icon infrastructure, dependency, shell behavior or accessibility-provider change was introduced. **Production accessibility remains BLOCKED.** This is not release approval or full mockup/platform qualification.

## Baseline and preservation

Repository: `X:\Project\SeiDesk\sei-desk`. Starting/final branch: `main`. Starting/final HEAD: `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`.

Read Stage 7A and 7B reports, plate 03, relevant specification sections, current launcher/desktop implementation, existing catalog/model and service tests. Prior implementation and accessibility decisions remain intact. Root PRODUCT.md and DESIGN.md are absent; the existing specification and approved mockup were used directly, without inventing or repairing design-system records.

Before editing, saved complete `git status --short` and SHA-256 hashes of all 737 existing tracked/nonignored files to ignored `target/stage7b1-baseline/status.txt` and `hashes.json`. The initial tree was the Stage 7B final tree, including its untracked source/reports/evidence. Final comparison shows exactly three existing paths changed by this stage and 734 original files byte-identical. The three paths were extended narrowly; prior Stage 7B tests and behavior remain. All nine inventoried Cargo manifests/lockfiles, manager entry/view, preferences, monitoring code, earlier fixes, historical reports, approved artwork and evidence are unchanged from this stage's baseline. No reset, clean, restore, stash, stage, commit or push occurred.

## Design audit and disposition

| Area | Evidence and disposition |
| --- | --- |
| Search prominence/spacing | Existing 15 DIP search with 12 DIP padding remains prominent; no defect warranting a redesign. Preserve. |
| Selected title/hierarchy | Plate 03 explicitly uses semibold selected names and an Enter cue on the active row. Stage 7B lacked both, leaving keyboard intent in the distant footer. Added selected Segoe UI semibold and a reserved 38 DIP Enter cue; unselected names remain regular. |
| Selection indicator | Existing mint outline plus slate-blue fill is clear. Preserve the outline rather than changing it to the plate's rail. Text weight and Enter cue now supplement color. |
| Artwork | Catalog projection does not supply usable decoded application artwork; extraction would require additional infrastructure. Deferred under the explicit scope exclusion. No fabricated icons or approved-asset replacement. |
| Favorites | Preserve confirmed shared favorites as the incumbent list, with Favorite metadata and All applications control. Grid/icon arrangement is visual drift, not a demonstrated functional defect. No persistence or search changes. |
| Empty/no-match | Preserve truthful no-favorites/catalog-unavailable/loading distinctions. Replace abbreviated no-match copy with the approved “No applications found” and “Try a different application name or check spelling.” |
| Launch failure/retry | Source showed generic feedback above results and ordinary Submit could retry a rejected launch implicitly. Plate 03 requires explicit inline Retry. Add a selected-result failure panel with keyboard-reachable Retry launch, retaining query/selection. Bind it to the attempted identity AND generation. Ordinary Enter from Search/Results does not retry that failed target; Tab to Retry then Enter or an explicit click does. |
| Refresh failure | Existing Refresh is the correct bounded recovery operation; label it Retry refresh after a failed refresh. Preserve known catalog entries. |
| Compact layout/long names | A fixed 58 DIP row previously contained an unconstrained wrapping title truncated only by character count. Wide/long names could consume secondary-text space at narrow widths. Constrain title paint to an 18 DIP clipped column, keep the Enter cue separate, and expose the full title in a bounded wrapping hover tooltip. No layout-dependent identity conversion. |
| Narrow failure panel | Controlled 360×300 rendering exposed Retry below the initial viewport. Put the Retry action first in the inline panel and reveal that panel when keyboard focus reaches Retry. Detail text remains scrollable; no action beyond the viewport is required to recover. |
| Palette/dismissal | Preserve opaque slate/mint palette, visible Dismiss · Esc, ordinary Z-order and existing Escape/window lifetime policy. No new focus restoration or global input route. |

## Implementation and regression evidence

`crates/sei-desk/src/launcher.rs` contains the selected typography/cue, bounded title/tooltip, no-match copy, inline retry and refresh-retry label. It records the last accepted launch key locally and exposes Retry only for a matching failed current key with no pending launch. Its short Tab order includes Retry only while applicable; the existing scroll operation reveals the panel.

`crates/sei-desk/src/desktop.rs` routes Retry through the same launch service and pending/busy guard. A Retry message captured before a query, selection or catalog-generation change cannot launch a different target. Submit cannot implicitly retry the same failed target. If the selected result changes or failure becomes inapplicable, focus is reconciled from Retry to Results. No second coordinator, queue, worker or persistent state was added. The only added per-popup state is one optional stable key; the Tab-order collection is bounded to six items. Existing window-ID/lifetime/focus-generation routing and accepted-work ownership remain authoritative.

Focused regression coverage:

- `applications/runtime/tests.rs::companion_failed_launch_requires_explicit_identity_checked_retry`: actual controlled service returns a missing-target failure. Query/selection survive; ordinary Submit/Results activation does not issue another request; Tab reaches Retry; moving selection removes Retry focus; a saved Retry after a different query is rejected; explicit Retry on the retained matching target issues exactly one second request; duplicate submission is blocked; success closes only the popup. Manager query remains unchanged. No real application is launched by this test.
- `launcher/tests.rs::long_title_does_not_paint_over_secondary_text_or_outside_its_column`: renders actual iced title/tooltip widgets with short and long names, selected and unselected. Pixel equality below the title verifies that the long title cannot paint into secondary text. No mirror of a character-count implementation is used as the assertion.
- Existing laid-out widget/event-path test retains `[1,0,0,0,0]` submit counts for plain, Ctrl, Alt, Logo and repeated Enter. Existing popup isolation, old callback, geometry-ticket, pending-launch dismissal and failed-refresh replacement tests remain green.

An opt-in ignored fixture-render test uses the actual production launcher view and existing iced tiny-skia headless renderer. It produces seven controlled states at two sizes: results, long title, no-match, launch failure, loading, refresh failure and empty favorites at 580×440 and 360×300. Retry-focused captures apply the same scroll operation used by production. These are layout/state evidence, not observed Windows failure cases. They do not change the catalog or launch anything. RGBA files were encoded losslessly as PNG with the existing Python/Pillow environment; no new dependency was added.

## Validation commands and actual results

Commands used `CARGO_INCREMENTAL=0`, locked/offline Rust dependencies. Final broad validation followed the last Retry-focus correction.

| Command | Actual outcome |
| --- | --- |
| `cargo test -p sei-desk --locked --offline desktop::tests` | 9 passed at the initial refinement checkpoint |
| `cargo test -p sei-desk --locked --offline companion_failed_launch` | 1 passed, including final focus reconciliation |
| `cargo test -p sei-desk --locked --offline long_title_does_not` | 1 passed |
| Set `SEI_LAUNCHER_RENDER_DIR` to absolute `target/stage7b1-renders`; `cargo test -p sei-desk --locked --offline launcher_controlled_state_render_evidence -- --ignored` | 1 passed; fourteen controlled renders produced and inspected |
| `cargo fmt --all` then `cargo fmt --all --check` | Passed; hash comparison found no unrelated formatting changes |
| `cargo check --workspace --all-targets --locked --offline` | Passed |
| `cargo test --workspace --locked --offline --quiet` | 136 passed, 0 failed, 6 ignored: 45 service + 73 desktop + 18 Windows. Five existing native opt-in tests and one new opt-in rendering fixture are ignored by default. The rendering fixture was separately run as above; do not sum overlapping totals. |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Passed |
| `cargo build --workspace --release --locked --offline` | Passed after final source corrections |
| `git diff --check` | Passed; existing LF/CRLF notices only, no whitespace errors |

The first compile of the new render-test helper failed because its Arc import and explicit column macro import were missing; corrected, then focused and broad validation passed. No existing test was removed or weakened. Broad tests used ordinary filesystem access for existing atomic preference fixtures, avoiding the previously documented restricted-filesystem limitation; no admin installation or dependency resolution was involved. Existing native opt-in launch fixtures were not rerun in this refinement; their Stage 7B results are historical, not new Stage 7B.1 evidence.

## Live observations

Two final-release sessions ran with process-local `LOCALAPPDATA=target/stage7b1-profile`. This new isolated directory was seeded with the previous controlled CMake favorite; actual user preferences were untouched. Its pins file hash remained identical to the seed after testing.

1. Manager local Ctrl+L opened popup 133730. The empty query showed the confirmed CMake favorite, selected title/cue and search focus.
2. Typing `cmake`, Down and Up visibly moved selection. Before launch, the safe selected target was explicitly checked as **CMake (cmake-gui)**; the uninstaller entry was never activated. Ctrl+Enter opened no application. Plain Enter opened exactly one visible CMake GUI (395394) and dismissed the popup. CMake was closed without opening a project or changing settings.
3. Reopen created popup 460862. Reinvoking from the manager retained 460862. A no-match query displayed the approved guidance. Ctrl+A/Backspace cleared the text; Escape dismissed only the companion. Another popup 264802 was used for query/navigation without a launch.
4. A manager-after-dismissal capture returned foreign occluded content and was rejected as evidence. A targeted second session explicitly activated the manager before capture. Manager 1313324 retained its `paint` query, no-result selection and Grid presentation after popup 723070 independently searched `cmake`, moved selection and dismissed. Valid before/after captures replace the rejected observation; valid-selection isolation is also covered by the existing automated test.
5. Both diagnostic sessions exited with code 0 and `dashboard_workers_joined`. No resident process was introduced. Existing manager Escape handling remains unchanged and its automated coverage passes; no broader physical keyboard qualification is inferred.

Logs: ignored `target/stage7b1-runtime.log` and `target/stage7b1-isolation-runtime.log`. Controlled rendering artifacts: ignored `target/stage7b1-renders/`. Valid live screenshots are under `C:\Users\swani\.codex\visualizations\2026\10\17\01a14960-e87d-70c1-8db4-20816aebcadc`:

- `stage7b1-favorites.jpg`
- `stage7b1-selection.jpg`
- `stage7b1-no-match.jpg`
- `stage7b1-manager-before.jpg`
- `stage7b1-manager-after.jpg`

Fresh independent native finish review returned **SHIP** for the authorized internal refinement, reviewing three launcher live captures and fourteen controlled renders, with no material findings. A read-only design documenter confirmed the incumbent palette, typography, selection/recovery changes and remaining drift; it changed no design-system files. No web detector ran for this native surface.

Source-color calculations: selected text/metadata on selection fill ≈5.14:1; secondary text on slate ≈8.38:1; placeholder on field ≈5.40:1; inline failure text on field ≈7.33:1. These calculations and screenshots are local design evidence, not a passed production contrast/high-contrast gate.

## Remaining limitations

Application artwork, icon-grid favorites, plate's left selection rail and fuller no-match composition remain visual differences. Long titles are intentionally clipped in the compact row; the complete name is available on hover, without a claim of complete keyboard/semantic name access. At 360×300, Retry can be revealed by Tab and the failure details scroll; all result/context text cannot fit simultaneously. Ordinary launcher resizing/long-title layouts were validated through controlled renderer dimensions rather than changing physical monitor settings or claiming a live mixed-DPI test.

Loading, refresh errors, missing-target retry and pending-work closure were verified with controlled state/service tests. No real application was removed, no permissions/system settings changed and no risky failure injected into an installed application. Actual Windows focus denial, monitor removal, native launch failure during dismissal, physical key repeat, Narrator/UIA, real IME, mixed DPI, high contrast, fullscreen coexistence, Explorer restart and crash recovery remain unqualified. **Production accessibility remains BLOCKED.** No acceptance gate was bypassed or reclassified.

## Exact changed paths and final Git status

Changed relative to the Stage 7B.1 baseline:

- `crates/sei-desk/src/launcher.rs`
- `crates/sei-desk/src/desktop.rs`
- `crates/sei-desk/src/applications/runtime/tests.rs`

New:

- `crates/sei-desk/src/launcher/tests.rs`
- `docs/implementation/STAGE-7B1-LAUNCHER-UX-ACCEPTANCE.md`

All other 734 original tracked/nonignored files are byte-identical to the saved baseline. Final complete `git status --short` (older changes remain user-owned):

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
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

Stopped after this refinement/report for owner review. No Stage 7C or excluded dock/shell work began. No stage, commit or push.
