# Stage 2J — Non-editable first UI feasibility

2026-10-17. **Conditional feasibility only; not approved to build.** A small opt-in catalog window could have value and avoids the unresolved editable editor contract, but its actual list, scrolling, label relationships, action delivery and manual acceptance have not been qualified. Current development posture remains headless only. Overall J verdict: **CONTINUE — bounded upstream policy defined; non-editable UI remains separately gated.** Production accessibility remains **BLOCKED**.

This assessment follows the authoritative [Stage 2I decision](STAGE-2I-ACCESSIBILITY-ARCHITECTURE-DECISION.md), its R01–R22 matrix, [gate reassessment](ACCESSIBILITY-GATE-REASSESSMENT.md) and [upstream policy](STAGE-2J-UPSTREAM-REVIEW-POLICY.md). Actual HEAD/status and preservation are in [J evidence](evidence-stage2j/README.md). No mockup, feature, native control, bridge or runtime test was added. Existing designs remain unchanged; interaction choices below are candidate acceptance proposals, not approved redesigns.

## Proposed value and strict boundary

Candidate: an ordinary user-opened decorated window that explains its purpose, shows a keyboard-navigable application list, and offers explicit Launch and Refresh, with loading/empty/error/launch-failure states, Close/Escape and predictable focus. It uses the accepted catalog/consumer snapshot identities and explicit launch requests; no selection/refresh automatically launches an application. Production services would remain off the UI thread under their existing boundaries.

Potential benefit is modest but concrete: browse a bounded catalog, refresh it on demand and explicitly launch one selected entry while getting understandable failure feedback. It could validate the future UI-to-consumer boundary without search or shell replacement. No user study or operator demonstration of this scope was conducted. Large catalogs without search may be cumbersome; no evidence yet establishes sufficient user value. The owner must decide whether to fund it as a limited utility/qualification slice, rather than treating it as the approved keyboard-first Start launcher.

Exclude editable text, search, filter fields, rename, command/argument editors, path pickers, type-ahead composition, custom taskbar, dock, dashboard, optimizer, global hotkeys, Windows-key interception, AppBar, tray hosting and shell replacement. Do not later add an editable field under the noneditable approval. Static labels/status are not editable, but still require truthful semantics and any applicable text-reading/selection patterns; “noneditable” does not mean inaccessible text. Long names/error details must remain fully available without inventing a text-range implementation or silently truncating the accessible name.

No menus, toggles or sliders are needed for the proposed minimum. Scrollbars, if present, are real controls with additional accessibility obligations. Adding any of these families reopens R19 review. No useful first scope is established by an isolated button/checkbox alone.

## What published source actually supports

Pinned source provides a plausible **partial** list path: AccessKit Role::ListBox/ListBoxOption, selected state and set position/size metadata; consumer identifies selectable containers; Windows maps those roles to List/ListItem and conditionally exposes Selection/SelectionItem. This is source verification, not an implemented iced list adapter or runtime success. Exact source hashes and excerpts are in [list-source-excerpts.txt](evidence-stage2j/list-source-excerpts.txt), [archive checks](evidence-stage2j/additional-archive-source-check.json) and [source recheck](evidence-stage2j/source-recheck.json).

Relevant published [Windows provider source](https://github.com/AccessKit/accesskit/blob/ce8164ba92995cfa86005b6259115e08c8244253/adapters/windows/src/node.rs): selection predicates at 648–710; set_selected at 980–1002; ScrollItem/SelectionItem/Selection implementations at 1370–1430. [Consumer source](https://github.com/AccessKit/accesskit/blob/ce8164ba92995cfa86005b6259115e08c8244253/accesskit_consumer/src/node.rs) identifies ListBox as a selectable container at 938–956. Do not substitute generic Role::List without checking its container predicate. Metadata positions are authored zero-based and converted by the Windows adapter; conversions/limits need tests.

**Selection safety risk:** Select/AddToSelection/RemoveFromSelection use set_selected, which may dispatch **Action::Click** when a state change is needed; an already matching state emits no action. Therefore mapping item Click to process launch would make an accessibility selection request launch an application. Candidate rules must map list-item actions to selection only and keep Launch on its separate button. Removal and single-selection semantics still need explicit qualification; generic toggling could deselect unexpectedly. Focus and selection must remain distinguishable, with no unsupported Invoke/Toggle exposed on rows. Source is a reason to demand the regression, not evidence it already passes.

**Independent scrolling gap:** search across all 12 Rust files of accesskit_windows 0.35.1 found no IScrollProvider/UIA_ScrollPatternId/ScrollPercent/SetScrollPercent implementation. ScrollItem's supported ScrollIntoView action is separate; it is not container ScrollPattern. See [search record](evidence-stage2j/scroll-provider-search.json). This is a new source-level limitation recorded without rewriting I; no runtime ScrollPattern query was run.

Microsoft's [List contract](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-supportlistcontroltype) requires selection support for selectable lists, scroll support when scrollable, and a relationship to an associated static label. Its [ListItem contract](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-supportlistitemcontroltype) separates selection, invocation and scrolling. Thus a normal scrolling catalog cannot be called qualified on these source observations alone. A visible associated label retains the already-known LabeledBy provider blocker. Do not remove a label or rename the control to avoid it.

Possible non-scrolling pagination is only an **unapproved scope alternative**: all entries must remain reachable, page/count/status and focus transitions must be usable, and no hidden truncation may conceal data. It would need a product/design decision and its own test contract; it is not selected here or a silent workaround for scrolling. If neither a supported scrolling provider nor an owner-approved independently qualified alternative exists, continue headless only. Option A may await provider improvements independently of the text-editor snapshot.

## Candidate interaction invariants to qualify

Proposed acceptance behavior, subject to the separate scope review:

- One logical single-selection list; no selection initially is permitted, with Launch disabled until an explicit valid current-snapshot selection. Up/Down/Home/End navigate/select; Tab/Shift+Tab move between the list and enabled actions. Row mouse/UIA selection never launches. Enter/Space activate the separately focused Launch button, not an incidental selection event. Escape closes only this ordinary window.
- Refresh is explicit, bounded by the consumer's policy; loading and completion status are exposed without moving focus. Stale generations cannot replace newer content. Preserve a selection only by validated identity in the current snapshot; if it disappears, clear it, disable Launch and announce the change. A retained removed row/action cannot act on a replacement identity.
- Launch requires explicit action against a valid selected identity. Disabled/pending/repeated actions have defined rejection/coalescing behavior. Success means process creation accepted under the service contract, not application readiness. Failure stays visible and announced; cancellation never claims to terminate an already started application. No arbitrary catalog targets are launched during accessibility discovery tests; future launch tests use approved harmless fixtures only.
- Keep initiating focus on refresh/error status changes. If a focused control disappears, move to an existing logical list/action target, with a documented empty/error fallback. Dismissal/close attempts external restoration only for a valid owned prior target and safe foreground conditions; never pull focus from the user's newly chosen app. Handle denied/closed targets without false success.
- Every application name, selected state, list position/count, enabled action and error/status must be available to UIA and Narrator. Avoid redundant speech on unchanged snapshots. No per-frame announcements, continuous background polling or speculative layout/bounds. Bounds/offscreen/hit-testing reflect the real rendered controls.

These invariants are not runtime evidence or permission to connect the production consumer to UI now.

## Stage 2I applicability and blockers

| I requirement | Independent of unresolved editable geometry? | Current evidence / required disposition for this candidate |
| --- | --- | --- |
| R01 names/roles/values | Yes, for list/actions/status | Historical bridge button/checkbox success only. Need actual correct-PID/HWND raw/control list tree, names, states and selection container |
| R02 relationships | Yes; still applicable | LabeledBy mapping absent. Any associated visible list label requires its true reference; provider compatibility blocker remains |
| R03 keyboard/local focus | Yes | D human attestation applies to different controls. Need real physical list navigation, all action traversal, visible/UIA focus and dismissal |
| R04 external focus restoration | Yes | Unverified; explicit cross-app switching, safe denial/closed target and dismissal test required |
| R09 error exposure/removal | Yes | Historical error behavior partial. Need catalog/refresh/launch failure semantics, disappearing row/action and focus outcomes |
| R10 live events/announcements | Yes | Native emitted-event counts unverified; separate status/error event trace and actual Narrator speech required |
| R11 queue/rejection | Yes | Existing logged/dropped bridge actions unsuitable as qualified delivery. Need bounded truthful rejection, no false success/duplicate launch or UI-thread waits |
| R12 oversized/invalid input | Yes for actions/metadata, text SetValue excluded | No editable value write here, so its 4096-byte path must not be exposed. Long/malformed catalog names, status sizes, invalid IDs and bounded error payloads still need tests |
| R13 async/provider lifecycle | Yes | Headless generations do not guarantee UI tree/layout correctness. Need stale geometry/selection/actions, checked IDs, reactivation, removed nodes and close-under-load checks |
| R14 high contrast | Yes | Unverified. Actual candidate surfaces, list selection and focus need measured visibility/readability; no approved design changes here |
| R15 reduced motion | Yes | No required animation in candidate. Still test policy/query/reactivation and absence of unwanted transition effects; local checkbox history is insufficient |
| R16 DPI/displays | Yes | Existing current-DPI resize only. Candidate list bounds/offscreen/focus need nondefault DPI, then mixed scale/negative coordinates/display changes before release |
| R18 results/list/status | Yes as a distinct list family | Source supports constituents; actual list not implemented/tested. Selection/count/positions, empty/loading/error, removal and scrolling qualify independently |
| R19 used controls | Yes | Buttons/static text/list plus scrollbar only if selected; ScrollItem source does not supply container ScrollPattern. Menus/toggles/sliders excluded; adding them needs separate qualification |
| R20 physical keyboard/Narrator | Yes | Current candidate UNVERIFIED; require per-step input and heard speech evidence, not old attestation or synthetic/UIA substitution |
| R21 Windows/resources | Yes | Exact supported Windows builds undecided; host NT 10.0.26300.0 captures do not qualify Windows 10/11 families. Need approved build matrix and idle/hidden/AT-attached/stress evidence |

R05–R08 editable value/range/caret geometry and R17 editor IME composition do not apply **only while no editable/composition-capable control or text-editing claim exists**. They remain blocked for the production editable launcher; this is scope isolation, not a waiver. Unicode names and keyboard layout compatibility still apply to static/list content. Do not infer real IME validation from that exclusion. R22's deferral remains headless-only. Any independent noneditable UI needs a new approval, not reinterpretation of that deferral.

The list/action path is technically separable from editor geometry, but shares provider, focus, event, display and human-test blockers. It is not a demonstrated ready-to-integrate slice on the pinned stack. It does not authorize taskbar/dock/global activation or later shell work.

## Qualification proposal and measurable exit

Owner first decides product value, label/scroll presentation, target Windows builds and available physical-keyboard/Narrator operators. Then separately approve an **isolated non-editable control-family qualification experiment**, preserving the accepted spike and all current source/pins unless an exact separately reviewed change is authorized. No experiment is built in J.

Proposed experiment sequence: fixed nonpersonal rows in an ordinary window, real list selection and separate buttons/status, no application launching initially. Qualify correct provider structure/patterns, selection-only Click mapping, disabled/stale/removed actions, scrolling/offscreen and geometry; observe actual native events, then physical keyboard/Narrator and display/contrast/motion/focus behavior. Stop immediately on unsupported required relationships/ScrollPattern or silent action loss; do not patch around it or use native duplicate controls. Retain historical 29-check baseline tests separately instead of claiming a new list test replaces them.

Every future result must carry source/binary/dependency hashes, PID/HWND, Windows build/scale, exact action modality, expected/actual result and raw failure/cleanup records. Selection action tests must assert **zero launches**; future explicit-launch tests separately assert exactly one approved fixture request/outcome. Race/stale/overload cases must not rely on sleeps as proof. Provider APIs and actual event emission/speech are distinct results. No mandatory test may be inferred from service acceptance.

After all applicable Entry requirements pass, obtain an independent scoped UI-entry review before building the catalog feature. Before release, rerun against actual catalog contents/state integration on approved Windows/display/AT configurations, including refresh/selection/launch races, safe failure feedback, long names/large catalogs, process/provider cleanup and resource measurements. Missing operators or supported-build hosts keep the relevant gate closed. A successful fixed-list experiment alone cannot authorize the editable Start launcher.

## Decisions and recommendation

Owner must decide whether browsing without search has enough value, name accessibility/Windows reviewers, approve the exact noneditable scope and qualification experiment, resolve provider label/scroll limitations through Option A, and approve supported Windows/AT modalities. No Option B/C is approved, and no native control/framework replacement is proposed for this scope.

Recommend preserving this as a **conditional independently testable proposal**, while implementing only already-authorized headless work. If provider gaps cannot be resolved within the finite upstream policy, or the owner rejects the limited product value, keep headless-only scope and return to the dated owner checkpoint. Do not present a pagination idea, historical checkbox success or a new release number as acceptance.

Only documentation, official read-only research and integrity checks ran; see [evidence](evidence-stage2j/README.md). No build, GUI, UIA, physical keyboard, Narrator, real IME, environment change, runtime implementation, dependency/design change, commit or push.
