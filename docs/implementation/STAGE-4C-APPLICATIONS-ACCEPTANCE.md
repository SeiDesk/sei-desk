# Stage 4C — Applications catalog and launch acceptance

Date: 2026-10-17. Windows x64, Rust/MSVC 1.99.0, iced 0.14.0.

Starting and final branch: **main**. Starting and final HEAD: **f9f334bfb65bb57a6afc2047d47baad4738df52e**. No commit or push was performed.

## Preflight and preservation

Reviewed the complete existing diff, Stage 4A/4B acceptance reports, Stage 3A catalog/launch implementation, Stage 3B Consumer coordination, tests, dashboard navigation, monitoring channels/history and shutdown. Stage 4A/4B were already uncommitted and remain present.

The first attempt identified the missing sei-apps dependency as a blocker. At the user's resumed “re-run”, crates/sei-desk/Cargo.toml already contained the sei-apps path dependency and Cargo.lock already listed it. This implementation made **no dependency manifest or lockfile edits**. Their final SHA-256 hashes match that resumed preflight, rather than HEAD (they were already modified relative to HEAD). The resumed status and hashes are retained under ignored target/stage4c-baseline/.

Hash comparison covers the existing crates, tools, implementation reports, approved design package, root manifest, lockfile and README. Only main.rs, view.rs and README changed from those baseline files; new Applications source and this report were added. Existing monitoring, network, traffic/chart, validation, tools and Stage 4A/4B reports retain their baseline bytes. Stage 3A/3B source has no Git diff. No approved design file was changed.

## Changes and service contracts

| File | Purpose |
| --- | --- |
| crates/sei-desk/src/applications.rs | Display projection, bounded query, filtering, stable selection, explicit UI status and user-facing errors. |
| crates/sei-desk/src/applications/runtime.rs | Dedicated headless Consumer owner, bounded commands, event-driven service notices, latest-state publication, shutdown. |
| crates/sei-desk/src/applications/tests.rs | Pure search, reconciliation, bounds, stale publication and pending-launch guards. |
| crates/sei-desk/src/applications/runtime/tests.rs | Injected service, cancellation races, two lanes, errors, backpressure and lifecycle tests. |
| crates/sei-desk/src/main.rs | Applications state/subscription, local keyboard/focus routing and owned runtime lifetime. |
| crates/sei-desk/src/view.rs | Catalog search/results/details, refresh/cancel/launch feedback and focus styles; Overview retained. |
| README.md | Current feature description, keyboard behavior and report link. |

Production uses the existing Consumer with NativeService. Reused contracts include request_refresh, select/clear_selection, request_launch, start_pending, cancel, wait, snapshot/status access, begin_shutdown and shutdown. NativeService delegates to the existing sei-apps refresh and launch APIs. The adapter only adds notifications around that service boundary; it does not enumerate shortcuts or create processes itself.

The authoritative catalog and executable arguments remain inside Consumer. The UI retains one Arc display projection per generation and filtered indices, not a second launch target collection. Search uses case-insensitive display name and executable filename only; full paths, source labels and working directories are details, not searchable fields. Queries are limited to 256 Unicode scalar values and the existing service bounds catalogs at 8,192 entries. Missing icons/publisher metadata are not invented.

Selection uses the catalog's stable ID plus generation. Query/refresh reconciliation preserves a visible ID, otherwise selects the first valid result or clears selection. Up/Down clamp at boundaries. A stale generation cannot launch. A launch command supplies only that identity; Consumer resolves and freezes the original validated request. Selection changes and refresh cannot rename or redirect an in-flight launch receipt. Both UI pending acknowledgements and the service's occupied launch lane guard repeated launches.

Initial enumeration is lazy on opening Applications; subsequent refreshes are explicit. Loading, loaded, empty catalog, no matching results, refresh failure and stale command rejection are separate states. Failed refresh keeps the last good projection. Tab/Shift+Tab traverse navigation, search, results and enabled actions; Enter in search uses iced's submit path to avoid a duplicate global launch. Captured editing/modified keys and composition are guarded. Escape clears transient search/selection state without closing the dashboard. Search focus is not reset by metric or catalog updates.

## Bounded work and lifecycle

One dedicated owner thread holds Consumer, which permits at most one refresh worker and one launch worker. Enumeration, process creation, completion waits and joins happen off the UI/executor thread. The UI command channel has capacity four plus one fixed sender reservation (five queued commands); saturation returns a visible retry error. Service entry/return notices have their own bounded channel (four plus one reservation), independent of UI publication. The UI mailbox holds exactly one complete latest Frame; terminal results remain in subsequent frames until another operation replaces them. Idle work waits for events; this page adds no refresh timer or completion polling loop.

Development stress exposed a real cancellation race: cancelling before Consumer entered Service could bypass return-only notifications. The final adapter acknowledges service entry before issuing Consumer.cancel for a running refresh, and waits on Consumer's existing completion contract when a return notification precedes terminal publication. Another stress failure showed transient command errors being overwritten by entry notices; those errors now persist until a subsequent command. After both fixes, 30 consecutive runs of the then-six runtime tests passed. A seventh deterministic immediate-cancel test exercises 20 dispatch/cancel cycles. Repeated runs overlap the final suite and are not counted as unique tests.

Shutdown uses an independent atomic stop/waker, so a full UI command channel cannot prevent it. Consumer begins shutdown, cancels and joins its occupied workers on the owner thread; Owner joins after the iced event loop returns. Existing Stage 3B limits remain: cancellation is cooperative, a native call already in progress may delay shutdown, and process acceptance is not application readiness/window tracking. Launched processes are not terminated when Sei-Desk exits.

## Executed validation

Process-local CARGO_INCREMENTAL=0 avoids unsupported incremental hard links on this workspace drive. No tests or warnings were suppressed.

| Command/check | Actual result |
| --- | --- |
| cargo fmt --all --check | Pass, exit 0. |
| cargo check --workspace --all-targets --locked --offline | Pass, exit 0. |
| cargo test --workspace --locked --offline | **96 passed, 5 intentionally ignored**: 45 ordinary service, 35 app and 16 Windows-library tests. |
| cargo clippy --workspace --all-targets --locked --offline -- -D warnings | Pass, exit 0. |
| cargo build --workspace --release --locked --offline | Pass, exit 0. |
| cargo build -p sei-apps --release --examples --locked --offline | Pass, exit 0. |
| cargo test -p sei-apps --locked --offline -- --include-ignored | With SEI_APPS_FIXTURE set to this checkout's release launch_fixture.exe, outside the filesystem sandbox: **50 passed**, none ignored/failed (47 library, one consumer, two launch integration). Overlaps ordinary service tests. |
| git diff --check, baseline hashes and final diff review | Pass. Protected prior work, manifests/lockfile and service contracts preserved. |
| Actual release dashboard --diagnostics | Catalog, search, selection, refresh, fixture launch/error, compact scrolling, returning Overview and normal close observed. Exit 0; dashboard_workers_joined logged. |

There are **16 added tests**: seven pure Applications tests, seven injected runtime tests and two main/focus tests. They cover the requested search fields, Unicode/case handling, empty/no-match distinction, stable reorder/removal reconciliation, stale generation/publication rejection, failure retaining good data, invalid selection, identity during unrelated changes, duplicate launch prevention, terminal success/failure/panic, simultaneous lanes, finite queues/mailbox, cancellation and shutdown. Existing tests remain intact. During development, compile/lifetime errors and a Clippy collapsible conditional were corrected without allowances. One release rebuild initially failed with Windows access-denied because a native close had been rejected after intervening human input and the old executable was still running; normal close and a fresh rebuild resolved it.

## Windows observations

The existing native catalog loaded 79 supported entries and 83 source issues while one controlled fixture shortcut was installed (78 entries before that fixture). These are the service's supported subset, not a claim to list every installed Windows application. No fabricated entry was inserted into application state.

For the controlled real launch check, copied the existing launch_fixture example to an ignored, uniquely named target directory and created one uniquely named user Start menu shortcut with WScript.Shell. This was test setup only, not discovery code. Enter and the keyboard-focused Launch action used the normal catalog/Consumer path. Fixture output confirmed the exact stage4c-ui-acceptance argument and intended working directory. Renaming only that owned executable then produced an actual service Missing failure, Windows error 2, visibly below the controls. The selected identity remained intact. The single owned shortcut was subsequently removed after verifying its exact path and target; no other Start menu entry was changed. Ignored fixture output/logs remain as evidence.

Manual refresh retained the search and selected identity while advancing catalog generation. A no-match query showed zero matches against the nonempty catalog with Launch disabled. Escape kept the app open; Down changed selection; typing, Tab traversal and keyboard refresh/launch worked. The native text injection helper did not deliver one Unicode text attempt, so manual clipboard/Unicode input is not certified; pure Unicode filtering and query bounds are tested.

At the user's request to try another application, searched actual catalog results and launched **CrystalDiskInfo** through Consumer: the service returned AccessDenied with Windows error **740** (elevation required); no elevation prompt was automated. The finish reviewer identified generic access-denied wording as insufficient recovery guidance. The final UI explicitly explains the administrator-permission requirement and advises opening the application through Windows with appropriate permission, retaining error740. The existing mapping test now asserts this case. Rebuilt and repeated the real CrystalDiskInfo launch; the corrected message wrapped fully at normal and compact widths. After fixture cleanup this final catalog contained **78 entries and 83 source issues**. Final reviewed execution exited 0 and logged dashboard_workers_joined in target/stage4c-reviewed-runtime.log.

Selected **Microsoft Edge** from the same results and pressed Enter: Consumer reported success with accepted PID **18376**, and the UI displayed “Last launch: Started Microsoft Edge.” The window inventory did not show a new Edge window, so this is process-acceptance evidence only, not application-readiness or visible-window acceptance. An earlier current-session Brave receipt was also visible; it is not used as the controlled automated launch evidence here.

Follow-up at the user's request: verified **Notepad visibly opened** through Sei-Desk. Notepad had no entry in the supported catalog, so created one uniquely named temporary Start menu shortcut to the existing C:\Windows\System32\notepad.exe, with no arguments and that executable's directory as working directory. Selected the discovered entry in Applications and clicked Launch selected; Consumer accepted PID **20232**. Window inventory then contained a new “Untitled - Notepad” window, absent before the test; activated it and visually confirmed the blank editor. Evidence: stage4c-notepad-visible.jpg in the chat visualization directory and target/stage4c-notepad-runtime.log. Removed only that temporary shortcut after verifying its exact path and target; Notepad was left open. This verifies a visible utility launch without upgrading the earlier Edge result to a visible-launch pass. No source code or dependency changes were needed for this follow-up.

Observed normal and compact widths and a **760 × 520 client window**. At minimum size, PgDown exposes the list/details and wrapped executable, shortcut and working-directory text. Returning to Overview showed changing CPU, memory, disk, receive/transmit rates, 30 measured traffic intervals and 76% plugged-in battery; the independent clock continued. Normal Alt+F4 exited 0 and logged dashboard_workers_joined after owned joins. Runtime logs are under ignored target/stage4c-runtime.log, stage4c-final-runtime.log and stage4c-acceptance-runtime.log. Screenshots are in the chat's visualization directory with stage4c-* names.

The independent visual finish review found the incumbent type, palette, selection/focus, compact scrolling and Overview continuity preserved. Its sole material finding was the error740 guidance above; the verdict pass scored that fix **resolved**, disposition **ship**, at that correction's scope. All final quality gates were rerun successfully after the fix. No native design detector ran because it evaluates HTML/CSS rather than iced.

The read-only Impeccable documenter pass compared the incumbent specification, approved plate 06, Stage 4A/4B baseline, source and final catalog/minimum/error captures. It confirmed preserved palette, typography, surfaces and focus language; no approved design files or sidecars were changed. Stage 4C implements the bounded searchable catalog/launch flow; plate 06's icon grid, pinning and sort controls remain future scope. Existing shell differences from the broader design are retained. PRODUCT.md, DESIGN.md and .impeccable were absent before this extension; no unrelated design documentation migration was introduced.

Real empty-catalog and refresh-failure conditions were tested with injected services rather than removing installed applications or changing ACLs. Native cancellation timing, a blocked Windows call, other Windows versions, full DPI/high-contrast/locale/IME/Narrator matrix and prolonged resource qualification remain unverified. The prior validation harness was retained byte-for-byte and its nine tests pass; the legacy PowerShell UIA observer was not rerun for this stage. The production accessibility gate remains unresolved. No global hooks/hotkeys, Windows-key interception, shell replacement, Explorer changes or privileged optimization were introduced.

## Final Git status

Starting status is target/stage4c-baseline/status.txt. It contains all entries below except the two Applications source entries and this report. Final status includes preserved Stage 4A/4B work and the pre-existing dependency addition:

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
```
