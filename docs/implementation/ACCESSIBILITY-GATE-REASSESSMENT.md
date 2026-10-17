# Accessibility gate reassessment and next-stage plan

2026-10-15. **Final verdict: BLOCKED.** Headless Stage 3B remains accepted. Production launcher, taskbar, dock and Windows shell integration are not approved. The current isolated bridge does not provide the mandatory programmatic text-range/caret/selection capabilities; other production checks remain unverified. P3 runner cleanup is independently repaired and tested below; it does not change the accessibility verdict.

## Repository, evidence and interpretation

Reviewed the uncommitted `main` working tree based on HEAD `e11af2cf6baea5c20e1edc7d48c603c6ab77397c`, accessibility reports/matrices/indexes, source of the AccessKit/iced spike, approved design accessibility/focus rules, and the [Stage 3B patch review](STAGE-3B-PATCH-REVIEW.md). No existing Rust source, design, accessibility spike, dependency or previous evidence was modified. No commit or push.

Evidence keys:

- **A:** [Stage 2A report](STAGE-2A-ACCESSIBILITY-REPORT.md), [evidence](evidence-stage2a/README.md): native Window-only tree; failed client exposure, denied native foreground request, limited motion query.
- **B:** [Stage 2B report](STAGE-2B-ACCESSIBILITY-REPORT.md), [matrix](STAGE-2B-TEST-MATRIX.md), [evidence](evidence-stage2b/README.md): baseline failure confirmed; released-provider research did not itself implement iced semantics.
- **C:** [Stage 2C report](STAGE-2C-ACCESSKIT-REPORT.md), [matrix](STAGE-2C-TEST-MATRIX.md), [evidence](evidence-stage2c/README.md), [run 07](evidence-stage2c/run-07-final/run.json): isolated released-component bridge, 29 actual UIA checks; LabeledBy compatibility failure retained in run 04.
- **CA:** [Stage 2C acceptance](STAGE-2C-ACCEPTANCE-REPORT.md), [evidence](evidence-stage2c-acceptance/README.md): another real UIA pass and separate synthetic keyboard observations; physical input/Narrator then blocked.
- **D:** [Stage 2D closure](STAGE-2D-ACCESSIBILITY-CLOSURE.md), [matrix](STAGE-2D-TEST-MATRIX.md), [evidence](evidence-stage2d/README.md), [UIA run 01](evidence-stage2d/uia-01/run.json), [UIA final](evidence-stage2d/uia-02-final/run.json), [manual attestation and limited approval](evidence-stage2d/manual-attestation.json).
- **S:** current `experiments/accesskit-iced-spike/src/{main,native,semantics,wrapper}.rs`, locked manifest, [spike instructions](../../experiments/accesskit-iced-spike/README.md).

D's two archived UIA runs each contain **29/29 passing assertions**, using binary SHA256 `70307E37BB8F06E11FA09EA3DF07E1D164840EDCA5C2BF12E14B8BE6D719D83F`. Their JSON explicitly says Narrator/physical input were not run by that harness. Separate human evidence says `PASS`, then `EVERYTHING WORKS` when specifically asked about both modalities and the supplied checklist. These are valid **user-attested isolated-spike results**, not assistant-heard speech, exact transcripts, or inferred UIA speech success. Keep both evidence types; neither overrides the other.

This assessment reads those historical results; it does **not** claim a fresh accessibility runtime/manual run. Fresh executions in this task concern runner regression and headless service verification only. The [new evidence index](evidence-accessibility-gate-reassessment/README.md) separates them.

## Requirement-by-requirement status

PASS means the named scope has recorded supporting evidence, not general conformance. FAIL means an actual failed check; UNVERIFIED means absent coverage; DEFERRED is the explicitly approved headless-only permission; BLOCKED means current implementation/tooling cannot complete the specified requirement.

| Requirement / exact scope | Status | Trace and limit |
| --- | --- | --- |
| Original unbridged iced application's client controls exposed | FAIL | A/B actual raw/control trees contain no Edit/Validate/CheckBox. Later spike success does not retrofit this app. |
| Released Windows provider, real owned HWND, pre-show/UI-thread attachment | PASS | C/D traces + S native install checks; isolated SubclassingAdapter, not automatic iced support. |
| Correct PID/HWND raw/control discovery, names/roles/enabled/focusable states | PASS | C/CA/D real UIA assertions for spike. |
| Input ValuePattern read/set synchronized to actual value | PASS | D UIA; whole-string replacement only. |
| Validate/Dismiss Invoke, checkbox Toggle/state synchronization | PASS | D actual cross-process actions and model observations. |
| Explicit UIA LabeledBy relationship to separate static label | FAIL | C run 04 and D closure: relation authored but absent in released provider. Correct Name is not a relationship pass. |
| Programmatic text ranges/caret/selection under headless-stage approval | DEFERRED | D explicit user approval only for headless catalog/service work. |
| Production editable input TextPattern and truthful caret/selection/range bounds | BLOCKED | S supplies value/SetValue, no text runs/selection provider. D identifies insufficient iced selection readback boundary. Cannot pass using guessed positions. |
| Physical Tab/Shift+Tab, typing/edit-in-middle, selection replacement, Validate/Toggle/Dismiss | PASS | D user-attested checklist, specific isolated binary. Not synthetic input and not IME qualification. |
| Visible focus and local restoration to Sample text after Dismiss | PASS | A/C visuals, D manual checklist/UIA/local model; not external foreground restoration. |
| Narrator essential names/roles/value/editing/actions/state speech | PASS | D user attestation only; exact utterances/audio not supplied. Must rerun after text/control changes. |
| Error exposed/removed, stale retained Dismiss rejected | PASS | D UIA including ElementNotAvailable and semantic occurrence IDs. |
| Error feedback without stealing Validate focus, repeated errors | PASS | D UIA + user-attested Narrator checklist; no exact event/speech duplication count. |
| Independent native live-region event emission/count capture | BLOCKED | D managed client could not resolve needed event/property IDs. Live::Polite source/compilation is not emitted-event evidence. |
| Late async layout/focus result rejection and after-close model protection | PASS | D 13-test gate log includes new generation/late-message unit tests; not exhaustive native interleaving proof. |
| Current-DPI bounds, native move/resize geometry | PASS | C/D UIA captures and assertions on tested host configuration. |
| Normal provider detach, stale closed input rejection, no owned child | PASS | C/D normal close/ElementNotAvailable/exit logs. |
| Foreground ownership, external app switching and prior control restoration | UNVERIFIED | A request denied; CA/D explicitly do not qualify external switching. Safe denial is permissible, but actual no-steal/restore sequences need testing. |
| Bounded UI-thread action dispatch under ordinary load | PASS | S nonblocking queue, UI update path, event raising outside borrows; C/D basic operations. |
| Queue saturation, client-visible rejection and action-delivery reliability | UNVERIFIED | S try_send logs/drops on full/closed queue; no overload acceptance test or acknowledged error policy. |
| Oversize ValuePattern writes and boundary/error semantics | UNVERIFIED | S rejects values above 4096 UTF-8 bytes silently; CA flags this source-level risk. No observed runtime large-value result. |
| Early hidden activation, reactivation, forced-close/concurrent actions, counter exhaustion | UNVERIFIED | Basic pre-show/normal detach passes; empty initial bounds and unchecked prototype counters are not production-qualified. |
| Reduced-motion native query/basic model behavior | PASS | A query/policy tests; checkbox/local preference in D. No animation in spike. |
| OS reduced-motion setting change/reactivation and final motion behavior | UNVERIFIED | D notes no OS-toggle test; local checkbox is not OS integration validation. |
| High-contrast adaptation and reference-token composited contrast | UNVERIFIED | A/C/D not run; high-contrast implementation not qualified. Design targets are not measurements. |
| Non-default/mixed DPI, monitor changes and accessible geometry/focus | UNVERIFIED | D explicitly unrun; current-DPI resize does not cover this. |
| IME/AltGr/dead keys, graphemes, RTL, selection/caret edge cases | UNVERIFIED | D excludes these from aggregate manual confirmation. |
| Production result-list names/selection/status/empty/error semantics | UNVERIFIED | No production launcher exists. Existing spike has no catalog/result-list semantics; do not infer coverage. |
| Supported Windows build matrix, long-duration leaks/AT performance | UNVERIFIED | Tests on NT 10.0.26300.0 do not qualify target Windows 10/11 release families. |
| Production UI manual keyboard and Narrator acceptance | UNVERIFIED | Future changed/actual controls need their own tests; isolated spike acceptance cannot certify an unbuilt UI. |

Pinned experiment stack: iced 0.14.0, iced_widget 0.14.2, iced_winit 0.14.1, winit 0.30.13, AccessKit 0.25.1, accesskit_windows 0.35.1, accesskit_consumer 0.39.1, windows-sys 0.61.2. Existing released-source/license findings in C remain the basis; no dependency or fork was introduced here. No claim is made about today's upstream PR/release maintenance status beyond these pinned artifacts. Recheck it at a separately justified adoption decision. Versioned docs.rs pages failed retrieval in this assessment; no newly inspected API is claimed from those failed opens.

Microsoft's [Edit control requirements](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-supporteditcontroltype) call for Text support, string Value support and a relationship to an associated static label. This supports retaining the project's text and label gates; whole-value replacement alone does not close them. This is API research, not runtime validation or a conformance certification.

## Minimum path to a separate UI-entry review

**Entry** below means resolve in the isolated validation slice before production launcher construction is approved. **Release** means validate actual implemented product controls before shipping; release work is not permission to begin production UI now. Every artifact must include source/binary hashes, locked versions, OS/build, scale/monitor setup, observer/input modality, per-case outcomes and raw failures.

| Outstanding work | Requires / dependency | Expected artifact and precise acceptance | Gate |
| --- | --- | --- | --- |
| Truthful text model, caret, selection and range geometry | Code/API investigation on pinned iced widget/shaping boundary and released AccessKit text capabilities; real Windows host | Text capability decision plus cross-process raw/control/pattern/action logs. Actual widget caret/selection and ranges must agree after navigation, middle edit, selection replacement, Value update and resize; advertise only verified patterns/events, not guessed runs. Validate empty text, Unicode and selection bounds. | Entry |
| Restore LabeledBy correctness | Provider compatibility investigation; separate decision if an upgrade/patch is needed | UIA property references actual visible label under the same window. No invisible duplicate native input or design removal of labels to hide the gap. If pinned provider cannot deliver it, document blocker and seek a justified compatibility decision. | Entry |
| Revalidate essential manual accessibility after changes | Physical operator and Narrator on interactive Windows | Per-step keyboard log and speech transcript/observations, names/roles/value/state, caret/selection use, repeat error feedback, missing/duplicate speech, no surprise focus changes. All essential steps pass on the changed binary. Current aggregate attestation remains historical. | Entry; repeat on actual UI before release |
| Observe error announcements independently | Windows UIA event listener capable of resolving event IDs (SDK/Inspect/native client if managed lookup fails), plus Narrator | Event trace linked to each created/dismissed/recreated error, distinguish emitted events from actual speech; unchanged state must not emit redundant updates. Confirm focus stable, no missing/duplicate feedback. A tooling limitation alone is not a pass. | Entry |
| Cross-application focus and safe denial | Manual test plus existing guarded native focus policy; no global hooks needed | Input/Validate/Dismiss → switch app → return → Escape/Close, including target closed/replaced and restore denied. Restore only when owned/valid, never steal focus from user's newly activated app. | Entry; final launcher lifecycle release |
| Queue and long-value behavior | Narrow bridge code/policy decision, deterministic backpressure/removal/close tests, real UIA client | No silent successful action loss. Bounded overload must have a truthful client/diagnostic failure policy without GUI deadlock. Accepted values must be reflected; rejected values must have defined errors/limits. Late actions rejected. | Entry |
| Async/native lifecycle and identity exhaustion | Existing unit generation tests plus controlled reordered native actions/layout and teardown; checked counter policy | Current-generation geometry/focus survives deliberate reordering; early activation never exposes misleading actionable geometry; removed/closed nodes are unavailable; exhaustion cannot wrap IDs into stale accepted actions. Preserve 29 UIA checks and add focused cases. | Entry for changed slice; forced exit/long stress release |
| Representative display/accessibility settings | Manual Windows high contrast, one non-default DPI, OS reduced-motion query/reactivation; no persistent changes left behind | Visible focus, readable measured contrast, names/bounds/click targets remain correct; reduced-motion behavior follows OS/user preference. Record and restore settings. | Entry representative tests; mixed-DPI/hotplug/transparent actual surfaces release |
| IME and complex text | Real Windows IME/operator and text-provider tests | At least one composition/grapheme path cannot launch/dismiss mid-composition; caret/selection corresponds to actual text. Broader scripts/RTL/dead-key matrix recorded, no unsupported claim. | Entry core text tests; broader declared-language matrix release |
| Windows support and resources | Approve exact supported build range; target-host test matrix, stress/idle measurement | No unsupported Windows-family claim. Re-run essential UIA/manual cases on supported builds; no stale providers/leaks after repeated open/close, bounded updates and measured hidden/idle work. | Scope decision Entry; full matrix release |
| Result-list/menu/slider accessibility | Later feature-specific design/semantic review after entry approval | Real selected-result speech, names, list positions/virtualization, launch error feedback, empty/no-result states, and later dock controls verified individually with UIA/keyboard/Narrator. | Each feature's own build review and release gate, not implemented now |

The smallest safe next engineering task is **an isolated text-input capability investigation/validation extension**, not a launcher: identify real caret/selection/shaping state access first; prove released AccessKit text mapping against that state; then obtain new physical-keyboard/Narrator evidence. A wrapper that only knows a value and widget rectangle is insufficient. If the pinned iced boundary cannot provide truthful data, stop and present compatibility alternatives for explicit decision; do not silently fork iced, add a framework, invent ranges, or add dependencies. Assess the existing LabeledBy provider gap in parallel at the API-research level.

After entry rows pass, create a separate UI-entry review with explicit disposition of every remaining release condition. Only that review can authorize a bounded UI slice. The headless text deferral never becomes permission to ship without text semantics.

## Outstanding manual Windows checklist

Use a new evidence directory and binary identity for any changed validation slice. Start Narrator manually and record what is heard; physical keyboard and synthetic/UIA input get separate labels.

1. Tab/Shift+Tab entire visible order; Enter/Space activation; visible and UIA focus agree. Test error appearance, repeated error, Dismiss removal and restored local focus.
2. Type/edit in middle; select/replace/collapse via arrows/Home/End/Ctrl combinations; read caret/selected text through UIA and Narrator. Include non-ASCII, graphemes and IME composition; verify provider text positions, not merely visible editing.
3. Switch to another application and back, close/restore target, dismiss while foreground changed; record safe refusal and unexpected focus movement separately.
4. Repeat names/roles/state/error speech under high contrast, non-default and mixed DPI/monitor transitions; assess readable focus/bounds and OS reduced-motion changes without equating the local checkbox with the system setting.
5. Exercise stale actions/normal close and bounded repeated sessions. Record any unavailable case as UNVERIFIED or BLOCKED, never infer it from successful UIA discovery.

No new human result is requested or presumed during this assessment. Historical D attestation is retained exactly with its omissions.

## P3 runner disposition — separate PASS

Confirmed the disclosed ownership gap in `tools/Test-Stage3B.ps1`: observer exceptions previously reached Dispose alone, which closes a handle rather than stopping the owned process. The snapshot may leave the finite probe running until its own exit. No previous test result is rewritten as a leak observation.

Extracted `tools/Invoke-OwnedProbe.ps1`, used only by the Stage 3B test runner. Its try/finally covers process start, handle acquisition and all observation; it stops a still-running exact owned Process, waits for exit, then disposes. It tolerates a natural-exit/Kill race only when that exact process is confirmed exited. A live-process kill failure propagates, rather than reporting clean shutdown. No name-based kill, process-tree kill, service, dependency or runtime consumer change.

Added `tools/Test-OwnedProbe.ps1`: a real Windows PowerShell child signals readiness and blocks on a parent-owned named event. Deterministic injection then throws before metrics; the exception propagates and the owned PID no longer exists. Two other cases release normally or throw after confirmed normal exit. Named events are temporary kernel objects; test finally releases/disposes them. Ten-second event waits are test-hang safeguards, not cleanup timeouts or sleeps establishing order.

Fresh Windows PowerShell 5.1 run: **3/3 passed**, exit 0, all ChildRemaining=false; initial development run also 3/3 passed. Fresh modified Stage 3B runner: **56 ordinary workspace tests passed**, 5 opt-in fixtures then enabled; **50 enabled sei-apps tests passed** (47 library + 3 real Windows integration), plus fmt/all-target check/strict Clippy/release/example builds and diff check, overall exit 0. Existing lifecycle and accessibility sources are unchanged. The filesystem sandbox exception permits the already-known temporary shortcut rename; it is not Windows RunAs, and resource metadata reports a non-admin token. No current GUI/UIA/manual tests were run.

Reproduce with fresh output paths:

```powershell
powershell.exe -NoProfile -File tools/Test-OwnedProbe.ps1 -OutputDirectory docs/implementation/evidence-accessibility-gate-reassessment/another-owned-probe-run
./tools/Test-Stage3B.ps1 -OutputDirectory docs/implementation/evidence-accessibility-gate-reassessment/another-stage3b-run
```

See [regression results](evidence-accessibility-gate-reassessment/runner-regression-final/results.json) and [new execution index](evidence-accessibility-gate-reassessment/README.md). P3 is closed for the Stage 3B observer boundary; this does not assert every other script has equivalent cleanup or that arbitrary external termination can be guaranteed.

## Scope conclusion

Gate **BLOCKED**, with traceable historic isolated passes, explicit headless deferral, tested baseline failures and unverified production conditions. Headless acceptance and P3 cleanup remain independent successes. Only report/test-tooling files were added/changed in this task. Prior evidence, Rust consumer and bridge, designs and locks remain intact. No production UI or shell integration started; no commit or push.
