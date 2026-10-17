# Stage 2F — Upstream compatibility research and decision brief

2026-10-15. **Recommendation: upstream-first; retain the current pins and keep production accessibility BLOCKED.** No complete supported editable-text integration was established for the stock pinned TextInput. Do not authorize a project-owned input by implication. It is a separate, substantial maintenance decision, not a small bridge repair.

Headless Stage 3B and its lifecycle review remain PASS for their accepted scope; P3 runner cleanup remains PASS. This research changes none of those verdicts. No implementation, dependency change, fork, design change, GUI execution, commit or push occurred.

## Scope, sources and release verification

Read [Stage 2E](STAGE-2E-TEXT-INPUT-CAPABILITY-REPORT.md), the authoritative [gate reassessment](ACCESSIBILITY-GATE-REASSESSMENT.md), [Stage 2D evidence](evidence-stage2d/README.md) and manual attestation, the locked spike and its semantic/native/wrapper boundaries. The input is the uncommitted `main` working tree based on `e11af2cf6baea5c20e1edc7d48c603c6ab77397c`; HEAD alone does not describe that tree. [Research evidence](evidence-stage2f/README.md) records preservation, published source identities and limitations.

| Inspected released component | Exact source identity / relevant files |
| --- | --- |
| iced / iced_core 0.14.0 | Published VCS revision `3997291f318a8bc06fa522f5579836fb3feb94df`; operation/text_input.rs, text/paragraph.rs, shell.rs and input_method.rs. [Release tag](https://github.com/iced-rs/iced/releases/tag/0.14.0), [tagged widget source](https://github.com/iced-rs/iced/blob/0.14.0/widget/src/text_input.rs) also checked. |
| iced_widget 0.14.2 | Published VCS `54abf81d13fec06d4d9ac754b03ce2c3313e8a1f`, path widget; full cached text_input.rs, cursor.rs and value.rs. This newer widget patch is already pinned; it is not a proposed upgrade. |
| AccessKit 0.25.1 / accesskit_consumer 0.39.1 | Published `ce8164ba92995cfa86005b6259115e08c8244253`; semantic TextRun/TextPosition/TextSelection and consumer text support/geometry. |
| accesskit_windows 0.35.1 | Same published revision, path adapters/windows; full cached node.rs, text.rs and package metadata. [Tagged node source](https://github.com/AccessKit/accesskit/blob/accesskit_windows-v0.35.1/adapters/windows/src/node.rs), [release](https://github.com/AccessKit/accesskit/releases/tag/accesskit_windows-v0.35.1). |
| Existing compatible stack, not upgraded | iced_winit 0.14.1, winit 0.30.13, windows-sys 0.61.2 and provider windows 0.62.2 remain as locked. They do not supply missing widget semantics merely by being compatible dependencies. |

Full inspected local source is the published cache matching Cargo.lock; hashes and `.cargo_vcs_info.json` identities are retained, rather than treating a mutable branch as a released artifact. Failed commit/documentation fetches are not represented as inspected remote source.

Read-only archive verification matched all 11 inspected files to their cached published `.crate` contents, and each archive's SHA256 to Cargo.lock. See [archive-source-verification.json](evidence-stage2f/archive-source-verification.json). An earlier optional per-file checksum lookup was unavailable because this extracted cache has no `.cargo-checksum.json`; it was not a source mismatch. No archive was installed or extracted into the repository.

**Newer releases:** the accessible official [iced releases](https://github.com/iced-rs/iced/releases) and [tags](https://github.com/iced-rs/iced/tags) list 0.14.0 as the latest visible application release. The accessible [AccessKit releases](https://github.com/AccessKit/accesskit/releases) list Windows 0.35.1, core 0.25.1, consumer 0.39.1 and winit 0.34.1 at the top. These are already the relevant baseline components. No newer released fix was located or source-verified; a 0.15 development milestone is not a released API.

An independent read-only registry recheck outside the restricted network environment **succeeded**: crates.io reports max_stable_version and newest_version as iced 0.14.0, iced_widget 0.14.2, accesskit 0.25.1 and accesskit_windows 0.35.1. Exact response summaries, publication timestamps and source URLs are in [registry-recheck.json](evidence-stage2f/registry-recheck.json). Thus no newer published stable version exists in that observed registry snapshot for these four packages; their released source is already inspected locally. This is metadata/source verification, not execution of a new integration.

Initial web/API/sparse-index/docs.rs requests and a sandboxed direct request failed; those failures do not override the successful recheck. Some GitHub commit URLs remained unavailable. One main-branch release-manifest page was older than the release page, so it was not used to certify latest versions. Repeat registry and exact artifact/source/license review before a future upgrade. No candidate was installed, resolved, built or run in this task.

## Findings at the stock widget boundary

The pinned public typed State exposes cursor(), and Cursor::state(&Value) preserves selection direction. Generic Operation::TextInput omits that readback; typed child-Tree access is the plausible supported adapter boundary, with ownership/type checks. Value uses grapheme units; conversions must derive from actual selectable-unit prefixes and validate metadata limits. Stage 2E's three API tests support limited examples, not live selection or all Unicode/IME behavior.

The actual retained paragraph, preedit state and drawing-specific horizontal scrolling remain private. Public Paragraph methods can query a paragraph the caller possesses; they do not grant access to the stock widget's paragraph. Public layout/viewport parameters provide meaningful clipping context but not character rectangles. Reconstructing a second paragraph or probing temporary caret positions would risk divergence, focus/selection changes and misleading semantics; neither is a supported read-only snapshot solution.

### Additional partial path: the widget's own IME output

Fresh source inspection found **Shell::input_method()** and InputMethod::Enabled {cursor, purpose, preedit}. TextInput requests this strategy on RedrawRequested using its actual retained paragraph. A narrowly owned wrapper could observe that output after child update without independently shaping text. This is useful public evidence not fully explored in Stage 2E.

It is not a complete text snapshot. Requests are redraw-bound; Shell merging prioritizes an earlier enabled request, so attribution to this exact child must be proved. With a selection, TextInput's input_method computes its anchor from start.min(end), while the drawing/scrolling path uses the active endpoint. Therefore the IME rectangle cannot simply be advertised as the active caret/selection geometry. It also supplies neither arbitrary range rectangles nor all hit-test positions, and it carries no universal text/layout generation. Preedit selection uses its own byte-indexed units; it must not be conflated with document grapheme or UTF-16 indices.

A collapsed-caret/preedit observation experiment is **plausible**, not runtime-verified. It may reduce uncertainty about a partial capability; it does not unblock TextPattern or supersede the existing gates.

## Capability matrix

“Verified” below means inspected source, except where explicitly attributed to historical runtime evidence. “Plausible” requires a new approved experiment. “Unsupported” means the unchanged inspected stack does not establish the requested behavior; it is not a claim that all future architectures are impossible.

| Capability | Finding | Required decision/evidence |
| --- | --- | --- |
| Model value and ValuePattern | Verified source and historical Stage 2D/2E UIA | Preserve whole-value behavior; fix oversize/error policy separately. |
| Real caret/selection indices and direction | Verified public typed State API; plausible adapter | Collect coherent live state after edits/navigation, retaining anchor/focus direction. |
| UTF-8/grapheme/UTF-16 conversion | Verified constituent APIs; narrow historical API tests | Checked unit mapping, invalid positions, long clusters, bidi and IME must be qualified. |
| Actual retained paragraph readback | Unsupported for this stock widget wrapper | Upstream supported snapshot/readback API or explicit project-owned-widget decision. |
| Layout child bounds and viewport context | Verified public Widget parameters | Not sufficient alone to fabricate range rectangles or visibility. |
| Collapsed-caret rectangle and preedit through Shell | Verified output API/computation; plausible observation | Prove attribution, redraw timing, coordinate mapping, clipping and matching model/layout epoch. |
| Selection-active caret via IME anchor | Unsupported substitution | Source uses leftmost endpoint for IME; do not infer active caret correctness. |
| Complete range geometry/hit testing/scroll snapshot | Unsupported through existing readback boundary | One authoritative rendering/editing snapshot; no independently reshaped proxy. |
| Coherent native layout/text/composition generation | Plausible project protocol, not supplied atomically by widget | Checked epochs and reorder/close tests; current app generation guards are only partial protection. |
| Basic Windows TextPattern / TextRange methods | Verified provider implementations, conditional on TextRuns | Actual spike lacks TextRuns/TextPattern; provider availability is not widget integration. |
| LabeledBy mapping | Unsupported in inspected provider; historical actual property absent | Provider compatibility work and real property-target validation. |
| GetVisibleRanges | Unsupported visible-range behavior in inspected provider | Method returns null unconditionally; widget metadata alone cannot implement the missing method. |
| Caret semantics through degenerate GetSelection | Verified provider mechanism; plausible authored selection | Real collapsed selection and AT validation required. No GetCaretRange claim. |
| TextPattern2 / GetCaretRange | Unsupported in inspected Windows implementation | Separate provider/API/client-compatibility decision; not interchangeable with an index getter. |
| Full truthful stock TextInput accessibility on pinned stack | Not established | Both widget and provider boundaries must pass; no released upgrade shortcut was verified. |

## Windows provider versus widget semantics

The Windows implementation conditionally exposes ITextProvider when consumer text runs exist. GetSelection wraps the authored selection, DocumentRange uses consumer ranges, RangeFromPoint consumes text hit-test data, and ITextRangeProvider::Select dispatches SetTextSelection. GetBoundingRectangles adds the client origin to consumer-authored rectangles. These mechanisms do not discover iced text layout or validate independently reconstructed geometry.

The inspected Windows source has no LabeledBy dispatch or TextPattern2/GetCaretRange implementation. Its GetVisibleRanges returns null. Those are distinct provider gaps: even a perfect widget snapshot cannot fill an unimplemented provider method through semantic attributes alone. A custom widget does not automatically remove any of them.

A degenerate TextPattern GetSelection is a possible caret representation; it is not proof of separate ITextProvider2 support. The [Microsoft Text/TextRange contract](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-implementingtextandtextrange) distinguishes these operations and connects hit testing, selection and geometry. Decide the required client/API coverage explicitly and keep all current acceptance requirements in force; do not quietly redefine them around whichever method happens to be implemented.

## Options and costs

These are engineering planning estimates, not measured delivery guarantees. They assume one experienced Rust/Windows engineer, a Windows operator for AT/IME testing, one single-line input, existing released rendering/shaping infrastructure, and no production shell features. No implementation estimate includes a guaranteed upstream release date.

| Dimension | A — Wait for upstream-supported APIs (recommended) | B — Explicitly authorize one project-owned input |
| --- | --- | --- |
| Ownership | Maintain the bridge and tests; upstream owns stock editing/rendering internals | Own input editing, rendering integration, selection, scrolling, composition, accessible snapshot/actions and their compatibility |
| Initial effort | Roughly 2–5 engineer-days to specify required snapshots/repros and audit candidates; qualification after a usable release may take several engineer-weeks | Budget roughly 8–16 engineer-weeks for an isolated candidate and qualification, with substantial uncertainty; re-estimate after an initial 1–2-week design/API-validation phase |
| Calendar risk | Upstream response/release timing unbounded; no promise of launcher eligibility | Can make local progress, but IME/AT/provider blockers can invalidate the estimate; not a guaranteed faster release |
| IME | Reuse stock implementation; still test committed/preedit separation, candidate placement and cancellation | Own input-method transitions and interactions, including composition selection units and no accidental submit/dismiss |
| Shaping/geometry | Obtain actual stock retained data through a supported contract | One owned paragraph/run state must feed both rendering and accessibility; never maintain a proxy paragraph for a stock control |
| Selection/Unicode | Adapter conversions remain project responsibility | Full selection, anchor/focus, mouse/keyboard navigation, grapheme/bidi/hit testing, clipboard and invalid-range policy become project responsibilities |
| Maintenance | Recheck upstream breaking changes and provider behavior for every dependency change | Permanent owner and review budget; widget/runtime/renderer/font/shaping changes require requalification. Calling it bounded does not remove this obligation |
| Fork risk | None if only released supported APIs are accepted; no unofficial branch/backport by default | Not an iced fork if independently authored through supported Widget APIs; copying/vendor-patching stock widget code would create derivative/fork maintenance and needs its own approval/license review |
| Provider gaps | Still require independent resolution | Exactly the same LabeledBy/visible-range/caret API issues remain |
| Tests | Retain 29 baseline checks, add complete text and AT matrix | Same matrix plus editing/rendering state-machine, composition, selection, clipboard and stress coverage; significantly more unit/integration cases |

Neither option substitutes a framework, weakens physical keyboard/Narrator acceptance or authorizes production UI. The custom option is viable as an investigation hypothesis, not proven compatible with every pinned shaping/editing API. Reusing a lower-level shaper is acceptable only when it is the actual widget renderer's authority; independently reshaping a stock-widget proxy is excluded.

## Recommendation and owner decisions

Choose **A, upstream-first**, because the current constraint set favors supported stock behavior and no unreviewed fork. Retain pins until a candidate has exact released-source proof and an explicit dependency decision. The registry currently offers no newer stable shortcut. If upstream timing is unacceptable, choose B explicitly with a named maintenance owner, budget and stop criteria.

Approval decisions are concrete and separate:

1. Confirm waiting versus funding a project-owned input investigation. This brief itself approves neither implementation nor an indefinite schedule.
2. Authorize any upstream contact/issue/PR separately; no external message was sent. Agree on a read-only coherent snapshot contract rather than private-field exposure alone: actual text/selection units, retained layout geometry, viewport/scroll, composition, coordinate space and epoch/lifetime.
3. For any candidate upgrade, approve exact versions after registry/source/hash/license/MSRV/API review and before changing manifests. AccessKit Windows 0.35.1 declares Rust 1.87 while the workspace declares 1.88; historical execution used 1.99.0, so declared minimum compatibility is not already runtime-qualified.
4. Decide provider remediation and required caret API coverage without waiving existing acceptance. No project-owned widget approval implicitly authorizes an AccessKit fork/backport.
5. If B is chosen, authorize only an isolated one-input phase first, not a launcher or generic widget framework. Require one editable/rendered state, supported APIs, explicit composition/error/overflow behavior, source provenance, and rejection if it needs a hidden native duplicate or second shaping authority.

## Smallest next experiment after approval

A low-cost optional first experiment is observational: extend only the existing isolated wrapper to read the known child State and its Shell IME output on redraw, recording generation, ownership, collapsed caret and preedit metadata across a middle edit, reverse selection, resize and IME. It must not advertise TextPattern or approximate ranges. Success would establish a limited public readback path; a mismatch or ambiguous Shell attribution stops that path. This experiment is proposed, not built or authorized here.

For A's actual adoption experiment, wait for an exact supported/released snapshot candidate, re-audit its artifact, then connect that snapshot in the isolated spike and test real DocumentRange/GetText/GetSelection/Select/RangeFromPoint/bounds/visibility. For B, first prove one owned text run drives both painting and semantic geometry under resizing and selection; stop if the engine interfaces cannot supply truthful state. Both require separate provider disposition before claiming a complete text interface.

Retain all 29 baseline checks. Add raw/control/pattern/event observations with exact source/binary/environment identity, empty/Unicode/grapheme/invalid-bound cases, typing and whole-value changes, selection replacement/direction, composition transitions, stale updates/removed nodes, resize/DPI/scroll and normal teardown. Run physical keyboard and Narrator separately against the changed candidate with per-case speech/focus outcomes. No guessed geometry, silent limit or successful compilation counts as runtime acceptance.

## Gates and verification boundary

The authoritative reassessment remains unchanged: unresolved text ranges/caret/selection, LabeledBy, independently observed live error events, queue saturation/action rejection, oversized writes, external focus restore/safe refusal, refreshed manual keyboard/Narrator, high contrast/DPI/reduced-motion settings, IME/complex text, provider lifecycle/stress and production result-list semantics. Historical D user attestation remains valid for its isolated binary; it is not refreshed speech or proof of new text capabilities. Stage 2E's fresh 29 checks and pattern-absence query are also historical here.

**Performed now:** repository/source/manifest/VCS inspection, official release/tag searches, capability/compatibility analysis, preservation hash comparison, local-document-link/whitespace checks and git diff --check. **Not performed:** builds, cargo tests, new UIA queries, physical input/Narrator, dependency resolution, new-version execution, prototype construction or upstream communication. Only new documentation/research evidence is added.

Result: no source-verified released shortcut closes the complete gate. Recommend the explicit upstream-first decision and, if approved, the narrowly scoped observational or released-snapshot experiment above. Stop before implementation; production accessibility remains BLOCKED.
