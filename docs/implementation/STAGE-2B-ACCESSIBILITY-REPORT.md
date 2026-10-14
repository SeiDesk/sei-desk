# Stage 2B — accessibility integration investigation

14 October 2026. Baseline: `7aced5c64c3658b9fbd5d8b3c42dae46e1af1929` on `main`. Read with the [Stage 2A findings](STAGE-2A-ACCESSIBILITY-REPORT.md), [architecture](STAGE-2A-ARCHITECTURE.md), [Stage 1 risks](STAGE-1-RISK-REGISTER.md), [test matrix and procedures](STAGE-2B-TEST-MATRIX.md) and [new evidence](evidence-stage2b/README.md).

## Decision: gate remains blocked

No suitable maintained **iced 0.14.x semantic integration** was found in the released interfaces and upstream candidates inspected. AccessKit itself has a maintained, released Windows provider; these are different findings. The existing validation window still exposes no client controls in actual Windows UIA inspection. Stage 2B does **not** pass.

The task explicitly says: “If no suitable maintained integration exists, stop before introducing an unreviewed fork. Present the alternatives and their trade-offs.” This investigation follows that stop condition. No bridge, fork, dependency, production feature or replacement GUI has been introduced. No new semantic regression tests are claimed: there is no new runtime implementation to test. Existing 11 tests and the Stage 2A procedure are rerun instead. The required reports, reproducible verification steps and separate evidence are provided even though the implementation objective is blocked.

This is not proof that a project-owned integration is impossible. A fork-free bridge using released AccessKit and iced custom widgets is a plausible alternative, but it would be new accessibility infrastructure owned by Sei-Desk, not an existing maintained iced adapter. Its window-creation ordering, layout synchronization, thread ownership and text semantics require a separately reviewed approach. Merely attaching a provider to the HWND would not close the gate.

## Research and exact versions

Research performed on 14 October 2026 using official upstream pages, released crate documentation, and the locally cached source matching Cargo.lock. Research is not execution of AccessKit in Sei-Desk. No AccessKit candidate was installed, compiled or runtime-tested. Versions below are researched candidates, **not new tested dependencies**.

| Component | Version / license | Maintenance and compatibility finding |
| --- | --- | --- |
| Existing iced | `iced 0.14.0`; `iced_core 0.14.0`, `iced_widget 0.14.2`, `iced_winit 0.14.1`, `winit 0.30.13`; iced MIT | Exact running stack from Cargo.lock. Official release page still lists 0.14.0 as latest. The resolved normal tree has no AccessKit dependency; released Widget/Operation APIs do not supply native accessibility semantics. [Releases](https://github.com/iced-rs/iced/releases), [source/license](https://github.com/iced-rs/iced/tree/0.14.0) |
| AccessKit semantic model | `accesskit 0.25.1`; MIT OR Apache-2.0, with Chromium-derived portions under a BSD-style license | Released model for nodes, identities, updates, actions and text positions; it does not discover iced widgets. [Versioned crate](https://docs.rs/accesskit/0.25.1/accesskit/), [license notice](https://github.com/AccessKit/accesskit/blob/main/README.md#license) |
| Windows provider | `accesskit_windows 0.35.1`; same AccessKit licensing notices | Released 25 September 2026; published MSRV 1.87 is below this workspace's declared 1.88. Actual compatibility with this app is untested. Exposes an application-supplied tree through Windows UIA. [Release notes](https://github.com/AccessKit/accesskit/releases/tag/accesskit_windows-v0.35.1), [provider docs](https://docs.rs/accesskit_windows/0.35.1/accesskit_windows/) |
| winit adapter | `accesskit_winit 0.34.1`; same AccessKit licensing notices | Released 25 September 2026. Declares `winit ^0.30.5`, `accesskit ^0.25.1`, Windows adapter `^0.35.1`; the winit range admits locked 0.30.13. Dependency compatibility alone does not give access to iced's owned event loop or widget semantics. [Crate metadata](https://docs.rs/crate/accesskit_winit/0.34.1) |

Active releases establish provider maintenance, not conformance or a commitment to support this app. Before adopting any candidate, verify its exact package licenses/transitive notices and lockfile resolution; no license audit of an installed AccessKit dependency set was performed.

### Upstream iced integration candidates

- [Issue #552](https://github.com/iced-rs/iced/issues/552) remains open. It requests native widget hierarchy, properties and actions; the issue itself is not an implementation.
- [PR #3111](https://github.com/iced-rs/iced/pull/3111) remains a draft. Its description limits demonstrated widget support to Button/Text and describes a macOS counter example. It modifies core/runtime integration and discusses dependency patching. That does not establish Windows input, checkbox, error or text-selection support on this lockfile.
- [PR #1849](https://github.com/iced-rs/iced/pull/1849) remains a draft work in progress, not a released iced 0.14 adapter.
- [PR #3281](https://github.com/iced-rs/iced/pull/3281) was closed on 14 March 2026 without merging. The author describes continued work in a vendored fork and limitations in text character positions. That fork has not been reviewed, adopted or tested here. An upstream discussion of a fork is not approval to import it.

No inspected merged change or released integration closes this exact version's gap. This is a bounded research conclusion, not a claim to have audited every third-party repository or all future upstream work. Dynamic upstream pages can change; recheck at the next decision.

### Released integration boundaries

[AccessKit's Windows SubclassingAdapter](https://docs.rs/accesskit_windows/latest/accesskit_windows/struct.SubclassingAdapter.html) can attach through HWND subclassing without an iced fork. Its documented constraints matter: construct on the window-owning thread **before first show/focus**, keep its non-Send/non-Sync owner there, allow action callbacks from other threads, and raise the events returned by updates. The current app obtains its HWND in an Opened task after startup; reusing that unchanged is not a validated pre-show attachment strategy.

[accesskit_winit's Adapter](https://docs.rs/accesskit_winit/latest/accesskit_winit/struct.Adapter.html) needs construction before visibility and delivery of each native winit event before application handling. iced's application subscription receives translated iced events, not that complete native stream. `window::run` exposes a raw-window interface, not an arbitrary winit event-loop proxy or a semantic tree. Using this adapter therefore requires a new lifecycle integration, rather than adding one dependency and assuming it works.

Local released source review found:

- `Widget::operate` and `Operation` permit custom traversal with layout rectangles. They can support project-owned semantic wrappers; they do not automatically publish roles, Invoke/Toggle actions or label relationships.
- Button traversal exposes a container and its content. Checkbox traversal exposes label text, without a semantic toggle node. Inferring these controls from their labels would be brittle and would omit real enabled/focus state.
- `operation::TextInput` exposes visible text and cursor/selection mutation operations but no current selection getter. Its visible text can be a placeholder. It is insufficient to claim a synchronized UIA caret/range/text layout implementation. Actual model value must remain authoritative; text range support needs widget-level state and shaping evidence.

Hashes and excerpts of the exact cached files are in [local-source-review.txt](evidence-stage2b/local-source-review.txt). Relevant official tagged source: [Widget](https://github.com/iced-rs/iced/blob/0.14.0/core/src/widget.rs), [text-input operations](https://github.com/iced-rs/iced/blob/0.14.0/core/src/widget/operation/text_input.rs). Some version-specific web source URLs failed retrieval; no claim relies on unread source from those URLs.

## Alternatives and trade-offs

| Alternative | Benefit | Cost / acceptance requirement | Decision |
| --- | --- | --- | --- |
| Wait for a released, maintained upstream iced semantic bridge | Lowest ongoing divergence; retains Rust + iced | Delivery date unknown; must still verify exact widgets, Windows UIA and Narrator on a new locked release | Preferred maintenance outcome; production UI stays blocked |
| Project-owned, fork-free bridge using released `accesskit_windows`, custom iced semantic wrappers and local messages | Retains pinned iced and limits scope to this window; no second GUI | Sei-Desk owns layout/focus/action synchronization, pre-show attachment and text behavior. Must establish lifecycle and UI-thread delivery first; selection may require a custom input implementation | Smallest plausible active experiment if this maintenance cost is explicitly selected; not validated or implemented here |
| Reviewed, pinned iced fork or upstream backport | Could reuse broader widget integration | Divergence, security/update work, license/provenance review and Windows qualification; draft/demo success is insufficient | Requires explicit justification and approval under this task; no fork adopted |
| Native UIA provider written from scratch | Full provider control, still Rust | Own COM lifetimes, threading, patterns, event ordering, text ranges and recovery; duplicates maintained AccessKit infrastructure | Poor minimum approach; not recommended |

A second GUI framework/WebView and invisible duplicate native controls are outside scope. Fixed pixel rectangles or a detached fake tree would misrepresent the actual rendered controls. Do not implement them as a shortcut.

## Requirements for a future project-owned bridge

These are review criteria, not implemented behavior or a change to the existing architecture.

Keep the existing model authoritative. Assign fixed semantic identities to Window, Sample label/input, Validate, Motion, feedback, conditional Dismiss and Close; preserve identities across updates and do not reuse removed IDs for unrelated controls. Associate label and input explicitly. Derive bounds from the current iced layout, convert using current client origin/scale, and invalidate on movement, resize, DPI change and conditional-content changes. Do not independently reproduce view geometry in native code.

Keep native adapter ownership/drop and subclass lifecycle in `sei-windows`. Semantic wrappers and action-to-model mapping belong with the iced UI. Callbacks must use a bounded, nonblocking delivery path to the existing UI thread, validate window-generation/node existence on receipt, and never wait for the UI thread while it could be answering UIA. Initial queries need a valid snapshot or the adapter's documented deferred mechanism, not a synchronous UI round trip. Publish immutable snapshots after actual layout/model changes; no continuously polled tree or duplicate editable model.

Expose only verified patterns: input name/value/editability and real focus; button name/enabled/Invoke; checkbox name/enabled/checked/Toggle. Route actions through the same validation/dismiss/toggle logic as user input. Publish feedback changes without moving focus, and prove the selected notification/live semantics with both an event listener and Narrator. Remove Dismiss with its parent update and move focus only when the removed control held focus. Reject stale actions after removal/close, clear the provider on HWND destruction, and avoid callbacks retaining the app forever. If selection/caret geometry cannot track the real input, explicitly omit unsupported patterns and assess whether that limitation prevents Narrator operation.

Before any broader tree implementation, prove pre-show adapter construction, UI-thread action delivery and teardown in the same ordinary validation window. Retain standard titlebar behavior and the existing guarded external-focus policy. Do not add global input hooks, shell resources, background services or a different GUI framework.

## Actual verification and remaining limits

Environment: Windows NT `10.0.26300.0`, x64/MSVC, Rust/Cargo `1.99.0`, tiny-skia release renderer. This host is not qualification for Windows 10/11 release families or mixed-DPI configurations. New runs use the unchanged executable hash `9E4BD7E527819CD7573783880C0350C89C9FE334559F897BBEBFCC042EB8AC4A` and existing `tools/Test-Stage2A.ps1`; previous evidence is untouched. Runtime scripts use ordinary session rights, not RunAs/UAC.

The [test matrix](STAGE-2B-TEST-MATRIX.md) distinguishes actual UIA tree inspection and lifecycle from unperformed provider actions, physical interaction and speech. The normal run reconfirmed zero Edit, CheckBox and Validate nodes, with the selected root HWND matching iced's diagnostic. Missing nodes prevent meaningful Value/Invoke/Toggle tests; provider interactions are blocked, not successful. Native WindowPattern.Close is available but is not a client Close/Invoke pass. Error-fixture results and resource observations are retained separately.

All three new runs independently confirmed iced's actual HWND, exited with code 0 and left no owned child. Raw and control views each contain only the native Window with Window/Transform patterns. The error image was visually inspected: Validate's local focus border and the inline error/Dismiss action are visible, yet neither is exposed semantically. Every close trace reported native foreground restoration Denied. The keyboard run's first guarded SendInput pair returned `0`, expected `2`, and stopped remaining input; the cause is unestablished and no elevation retry occurred.

| Release regression case | Observed idle duration | App CPU delta | End working set / private commit, bytes |
| --- | --- | --- | --- |
| Normal, input focused | 30.008959 s | 0 s at counter resolution | 18,194,432 / 4,718,592 |
| Seeded error, logical Validate focus | 30.013163 s | 0 s at counter resolution | 18,362,368 / 4,739,072 |
| Keyboard injection blocked, input focused | 30.000677 s | 0 s at counter resolution | 18,423,808 / 4,759,552 |

Method remains Stage 2A's 3 s settling followed by process TotalProcessorTime/Stopwatch and refreshed memory counters. No measurable CPU increment in these short samples is not a zero-CPU guarantee. These samples cannot estimate future provider overhead, Narrator-attached performance or hidden-window behavior.

Formatting, check, all 11 existing tests, strict all-target Clippy and release builds passed. The initial sandboxed formatting command failed with Access denied; the documented filesystem exception retry passed, without elevating runtime input tests. Build hard-link/cache warnings are retained. No new accessibility integration or semantic unit tests were fabricated to turn these baseline gates into a Stage 2B pass.

Narrator speech/operation, physical Tab/editing, error announcements, stale accessible-node behavior, non-default DPI, high contrast and external focus switching remain unvalidated. Stage 2B cannot pass without the essential Narrator test even if a future UIA tree passes. The recommended next stage is an accessibility approach decision followed by the same validation-window experiment; production shell development remains closed.

## Change and preservation summary

Added this report, the Stage 2B test matrix/instructions and `evidence-stage2b/` only. No existing architecture, code, Cargo manifest/lockfile, test tool, design or prior evidence changed; all 120 inspected tracked files match their pre-edit SHA256s. The evidence index includes the preservation result. Work remains uncommitted on `main`. The previous Stage 2A documents' uncommitted-status wording is historical and has not been rewritten; their baseline was committed as `7aced5c` before this stage.
