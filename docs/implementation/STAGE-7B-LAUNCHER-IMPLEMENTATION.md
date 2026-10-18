# Stage 7B — Explicit Launcher Companion Implementation

## Result and scope

Implemented the explicitly invoked internal launcher with the existing Rust + iced dependency graph. The management window remains the primary lifetime owner. No resident mode, shell changes, global input interception, dependencies, provider work, commit, or push was introduced. **Production accessibility remains BLOCKED.** This report is implementation evidence, not release approval or complete plate 03 qualification.

## Baseline and preservation

Repository: `X:\Project\SeiDesk\sei-desk`. Starting and final branch: `main`. Starting and final HEAD: `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`.

Before editing, the complete short status and SHA-256 inventory of 732 existing tracked/nonignored files were saved under ignored `target/stage7b-baseline/` (`status.txt`, `hashes.json`, and copies of the two manager entry/view files). Stage 7A, the existing Stage 4–6 implementation/reports, service ownership and locked iced/winit sources informed the implementation. Existing Stage 5 decisions and acceptance gates were preserved.

Starting short status:

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
?? docs/implementation/STAGE-7A-DESKTOP-FEASIBILITY.md
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

Final hash comparison: 724 original files unchanged; exactly eight original paths changed for Stage 7B, listed below. Existing modifications in the manager entry/view/model/search files were extended rather than replaced. Preferences, preference tests, traffic chart, README, scripts, historical reports, evidence, assets, manifests and lockfiles are byte-identical to this stage's baseline. No reset, clean, restore, stash or staging occurred.

## Architecture and ownership

`main.rs` uses iced's supported multiwindow `daemon` API, opening the manager immediately. This API name does not create a resident application: manager close/destruction explicitly exits, a three-second unconfirmed startup deadline fails closed, and event-loop return drops and joins the same monitoring/application/preferences owners. The initial attempt to use two-argument window-specific callbacks with `iced::application` failed compilation (E0593); the supported multiwindow API resolved that mismatch without dependency changes.

`desktop.rs` is the small outer window owner. It routes raw events by actual iced window ID, retains the existing manager reducer, and supplies a separate launcher model, query, selection, focus, composition state, dimensions and lifetime-specific widget IDs. Manager Escape still clears its query. The manager's named Open launcher command and local Ctrl+L deliberately invoke the companion; neither is a global hotkey. The popup is opaque, borderless, focusable and ordinary Z-order, with visible Dismiss and Escape dismissal.

There is one shared catalog/launch coordinator, one pins service and one monitoring subscription. Existing bounded search, stable launch keys, identity validation, queues and persistence remain authoritative. Popup actions send the selected current key, never a display name or index. Confirmed favorites populate the initial empty-query view; All applications and search use the existing catalog/filter semantics. Manager view preferences remain session-only. Popup interaction does not write preferences.

Service envelopes/frames carry a window-lifetime origin. Shared fresh catalog and pins deliberately propagate, but window-specific launch/refresh feedback is projected by origin. Rejected requests cannot retag an accepted pending operation. Dismissing a popup does not resend or detach accepted work; the existing owner completes it. Its old success/error cannot dismiss a replacement or overwrite manager feedback. The popup blocks duplicate submission while an accepted launch is pending.

Opening, placement, focus observations and widget callbacks check window IDs and lifetime/focus generations. A closing-window gate waits for actual destruction before replacement. Every geometry query has its own checked-unique ticket: a reproduced old-reply race after reuse previously dismissed the current popup; the regression failed before correction and passes with exact query-ticket matching. Only one automatic geometry inspection may be outstanding.

`sei-windows/src/launcher.rs` obtains the typed Win32 handle through iced's borrowed `window::run`/`HasWindowHandle` path. No opaque iced ID or `raw_id` is cast to HWND. Native inspection validates a live, owned current-process window, manager foreground/nonminimized state for invocation, monitor work area and DPI. Placement revalidates current work area and uses owned-window `SetWindowPos` with no activation or Z-order change. Geometry is physical, bounded within work area, checked for invalid/small rectangles and supports negative origins. Popup placement is reconsidered on movement/scale and the existing one-second freshness pulse while open, with no pointer polling or extra worker. Invalid geometry closes the companion and leaves an error on the usable manager.

Focus is requested once after deliberate invocation and actual observations are checked. Unconfirmed focus times out rather than repeatedly forcing focus. Manager focus loss during pending creation cancels it. Active popup focus loss does not force restoration. Successful launch/dismissal never restores a foreign foreground bookmark or focuses over the launched application.

## Presentation and concrete corrections

The launcher follows plate 03's opaque slate/mint direction while retaining the pale manager. It is a text-first internal implementation: application artwork, selected-name semibold/per-row Enter cue and dedicated Retry presentation are not complete plate 03 parity. Other differences include a mint selection outline rather than a left rail, source/favorite subtitles, abbreviated no-match copy and feedback above results. Global keyboard guidance supplies Enter instruction; retained selection can be retried with Enter and refresh with Refresh. No approved asset was overwritten. A read-only design documenter compared source and final captures against the incumbent spec; root PRODUCT.md and DESIGN.md were absent and were not invented or changed.

Final review identified selected subtitle contrast of approximately 3.78:1. Selected subtitles now use the existing light TEXT color (approximately 5.14:1 against selection fill); unselected subtitles retain SECONDARY. This local correction does not pass a production contrast gate. The reviewer inspected the rebuilt capture and returned SHIP for the authorized internal milestone, with no further material findings.

A live sequence (no-match query, Enter, then valid `cmake` query) reproduced obsolete “Select a current application” feedback beside valid results. Query edits now clear local error feedback. A focused reducer regression failed on the old behavior and passed after the correction; the final live capture confirms recovery.

## Regression evidence

Fourteen focused tests added: nine desktop/window tests, three runtime provenance tests, two pure placement tests. Existing tests were retained; the shared loaded-catalog fixture was exposed only within test builds.

Desktop coverage includes independent manager state, duplicate invocation, old-window callback rejection, stale completion projection, invalid geometry/focus timeout, focus-generation changes, manager destruction, the geometry-ticket race, query-error recovery, and the actual laid-out companion text-input plus routed event path. Plain Enter emits one submit; Ctrl/Alt/Logo Enter and repeated Enter emit zero (`[1,0,0,0,0]`). This is synthetic widget evidence, not physical keyboard qualification.

Runtime tests hold controlled launch/refresh work across popup dismissal/replacement, assert one launch request, preserve origin through busy rejection/completion, and prevent old errors/successes from affecting the replacement. Existing removed-target/identity tests remain green. Pure placement tests cover negative origins, scales, invalid and undersized work areas. Injected failures prove reducer handling; they do not establish actual Windows focus-denial, window-creation failure or physical monitor-removal behavior.

## Validation commands and actual outcomes

Commands used existing locked/offline dependencies with `CARGO_INCREMENTAL=0`.

| Command | Outcome |
| --- | --- |
| `cargo test -p sei-windows --locked --offline launcher::tests` | 2 passed |
| `cargo test -p sei-desk --locked --offline companion` | 4 passed at that checkpoint |
| `cargo test -p sei-desk --locked --offline an_old_geometry_reply` | Failed before fix; passed after fix |
| `cargo test -p sei-desk --locked --offline changing_query_clears_obsolete` | Failed before fix; passed after fix |
| `cargo fmt --all --check` | Passed after final corrections |
| `cargo check --workspace --all-targets --locked --offline` | Passed after final corrections |
| `cargo test --workspace --locked --offline --quiet` | 134 passed, 0 failed, 5 opt-in tests ignored after final corrections (45 service, 71 desktop, 18 Windows) |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Passed after final corrections |
| `cargo build --workspace --release --locked --offline` | Passed after final corrections |
| `cargo build -p sei-apps --examples --release --locked --offline`, then `SEI_APPS_FIXTURE=target/release/examples/launch_fixture.exe cargo test -p sei-apps --locked --offline -- --include-ignored` (PowerShell environment equivalent) | Build passed; 50 tests passed including controlled native launch tests; overlaps workspace tests, not an additional unique total |
| `git diff --check` | Passed; Git emitted existing LF/CRLF conversion notices, no whitespace errors |

Existing Windows preference fixtures require ordinary filesystem access for their atomic-write behavior. Tests were run with that access after the restricted environment limitation was identified; no UAC/admin installation or dependency access was involved.

## Runtime evidence and limits

All live sessions used process-local `LOCALAPPDATA=target/stage7b-profile` to protect real preferences. Safe installed CMake GUI was searched, Ctrl+Enter did not launch it, and plain Enter opened one visible CMake GUI; the popup closed. CMake was closed without loading a project or changing settings. A manager-pinned CMake favorite survived restart and populated the popup. Query changes, no-match/empty-query views, selection, All applications, Escape and visible Dismiss were exercised. Manager `paint` query and Grid choice survived popup-only search/dismissal.

Repeated invocation reused popup HWND 5114106 in the first session and 526368 in the final build. In the final build, Escape removed only 526368; reopening created 1050662. Closing manager 3606752 with the popup open returned process exit code 0, and subsequent window inventory contained neither owned window. Earlier diagnostic sessions recorded `dashboard_workers_joined` in `target/stage7b-runtime.log`, `target/stage7b-final-runtime.log` and `target/stage7b-smoke-runtime.log`; the final non-diagnostic confirmation log is empty, with exit code 0. Owner drop/join policy is unchanged and covered by existing lifecycle tests.

Useful external screenshots are in `C:\Users\swani\.codex\visualizations\2026\10\17\01a14960-e87d-70c1-8db4-20816aebcadc`: `stage7b-manager-final.jpg`, `stage7b-final-favorites.jpg`, `stage7b-query-error-reproduced.jpg`, and final `stage7b-confirmed-results.jpg`. Earlier captures precede the selected-subtitle correction; the confirmed-results capture is the final corrected popup. Logs/profile/build artifacts remain ignored under target, not committed evidence.

Not performed: physical monitor removal, actual OS focus denial/forced creation failure, real pending native launch failure during dismissal, physical key-repeat qualification, Narrator/UIA, real IME, mixed-DPI hardware, high contrast, fullscreen coexistence, Explorer restart, crash recovery, narrow physical display/extreme-title layout qualification or non-Windows compilation. Loading/error/stale/pending-close branches have controlled test evidence rather than claims of all corresponding live failures. No acceptance gate was weakened. The next step is review of this internal milestone and separately authorized platform/human qualification, not automatic Stage 7C work.

## Exact Stage 7B paths and final status

Existing paths changed in this stage:

- `crates/sei-desk/src/applications/search.rs` — generalize the existing Enter guard message type for both windows.
- `crates/sei-desk/src/applications.rs` — cloneable frame and request provenance.
- `crates/sei-desk/src/applications/runtime.rs` — bounded existing coordinator provenance.
- `crates/sei-desk/src/applications/runtime/tests.rs` — three focused completion/closed-popup tests.
- `crates/sei-desk/src/applications/tests.rs` — test fixture visibility only.
- `crates/sei-desk/src/main.rs` — multiwindow entry/ownership; preserve existing manager fixes.
- `crates/sei-desk/src/view.rs` — named manager invocation; preserve incumbent presentation.
- `crates/sei-windows/src/lib.rs` — narrow launcher module export.

New files: `crates/sei-desk/src/desktop.rs`, `crates/sei-desk/src/desktop/tests.rs`, `crates/sei-desk/src/launcher.rs`, `crates/sei-windows/src/launcher.rs`, and this report. No other nonignored file was created by this stage.

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
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

Stage stopped at implementation and acceptance documentation. No commit or push. Production accessibility remains **BLOCKED**.
