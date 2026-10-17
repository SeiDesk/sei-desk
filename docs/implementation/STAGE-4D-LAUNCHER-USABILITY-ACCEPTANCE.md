# Stage 4D — Launcher usability acceptance

2026-10-17, Windows x64/MSVC, Rust 1.99.0, iced 0.14.0.

Starting and final branch: **main**. Starting and final HEAD: **f9f334bfb65bb57a6afc2047d47baad4738df52e**. No commit or push.

## Preflight and preservation

Read the Stage 4A–4C reports, complete existing diff, Applications model/view, Consumer adapter and service contracts, keyboard events, shutdown, approved specification and plate 06. There was no existing application-configuration convention. The catalog provides executable/source metadata and icon path/index references, but the enabled iced stack has no supported icon extraction/decoding path; the explicitly permitted text-first tile fallback is used without dependency changes.

Stage 4A–4C and the existing sei-apps dependency addition were already uncommitted. Starting status, tracked diff, source copies and SHA-256 hashes were recorded under ignored target/stage4d-baseline/. Final hashes confirm all prior files except README.md, main.rs, view.rs, applications.rs and its tests.rs are unchanged. In particular, all manifests and Cargo.lock retain their starting bytes, as do the Stage 3A/3B services/tests, Stage 4C runtime adapter/tests, monitoring, Windows metrics/network, traffic/history/chart, validation harness, tools, approved design assets and historical reports. No existing tests were deleted or weakened; traversal expectations were extended for the new controls.

## Files and delivered behavior

| File | Purpose |
| --- | --- |
| applications.rs | Pin-aware filtering and stable reconciliation between available and missing saved identities. |
| applications/browser.rs and browser/tests.rs | Sort/group projection, session presentation, pin transactions, reading-order navigation, selection offsets and focus order. |
| applications/identity.rs | Deterministic SHA-256 token of the service identity, tested against published empty, abc and multi-block vectors. |
| applications/preferences.rs and preferences/tests.rs | Versioned bounded file format, recovery, lock/conflict checking and atomic replacement. |
| applications/preferences/worker.rs | One bounded background file worker, one-slot latest-state publication and owned shutdown. |
| applications/reveal.rs | Local widget operation using measured bounds to reveal keyboard focus, with geometry tests. No renderer added. |
| applications/tests.rs | Existing fixtures gain their corresponding identity token. |
| main.rs | Pin-state subscription and owned worker, browser actions, extended Tab path and guarded focus reveal. |
| view.rs | All/Pinned, View/Sort/Group controls, pin actions, text tiles, group headings and unavailable-pin details. Overview retained. |
| README.md and this report | Usage, persistence policy and actual acceptance evidence. |

Source paths above are relative to crates/sei-desk/src/ unless otherwise stated.

Consumer remains the sole authority for actual targets and launches. The UI never builds a command from a pin label, search query or executable display path. Existing operation IDs, generation rejection, cancellation, pending-launch guard, captured input/composition handling and owned joins are retained. Process acceptance now explicitly says that window opening is not verified; a separate visible-window observation is recorded below.

## Persistence contract

Production location: **%LOCALAPPDATA%\SeiDesk\pins-v1.txt**. The base must be an absolute per-user path; missing LOCALAPPDATA produces read-only favorites feedback. No runtime configuration is placed in the repository or installation directory.

The strict UTF-8 schema starts with `sei-desk-pins-v1` and a newline. Each record is a 64-character lowercase hexadecimal SHA-256 token, a tab, the display label, and a newline. Limits: **600,000 file bytes, 256 pins, 1,024 UTF-8 bytes per label**. Empty/control-containing labels, invalid/duplicate tokens, excess records, invalid UTF-8 and incomplete records are rejected. Reads take at most the byte limit plus one before rejecting oversize data. The format uses no serialization dependency.

The existing service ID includes normalized executable path, argument text and working directory. Saving it directly would persist launch material. Instead the token hashes that complete stable identity; only the token and label are saved. A changed target/argument/directory becomes a different token, rather than inheriting an old pin by display name or index. Digests are identity references, not commands or an application-readiness guarantee. No paths or arguments are duplicated into favorites.

Missing file is normal first run. Valid pins reload on worker startup. Malformed v1 data is preserved in a create-new, flushed `pins-v1.invalid-<pid>-<slot>` recovery copy before new pins can be saved; up to 32 backup slots are tried without overwriting an existing backup. If backup fails, the original remains read-only. Unrecognized headers/versions, oversize and unreadable files remain untouched and read-only, with guidance to resolve the problem and restart. Unsupported versions are not guessed or silently migrated.

Saving validates first, takes a create-new sibling `.lock`, and checks that current disk bytes match the worker's last loaded/saved bytes. Another instance or external edit therefore causes a visible conflict instead of silently losing its pins. The worker writes a create-new same-directory `.pending` file, flushes it with sync_all, closes it, and calls Windows MoveFileExW with REPLACE_EXISTING and WRITE_THROUGH; it never deletes the valid target first. Owned temporary/lock files are removed on ordinary success/failure. Native replacement failure leaves the previous valid file intact. An interrupted process may leave `.pending` or `.lock`; those files are not silently deleted by another instance and block saves with recovery feedback. This is not a claim of exhaustive power-loss durability on every filesystem.

UI pins change only after the worker confirms a successful save. A failed save reports its error and retains the previous confirmed set in memory and on disk. Pin actions are disabled while loading, read-only or awaiting an acknowledgement, preventing overlapping UI transactions. Available catalog labels use current metadata; unavailable pins retain their last saved label and can be removed. Favorites are not automatically pruned when an application disappears. Reappearing exact identities regain their pin and selection; no missing entry can produce a valid launch key.

One file worker waits on a capacity-one command channel, with at most one active write plus one queued command. The normal UI allows only one pending save. Publications coalesce into one complete latest state with monotonic revision; stale receipts cannot overwrite newer state. No polling loop or thread-per-pin was added. The owner is dropped after iced exits, queues Stop behind accepted work and joins the exact worker. Blocking native filesystem calls can still delay that join; no hard shutdown deadline is claimed.

## Projection and keyboard rules

Search remains case-insensitive over actual display name and executable filename, with the existing 256-character query and 8,192-entry service bounds. It composes with the Pinned filter, grouping and sorting without changing authoritative entries.

Sorting cycles Name A–Z, Name Z–A, Executable, Pinned first. Stable identity breaks ties; missing executable filenames have deterministic empty-string order. Grouping cycles Ungrouped, Pinned/Other applications, and the real user/shared Start menu source. Groups retain their order while descending names reverse within a group. Missing source metadata has an explicit fallback in pure tests; no publisher/category is fabricated. Missing pins appear in a separate Unavailable pins section of Pinned; lacking current executable/source metadata, they use saved label/token ordering and are never launchable. Empty catalog, no search matches, no saved pins and unavailable saved pins have distinct explanations.

View switches List/Grid. Tiles use actual names and source/pin labels, two columns at compact widths and four from 1,000 DIP; names truncate in results and remain complete in details. No fake icons or remote downloads are used. These presentation/sort/group/filter choices reset on restart; only pins persist.

Tab/Shift+Tab traverse navigation, search, filter, view, sort, group, results and enabled actions. Enter/Space activate the focused control; Enter in search/results launches only a current valid identity. View/Sort/Group activate to cycle explicitly labeled choices. Up/Down follow flattened reading order in either presentation and clamp at boundaries, including missing pin rows. Left/Right remain text-editing keys; no spatial grid handler was added. Escape clears transient search/selection without closing the dashboard. Local widget bounds reveal offscreen focused controls/results, with generation guards; metric updates do not steal focus.

The independent finish review found one material issue: asynchronous unpin could reorder the selected item outside the results viewport. The correction scrolls only the inner results to the reconciled current selection after a confirmed pin-set change. It preserves action focus and outer scroll; stale/error-only pin publications do not trigger the scroll. A regression test exercises deep reorder and selected-pin removal in both presentations. The verdict pass scored this fix **resolved**, disposition **ship**, at that correction's scope.

The read-only Impeccable documenter compared the specification, plate 06 SVG/PNG, Stage 4A–4C reports, baseline/final source and six runtime captures. It found no material design-consistency blocker: palette, type hierarchy, pale surfaces, mint selection and slate focus remain intact. Approved design assets and historical reports were preserved. The text-first grid is the authorized fallback; icon artwork, spatial grid navigation, context menus and custom pin ordering remain outside this stage. PRODUCT.md, DESIGN.md and .impeccable were absent before this work; no unrelated design-document migration was performed. The skill's context engine was unavailable, so the checked-in context was read directly. No HTML/CSS detector was applied to native iced source.

## Commands actually executed

Process-local CARGO_INCREMENTAL=0 avoids unsupported incremental hard links on this workspace drive. No lint suppressions or test exclusions were added.

| Command/check | Final actual result |
| --- | --- |
| cargo fmt --all --check | Pass, exit 0. |
| cargo check --workspace --all-targets --locked --offline | Pass, exit 0. |
| cargo test --workspace --locked --offline | **117 passed, 5 intentionally ignored**: 45 ordinary service, 56 application, 16 Windows-library tests. |
| cargo clippy --workspace --all-targets --locked --offline -- -D warnings | Pass, exit 0. |
| cargo build --workspace --release --locked --offline | Pass, exit 0. |
| cargo build -p sei-apps --release --examples --locked --offline | Pass, exit 0. |
| cargo test -p sei-apps --locked --offline -- --include-ignored | With SEI_APPS_FIXTURE set to this checkout's built release launch_fixture.exe, outside filesystem sandbox: **50 passed**, none ignored/failed (47 library, one consumer, two launch integration). Overlaps ordinary service tests. |
| git diff --check, baseline hashes and final protected-file review | Pass; existing dependency changes preserved byte-for-byte, not introduced here. |
| Actual release dashboard --diagnostics | Pin/save/restart/reload, keyboard/control composition, list/grid, compact scrolling, real Consumer launch and normal owned shutdown observed. |

There are **21 added tests**: nine persistence, two file-worker, eight browser, one identity-vector and one focus-geometry test. Existing Stage 3A–4C tests remain. They cover first run/reload, pin/unpin, stable reordering, missing removal/reappearance, strict schema and limits, backup/read-only recovery, foreign lock/temp preservation, native replacement failure retaining old data, atomic cleanup, concurrent-instance conflicts, failed-save receipts, mailbox coalescing and shutdown flushing, sorting/grouping/search composition, session preference races, list/grid navigation and missing launch-key rejection. Persistence tests use isolated temporary directories and never mutate actual user favorites or the Start menu.

Initial development outcomes are not hidden: one new selection assertion expected the wrong identity after filtering; it was corrected to the deterministic current identity. Seven native persistence tests initially failed inside the filesystem sandbox because MoveFileEx returned Windows error 5; unchanged native behavior passed outside that sandbox. The first outside-sandbox retry was not executed because automatic approval review hit a usage limit; the user's resume allowed a later reviewed retry, which passed. A Python edit briefly introduced non-UTF-8/newline damage in view.rs; it was repaired and the unaffected Overview/rail source was checked against the preflight copy. Strict Clippy found a dead helper, chunk iteration suggestions and a test clone; all were fixed without allowances. Final commands after the selection-visibility correction all passed. Logs are under ignored target/stage4d-final-quality.log and stage4d-service-tests.log.

## Windows runtime observations and limits

The exact workspace release executable loaded **78 supported entries and 83 source issues**. Started with no production pins file. Through ordinary UI actions, pinned CMake (cmake-gui), confirmed a token/label-only file under C:\Users\swani\AppData\Local\SeiDesk, closed normally and restarted. Pinned filtering then showed the same CMake identity with Unpin selected enabled. Grid, source grouping, descending-name grouping and search were exercised together; Up/Down and Tab/Shift+Tab/Enter changed selection or controls without an accidental launch.

Explicitly clicked Launch selected on the reloaded CMake pin. Consumer reported accepted PID **4544**; a new CMake 4.3.4 window appeared and was activated and visually confirmed. No project was configured/generated and no CMake fields were edited. The independent launched window was left open. The UI accurately reports process acceptance separately from this manually observed window evidence.

Verified 760 × 520 client controls, measured focus reveal, internal scrolling, two-column grid and a maximized four-column grid. Actual pin/unpin reorder was repeated in list and grid after the reviewer correction; CMake remained visible with action focus retained. Removing the selected last pin under Pinned was repeated in both modes, producing no selection and disabled launch. Temporary CMake/Word verification pins were unpinned through the UI, restoring the original empty pin set; only the valid header remains in the new configuration file. No Start menu entries were created or changed for Stage 4D.

Returning to Overview showed changing CPU/memory, disk capacity, live receive/transmit rates, a 30-interval traffic graph, 76% plugged-in battery and independent clock. All three owned Stage 4D executions closed normally; final reviewed execution exited 0 and logged dashboard_workers_joined after joins. Runtime logs: target/stage4d-runtime.log, stage4d-final-runtime.log, stage4d-reviewed-runtime.log. Selected screenshots are in this chat's visualization directory with stage4d-* names, including pins-reloaded, visible-launch, minimum-controls/results-focus/grid, grid-wide, list-unpin-reconciled, pinned-removal and overview-live.

Missing pins, malformed/unreadable/oversize/future configurations, write failures and concurrency conflicts have isolated tests rather than manipulations of installed apps or actual user files. Abrupt power loss/process termination, blocked native calls, full disk, unusual filesystem semantics, very long sessions, other Windows builds, full DPI/high-contrast/locale/IME/Narrator/UIA matrix remain unverified. The production accessibility gate remains unresolved. The retained validation harness's tests pass, but its full legacy UIA observer was not rerun. No global hooks/hotkeys, Windows-key interception, shell replacement, Explorer modification or privileged optimization was introduced.

## Git status

The following was the starting status. The final status is identical except for the added Stage 4D report line shown last; modifications/new files inside the already-untracked applications directory are detailed above. Both branch and HEAD remain as recorded at the top.

```text
 M Cargo.lock
 M README.md
 M crates/sei-desk/Cargo.toml
 M crates/sei-desk/src/main.rs
 M crates/sei-desk/src/view.rs
 M crates/sei-windows/src/lib.rs
 M tools/Test-Stage2A.ps1
?? crates/sei-desk/src/applications.rs
?? crates/sei-desk/src/applications/
?? crates/sei-desk/src/monitor.rs
?? crates/sei-desk/src/traffic.rs
?? crates/sei-desk/src/traffic/
?? crates/sei-desk/src/traffic_chart.rs
?? crates/sei-desk/src/validation.rs
?? crates/sei-desk/src/validation_view.rs
?? crates/sei-windows/src/metrics.rs
?? crates/sei-windows/src/network.rs
?? crates/sei-windows/src/network/
?? docs/implementation/STAGE-4A-DASHBOARD-ACCEPTANCE.md
?? docs/implementation/STAGE-4B-NETWORK-MONITORING-ACCEPTANCE.md
?? docs/implementation/STAGE-4C-APPLICATIONS-ACCEPTANCE.md
?? docs/implementation/STAGE-4D-LAUNCHER-USABILITY-ACCEPTANCE.md
```
