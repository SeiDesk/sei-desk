# Stage 2E — Isolated text-input capability investigation

2026-10-15. Verdict: **BLOCKED for complete truthful text-range integration on the pinned, unchanged spike.** The investigation establishes a narrower result than earlier generic-operation analysis: typed iced widget state does expose caret/selection indices, but the existing wrapper cannot read the retained shaping/viewport data needed to back truthful range geometry. No text pattern or guessed geometry was added. A compatibility decision is required before extending the bridge.

Headless Stage 3B and P3 runner acceptance remain unchanged. The production accessibility gate remains blocked under the [authoritative reassessment](ACCESSIBILITY-GATE-REASSESSMENT.md). No production UI or shell integration was started; no dependency, fork, existing source, design or historical evidence changed; nothing was committed or pushed.

## Input state and exact inspected stack

Reviewed the uncommitted `main` working tree at base HEAD `e11af2cf6baea5c20e1edc7d48c603c6ab77397c`; recorded status and pre-existing tracked/untracked hashes before creating probes. Read the reassessment, [Stage 2D closure](STAGE-2D-ACCESSIBILITY-CLOSURE.md), [matrix](STAGE-2D-TEST-MATRIX.md), [evidence](evidence-stage2d/README.md), existing spike and [Stage 3B review](STAGE-3B-PATCH-REVIEW.md). The D user-attested manual results and headless-only text deferral remain historical with their original scope.

Inspected locally cached released source matching the standalone Cargo.lock:

| Component | Exact version / boundary inspected |
| --- | --- |
| iced / iced_core | 0.14.0 / 0.14.0; advanced Tree, Operation::TextInput and Paragraph APIs |
| iced_widget | 0.14.2; actual TextInput State, Cursor, Value, drawing/layout/input paths |
| iced_winit / winit | 0.14.1 / 0.30.13 in locked stack; no event-loop replacement |
| AccessKit | 0.25.1; TextPosition, TextSelection, TextRun character metadata |
| accesskit_consumer | 0.39.1; valid text positions, support predicates and range rectangles |
| accesskit_windows | 0.35.1; actual ITextProvider/ITextRangeProvider, action and coordinate mapping |
| Windows bindings | experiment windows-sys 0.61.2; existing provider windows 0.62.2 |

Native probe linking also used the existing transitive windows_x86_64_msvc 0.52.6 import-library directory. No package was added/upgraded. Existing C/D licensing findings apply; no new license or latest-upstream maintenance claim is made. Exact source hashes and line references are in [source evidence](evidence-stage2e/source-review.txt) and [inspected hashes](evidence-stage2e/inspected-source-hashes.json). Versioned docs.rs retrieval failed; conclusions rely on inspected cached source, compiler results and observed Windows behavior, not unread web pages.

## Real sources of truth

1. **Text value:** the spike's model String feeds the actual TextInput and current ValuePattern. The generic Operation::TextInput::text() is not an authoritative editable-value getter: it can return the placeholder. Do not use placeholder text as the document. ValuePattern currently replaces the entire value, and does not establish selection/caret support.
2. **Caret and direction:** iced_widget text_input::State<P> publicly exposes cursor(); Cursor::state(&Value) returns Index or Selection {start,end} and clamps to that Value's grapheme count. Cursor::selection returns sorted endpoints, losing direction; preserve state() endpoints when constructing anchor/focus. The existing generic operation trait omits this getter, but a wrapper owning the known TextInput child Tree can downcast its public state to the exact renderer paragraph type. A wrong tree/type would panic; this is a typed-wrapper responsibility, not arbitrary widget introspection.
3. **Position units:** Value::new uses the pinned UnicodeSegmentation graphemes. Obtain exact prefix bytes via Value::until(index).to_string(), then count that prefix's UTF-16 units. AccessKit TextPosition.character_index indexes the authored TextRun character_lengths array, not a UTF-16 offset. Those lengths are UTF-8 byte lengths of selectable units; sums, limits and conversions must be checked. Counting total string length or scalar values is not a valid caret mapping. The probe demonstrates selected samples only, not every script or arbitrarily long grapheme/metadata limit.
4. **Rendered geometry:** State.value (retained paragraph), placeholder/preedit paragraphs and related editing internals are private. The draw path uses the retained paragraph's grapheme_position, text-child bounds, clipping, alignment and private scrolling measurements; it changes the paragraph used when placeholder/preedit is present. Paragraph::grapheme_position/hit_test/span_bounds are public **on a paragraph you actually possess**. The wrapper does not possess the widget's retained paragraph. A new independently shaped paragraph or cloned private drawing algorithm would need a separate correctness/architecture decision; no equivalence is assumed.
5. **Updates:** authoritative readback would need a coherent actual model value + child state + actual layout/viewport generation, after the relevant edit/layout, including selection-only navigation. Existing generation rejection prevents some stale layout/focus messages, but the current wrapper publishes only node rectangles. There is no live text-state collector or range-action-to-widget mapping implemented in this investigation.
6. **Label:** the spike authors labelled_by to its visible Sample text label, but the actual Windows provider does not expose LabeledBy. The separate runtime check below reconfirms that failure. A correct name does not resolve the missing relationship.

This refines the earlier limitation: **selection readback is missing from the generic Operation trait, not from all public iced State APIs.** Earlier reports remain intact; the outstanding range-geometry and provider obligations remain real.

## Capability matrix

Statuses describe source-level capability of the inspected stack, not an award of runtime accessibility. AVAILABLE requires a demonstrable current source; AVAILABLE_WITH_ADAPTER identifies supported APIs that still need a coherent bridge; REQUIRES_COMPATIBILITY_CHANGE needs a separate API/provider/architecture decision; UNAVAILABLE means no defensible complete path is established within the unchanged spike.

| Capability | Classification | Source of truth / evidence | Runtime limit |
| --- | --- | --- | --- |
| Current editable String and whole-value UIA read/set | AVAILABLE | Spike model, semantic value and existing SetValue dispatch; fresh 29-check Windows run | Current 4096-byte action limit remains unresolved; placeholder is excluded. |
| Caret grapheme index | AVAILABLE_WITH_ADAPTER | Public State::cursor + Cursor::state, compiled typed-tree access, API unit test | Not collected from a live input or exposed through UIA here. |
| Selection endpoints and direction | AVAILABLE_WITH_ADAPTER | Cursor::state retains start/end; selection() gives sorted range; API test | No programmatic selection read/action bridge implemented. |
| UTF-8/UTF-16 prefix mapping | AVAILABLE_WITH_ADAPTER | Pinned Value graphemes and actual prefix contents; Unicode API test | Must validate metadata unit/length limits and reject invalid external positions; no arbitrary byte arithmetic. |
| Logical range contents/boundaries | AVAILABLE_WITH_ADAPTER | Value::select/ until plus actual String and normalized verified endpoints | Reversed/invalid ranges need explicit validation; direct State setters clamp rather than define UIA rejection semantics. |
| Empty text and endpoint clamping | AVAILABLE | Actual State/Value API tests | Empty document is not placeholder; UIA invalid-node/range behavior is not tested by these unit tests. |
| Non-ASCII/grapheme semantics | AVAILABLE_WITH_ADAPTER | Tested accent, surrogate pair, combining sequence and ZWJ emoji using iced Value | No IME/bidi/all-grapheme conformance claim. AccessKit metadata limits still require checks. |
| Read actual retained paragraph from stock TextInput State | REQUIRES_COMPATIBILITY_CHANGE | State.value private; negative compile probe E0616 | No supported getter established for existing wrapper. |
| Truthful caret/range pixel bounds and RangeFromPoint | REQUIRES_COMPATIBILITY_CHANGE | Private paragraph/scroll path; generic layout gives widget bounds only | No fabricated rectangles or independently shaped proxy accepted. |
| Released provider basic TextPattern/TextRange machinery | AVAILABLE_WITH_ADAPTER | accesskit_windows node.rs/text.rs, consumer text model | Conditional on valid authored TextRun data; actual spike input does not advertise TextPattern. |
| Provider selection actions | AVAILABLE_WITH_ADAPTER | ITextRangeProvider::Select emits SetTextSelection; public State select_range exists | Current semantic dispatcher rejects unsupported actions; no real UIA Select success. |
| Visible-range enumeration | REQUIRES_COMPATIBILITY_CHANGE | Pinned ITextProvider::GetVisibleRanges returns null; source TODO | Needs provider/API disposition and actual visible-range testing; model concepts alone are insufficient. |
| Separate TextPattern2/GetCaretRange API | REQUIRES_COMPATIBILITY_CHANGE | No TextPattern2 implementation found in pinned Windows source | Do not claim it from a TextSelection property; core degenerate GetSelection still needs validation. |
| UIA LabeledBy relationship | REQUIRES_COMPATIBILITY_CHANGE | Fresh actual property absent; C/D failure retained | Separate provider compatibility blocker. |
| Complete truthful text interface after typing, selection replacement, error and resize | UNAVAILABLE | No established joined editing/shaping snapshot + complete provider path | Investigation stops before semantic extension; not evidence of an attempted full implementation failing. |

The Windows provider really has GetSelection, DocumentRange, RangeFromPoint, GetBoundingRectangles, GetText and Select implementations. GetBoundingRectangles uses consumer range geometry plus actual client-origin conversion; this cannot repair invented upstream character positions. AccessKit TextRun and TextSelection concepts alone do not make iced expose them. Microsoft's [Text/TextRange guidance](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-implementingtextandtextrange) distinguishes document/selection/visible ranges, hit testing and range geometry; the caret can be represented by a degenerate range. This is source/API research, not proof of this spike satisfying those operations.

## Isolated probes and fresh execution

No semantic control tree, UI input, application logic or dependency was changed. Only small evidence-local API/client probes were written; hashes are in [probe identities](evidence-stage2e/probe-identities.json). Executables/debug outputs live under the existing ignored experiment target directory; no production crate or new workspace was created.

| Executed test | Observed result |
| --- | --- |
| Typed State cursor/selection public API, exact selection direction/content | Passed |
| Pinned grapheme prefix mapping for `aé😀e + combining acute + woman-ZWJ-laptop` | Passed: UTF-8 boundaries 0,1,3,7,10,21; UTF-16 boundaries 0,1,2,4,6,11 |
| Empty input/current-value clamping and out-of-range setter behavior | Passed; demonstrates clamping, not safe UIA invalid-range rejection |
| Attempt retained State.value paragraph access | Expected compile rejection E0616, exit 1; confirms private boundary, not a successful geometry implementation |
| Unmodified spike fmt/check/test/strict all-target Clippy/release build | All exit 0; 13 existing unit tests passed |
| Unmodified cross-process Test-UIA.ps1 | All **29 existing checks passed freshly**, exit 0; raw/control trees, actions and normal provider cleanup preserved |
| Separate actual pattern/property observation | Own PID/HWND + Sample text Edit: ValuePattern=true, TextPattern=false, LabeledBy=false; observation client exit 0 |
| Both owned spike processes | Normal close, exit 0, no owned child remaining |
| Physical keyboard, Narrator, live-event listener, actual UIA text ranges | Not run; no range provider advertised, no changed text UI to qualify |

Host: Windows NT 10.0.26300.0 x64/MSVC, Rust 1.99.0 (b940084d7 2026-09-28), Windows PowerShell 5.1 clients. The fresh release binary SHA256 is unchanged from D: `70307E37BB8F06E11FA09EA3DF07E1D164840EDCA5C2BF12E14B8BE6D719D83F`. Matching bytes do not make old evidence fresh: the new runs have new processes, HWNDs and timestamps.

The first direct rustc unit-probe link failed because invoking rustc outside Cargo omitted the already-installed windows.0.52.0.lib search path (LNK1181). The failed output is retained. Supplying the existing native import-library directory fixed linking; the final three API tests passed. This is a probe-link invocation correction, not an iced API or dependency change. Expected negative compilation and positive runtime tests are recorded separately. API tests instantiate the real pinned State/Value types without a window; typed Tree readback is compiled but not executed against physical input. They are not mocks presented as UIA proof.

The fresh UIA run used a two-second idle sample, not a new 30-second resource baseline. Its static TextSelection metadata is not used as proof of pattern absence: the separate client actually queried TryGetCurrentPattern and LabeledBy. Existing whole-value/move/resize tests remain ValuePattern/layout tests, not text-range validation.

Reproduce the established spike gates and client using the [new evidence instructions](evidence-stage2e/README.md). There are no newly implemented runtime text-range cases to run; actual typing/navigation/middle-edit/selection/value/resize range agreement remains blocked. Do not award a PASS to unperformed tests.

## Compatibility decision and smallest next step

Prefer a narrowly scoped **supported read-only editing/layout snapshot API** exposing the stock input's current selection, retained shaping data/character positions, text origin, viewport/clipping/scroll/alignment and composition state under one coherent generation. Cursor getter already exists; avoid a broad widget replacement just to obtain it. Investigate a released upstream API or propose that precise upstream surface, without assuming a newer version fixes it. Any upgrade/fork/backport requires separate compatibility review and explicit decision; none is selected or imported here.

If no such supported surface is available, the alternatives require a decision: wait for upstream, or separately authorize a bounded project-owned input widget whose own rendering/editing state is the single source of truth. The latter has substantial IME/shaping/maintenance obligations and is not a thin adapter; it was not built. A native duplicate input, independently shaped hidden proxy or parsed Debug state is not recommended.

In parallel, investigate Windows provider LabeledBy and visible-range/caret API limitations against released code. After approved compatibility work, extend only the isolated spike, retain all 29 checks, and add actual cross-process DocumentRange/GetText/GetSelection/Select/RangeFromPoint/rectangles and invalid-bound tests. Repeat after actual typed edits, reverse selection, Value writes, error layout changes and resize, with Unicode/IME and non-default DPI. Require fresh physical keyboard/Narrator observations; no existing headless deferral waives this.

## Independent gates remain open

The [reassessment](ACCESSIBILITY-GATE-REASSESSMENT.md) remains authoritative. This investigation does not resolve: LabeledBy (fresh failure), live error-event observation (not run), action-queue saturation/rejection (not run), oversized values (not run), safe external focus restoration (not run), new manual keyboard/Narrator after changes (not run), high contrast/mixed DPI/OS reduced motion (not run), actual IME/complex-script behavior (not run), or production result-list/launcher semantics (no production UI exists). Existing user-attested D manual passes remain scoped historical successes, not a new speech test.

Final disposition: **BLOCKED for the complete text-input experiment; source-level partial availability established.** Stop at this boundary and request a separately scoped compatibility decision. No production UI eligibility, shell integration, commit or push follows from the successful API and unchanged-baseline tests.
