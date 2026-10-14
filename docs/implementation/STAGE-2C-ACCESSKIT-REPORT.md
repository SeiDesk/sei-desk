# Stage 2C — isolated AccessKit + iced feasibility

14 October 2026. Baseline `7aced5c` on `main`, with uncommitted Stage 2B work preserved. Read the [Stage 2B report](STAGE-2B-ACCESSIBILITY-REPORT.md), [matrix](STAGE-2C-TEST-MATRIX.md), [experiment instructions](../../experiments/accesskit-iced-spike/README.md) and [separate evidence](evidence-stage2c/README.md).

## Decision

**A project-owned, fork-free bridge can expose and operate these real iced controls through Windows UIA on the tested host. The complete accessibility gate has not passed.** Actual client tests exercised Value, Invoke, Toggle, error insertion/removal, focus, layout changes and normal cleanup. Physical keyboard and Narrator operation remain unavailable in this tool session and unvalidated. No production integration or shell feature was started.

Recommendation: **proceed with project-owned bridge qualification in this isolated window**, conditional on physical keyboard/Narrator acceptance before any production adoption or expansion. There is no current evidence requiring a UI framework replacement. Text editing semantics, label-property mapping and maintenance cost remain material reasons to withhold production approval. Do not expand the bridge if essential manual tests fail; retain this experiment and investigate the smallest missing accessibility strategy first.

## Released API compatibility established

Official research was rechecked on 14 October 2026; exact crates were fetched from crates.io, inspected locally, compiled and exercised on Windows. No arbitrary branch, patch or fork was used. The new standalone workspace/lockfile does not alter root dependencies.

| Component | Exact resolved version | License / status |
| --- | --- | --- |
| iced / core | 0.14.0 / 0.14.0 | MIT, released; same pinned GUI release as application |
| iced_widget / iced_winit / winit | 0.14.2 / 0.14.1 / 0.30.13 | Released transitive stack, matches Stage 2A |
| accesskit | 0.25.1 | MIT OR Apache-2.0; released, MSRV 1.87 |
| accesskit_windows | 0.35.1 | MIT OR Apache-2.0; released 25 September 2026, MSRV 1.87 |
| accesskit_consumer | 0.39.1 | Released transitive semantic consumer; AccessKit licensing |
| windows-sys / provider's windows | 0.61.2 / 0.62.2 | MIT OR Apache-2.0; separate maintained binding families coexist |

AccessKit's distribution also notes Chromium-derived portions under its BSD-style notice; retain those notices when distributing. Package metadata/licenses were inspected for the AccessKit components; this is not a complete distribution license audit of every transitive dependency. Workspace declares Rust 1.88, but only 1.99.0 was executed. Official references: [AccessKit releases](https://github.com/AccessKit/accesskit/releases), [license notices](https://github.com/AccessKit/accesskit#license), [iced release](https://github.com/iced-rs/iced/releases/tag/0.14.0).

The key lifecycle discovery is released iced `window::Settings.visible=false`, which preserves an initially hidden HWND, followed by `window::run` on the window thread and `set_mode(Windowed)` after attachment/layout publication. This satisfies [SubclassingAdapter's documented pre-show/thread requirement](https://docs.rs/accesskit_windows/latest/accesskit_windows/struct.SubclassingAdapter.html) without patching iced. Runtime trace and native checks confirm the adapter was installed while the window was hidden on its owning thread. Raw handle conversion uses iced's Win32 raw-window-handle and AccessKit's HWND wrapper. There is no reliance on a transitive accessibility feature or a late Opened attachment to an already visible window.

The Windows-specific adapter subclasses only this owned HWND. Its released implementation handles `WM_GETOBJECT`, native focus and window destruction, so a raw winit event stream is not needed here. `accesskit_winit` is neither installed as a bridge nor used to take over iced's event loop. [Released AccessKit action semantics](https://docs.rs/accesskit/latest/accesskit/enum.Action.html) and the inspected provider source map Invoke/Toggle to Click and ValuePattern to SetValue.

## What the prototype connects

`experiments/accesskit-iced-spike/` contains its own manifest, lockfile, four Rust sources, UIA client and instructions. It reuses the unchanged pure Stage 2A model by source reference, not by adding production dependencies or changing production behavior. Its real iced widgets are drawn and process mouse/text input normally; no hidden duplicate controls, fixed fake bounds or parallel renderer exists.

1. Create the ordinary decorated window hidden. A thread-affine owner installs `SubclassingAdapter` before show. Publish the first actual layout snapshot, then reveal the window.
2. Thin custom widget wrappers delegate all rendering/input and report their actual layout rectangles through an iced Operation. Scale client-relative DIP rectangles by the current native window DPI. AccessKit adds the current client-screen origin for UIA coordinates. Move/resize/rescale/focus/model events request updated snapshots; no synchronization timer or continuous polling is added to the app.
3. The model is authoritative. Derive semantic nodes, values, enabled/focus/checked state and child relationships from it. IDs 2–6 identify label/input/Validate/Motion/Close, also published as `spike-*` AutomationIds. Error/Dismiss use an instance epoch so a removed action's identity is not reused for a later error instance. Static children follow visual order.
4. A short snapshot lock supplies a full initial tree synchronously on AccessKit activation. Subsequent publications compare derived nodes and submit only changed/new nodes; removing error children updates the parent without resending removed nodes. Focus accompanies every update. Queue event raising occurs after owner/snapshot borrows are released.
5. Provider callbacks only try-send requests to a bounded futures channel (capacity 32 plus sender-reserved slots). An event-driven iced subscription receives them on the UI thread. Unsupported/removed/closed-node actions are ignored. No provider callback waits for a GUI round trip or mutates the model. Queue overload is logged/dropped; success acknowledgements under overload are not solved by this spike.
6. ValuePattern replaces the model string; Validate/Dismiss/Toggle use actual model transitions. Explicit provider Focus requests change the local real input/logical action focus. Error feedback itself does not request foreground activation or relocate focus. Dismissing a focused disappearing action returns local focus to input. No `SetForegroundWindow` or input injection is used by this experiment.
7. Normal Close routes through the UI thread, detaches/drops the subclass on its owning thread, then exits iced. Windows owns the HWND; all resources are process/window-local. No AppBar, shell registration, global keyboard hook, tray resource or persistent setting exists.

The provider needs a derived presentation snapshot; it is not a second editable state model. This deliberately single-window experiment uses a thread-local owner and one stable subscription/receiver. Neither is proposed as a general multi-window architecture.

## Actual Windows observations

Host: Microsoft Windows NT 10.0.26300.0, x64/MSVC, Rust/Cargo 1.99.0, tiny-skia release renderer. Windows 10/11 target release families, non-default/mixed DPI and other desktops are not qualified. Runtime tests used ordinary session rights, without RunAs/UAC. Build tooling needed the familiar filesystem sandbox exception; that did not elevate the UIA runtime client.

The separate Windows PowerShell 5.1 client loads Windows UIAutomationClient/Types. It starts/retains its exact child process, discovers the native Window by PID/title/type, validates HWND ownership and independently matches it to the pre-show attach diagnostic. It inspects raw/control views and selects client controls by role/name **and** `spike-*` identity, avoiding the native titlebar Close button. Client actions are real cross-process Windows UIA operations, not mock providers or synthesized application messages.

| Actual control | Observed UIA behavior |
| --- | --- |
| Sample text | Edit named Sample text; enabled/focusable; ValuePattern editable; read empty, set `sample`, read `sample`, then clear. Captured actual pixels display the updated value. No TextPattern/selection support advertised |
| Validate | Button, correct name/identity, InvokePattern. Nonempty validation keeps error absent; empty validation creates error and Dismiss. Explicit UIA Focus request reaches real action focus; validation preserves that initiating focus |
| Prefer reduced motion | CheckBox with TogglePattern; state changes Off→On→Off and real checkbox pixels/model trace agree. Local demonstration only; no OS setting write |
| Error and Dismiss | Named Text plus Button/Invoke when error exists. Dismiss removes both nodes from raw/control views. This is visible/exposed error text, not a demonstrated live announcement |
| Close | Actual client Button/Invoke, separate from titlebar Close. Process exits code 0 with provider-detached diagnostic and no remaining child |
| Geometry | Positive bounds inside actual client; native Transform resize increases real input width, restoring size restores width; move shifts screen-space bounds and restoring position restores them. Own-client captures inspected; no non-default DPI result claimed |

Retained patterns after removal and after close returned **System.Windows.Automation.ElementNotAvailableException**, HRESULT `-2147220991` / `0x80040201` (`UIA_E_ELEMENTNOTAVAILABLE`). Windows supplied the surprising message “Could not open the process token”; that wording alone is not evidence of a privilege blocker. Run 06 records the inner exception type/code; the final client asserts both. No stale request changed the model or kept the process alive in these tested sequences. This is not an exhaustive concurrent shutdown stress test.

The AccessKit model contains LabelledBy and the visible label/input names are correct. However **UIA LabeledBy is not exposed** by accesskit_windows 0.35.1; the inspected property mapping lacks that property. It is a released-provider limitation, not a reason to invent a successful relationship assertion or add an unreviewed native override. Explicit naming remains usable in the demonstrated UIA slice; Narrator acceptance is still open.

Evidence retains all runs: Run 01 exposed controls but selected native titlebar Close in its test and had incomplete label semantics; labels/IDs were corrected. Run 02 completed values/error insertion but used the wrong focus assertion (its trace contains an explicit Focus-to-Validate request before Click). Run 03 passed core action checks with weaker stale-pattern assertions. Run 04 intentionally checked LabeledBy and failed on the provider limitation. Run 05 passed move/resize and cached-pattern checks but logged only exception messages; Run 06 captured their exact UIA exception codes. The final run uses precise stale-error checks. Failures are preserved, not relabeled or overwritten.

## Resources and maintenance

Normal Windows dependency graph: **114 distinct package/version entries**, including the spike, versus **99** for the existing app. There are 20 new/different entries (including replacement app/native-crate entries and patch-version differences); this is not “20 accessibility libraries.” Full standalone lockfile resolves 240 packages across targets. Four direct dependencies, no added async runtime, service or another GUI framework. [Dependency list](evidence-stage2c/dependencies.txt).

Own Rust bridge/app/wrapper code totals 879 lines after formatting, plus the reused pure model and separate test client. Only `native.rs` contains own unsafe code: two documented blocks for live-window thread/visibility and DPI queries. AccessKit/windows bindings themselves contain unsafe platform implementation; “two blocks” is not a claim that the dependency stack is entirely safe Rust.

Run 06: 30.012277 s idle observation after a 3 s settle; CPU delta 0 s at counter resolution, working set 22,364,160 bytes, private commit 5,537,792 bytes, peak working set 22,745,088 bytes. Counters use refreshed app Process.TotalProcessorTime/working set/private memory and Stopwatch wall time; percentages use one logical core. The separate client is excluded. This is a short observation of a populated provider and exercised window, not a zero-wakeup guarantee, statistical acceptance result, ETW profile or isolated provider-overhead benchmark. Larger values than Stage 2A are not solely attributable to AccessKit: the binary/dependency graph, layout and preceding actions differ.

Final Run 07: 30.016416 s, CPU delta **0.515625 s / 1.717810% of one logical core**, working set 21,995,520 bytes, private commit 5,246,976 bytes, peak 22,667,264 bytes. All 29 UIA checks passed. Own-client screenshot captures were correctly refused because the HWND was not foreground at those capture points; use Run 06's images of the identical binary. The interval does not establish continuous foreground/focus status. Variation between samples warrants longer profiling before budgets or wakeup claims; do not silently substitute the zero-delta sample for the final nonzero result.

Lifecycle burden is manageable for this fixed window, but substantial for a general shell. Each dynamic result/menu must derive identities, roles, actions, selection, bounds and focus from real widgets; quick-control sliders need tested range semantics. The current text input lacks real text-range/caret/selection exposure. A larger shell must not maintain an independently authored tree whose structure/layout drifts from its view. This spike is evidence for a small boundary, not a completed reusable accessibility framework.

Known limits: UIA string replacement accepts at most 4096 UTF-8 bytes; overload drops requests; rapid pointer/layout/message races and provider reactivation stress are unqualified; OS high contrast/motion changes are not integrated (nothing animates); only normal exit is tested. Forced termination cannot run detach/destructors; Windows destroys this process's HWND/address space, but that path was not executed. No shell recovery resource is held. Error announcements, text selection/caret, Narrator editing, external focus switching and mixed DPI remain unverified or unsupported as detailed in the matrix.

## Quality and scope

Formatting/check, 10 spike tests (3 semantic regression tests + 7 reused model tests), strict all-target Clippy and release build pass. The unchanged root workspace's 11 tests also pass; shared model tests are counted in both executions, not claimed as 21 distinct tests. An initial Clippy style failure was corrected and retained in the gate log. No mocks stand in for the Windows action tests.

Added the standalone experiment, two Stage 2C documents and separate evidence only. All **147** pre-existing tracked/untracked files, including uncommitted Stage 2B artifacts, match their pre-edit hashes. Root architecture/production crates/lockfile and design assets remain unchanged. No experiment process remains. Nothing is staged or committed. The next milestone is **manual acceptance of this same prototype** with Narrator, physical editing/navigation, external focus and prepared non-default DPI/high contrast. Keep production development and bridge expansion closed until those essential results justify adoption.
