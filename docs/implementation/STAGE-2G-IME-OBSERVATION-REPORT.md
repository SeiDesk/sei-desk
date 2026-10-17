# Stage 2G — Isolated iced TextInput and IME observation

2026-10-17. **PASS for the bounded observational investigation, with real OS IME composition and native reorder cases UNVERIFIED.** A complete coherent text-accessibility snapshot was not established. Shared-Shell output can be inherited, and its rectangle is not a general active-caret/selection rectangle. **Production accessibility remains BLOCKED.** Option A (upstream-first) remains the recommendation; no TextPattern/TextRange integration was attempted.

## Baseline and isolation

Actual starting HEAD matches the requested baseline: `d14d4bede2096d4bea2561de22977d181376b640`, branch main, initially clean. Read [Stage 2E](STAGE-2E-TEXT-INPUT-CAPABILITY-REPORT.md), [Stage 2F](STAGE-2F-UPSTREAM-COMPATIBILITY-DECISION.md), authoritative [reassessment](ACCESSIBILITY-GATE-REASSESSMENT.md), Stage 2C/2D evidence and the accepted spike. Existing 29/29 UIA results and D manual attestation remain historical and untouched, not evidence for this experiment's geometry or new Narrator behavior.

Added only `experiments/iced-ime-observer/` (src/main.rs, src/observe.rs, Build.ps1, Run.ps1, README.md and target ignore), this report, and [new evidence](evidence-stage2g/README.md). No existing source, dependency, Cargo.lock, accepted spike, service, design or historical evidence was changed. No commit or push.

The observer uses **stock iced TextInput**, not a custom editable widget. The wrapper delegates rendering, input, operations and overlays; it reads its known child's public typed State and the actual shared Shell before/after child update. It does not reset Shell, isolate/replace its merge policy, patch widget internals or reshape a proxy. No AccessKit provider is installed in the observer, and no text-range patterns are advertised.

## Versions, sources and execution identity

Pinned iced/iced_core 0.14.0, iced_widget 0.14.2, iced_winit 0.14.1, winit 0.30.13, tiny-skia renderer. Existing spike AccessKit 0.25.1, accesskit_windows 0.35.1 and consumer 0.39.1 are unchanged; the new observer needs no AccessKit integration. Exact published source identities remain [Stage 2F's audited VCS/artifact identities](evidence-stage2f/published-vcs.json): iced_core `3997291f318a8bc06fa522f5579836fb3feb94df`, iced_widget `54abf81d13fec06d4d9ac754b03ce2c3313e8a1f`, AccessKit family `ce8164ba92995cfa86005b6259115e08c8244253`.

Windows NT 10.0.26300.0 x64/MSVC, Rust 1.99.0. Build links the exact accepted spike cached iced rlib/import-library graph, with artifact and unchanged lock hashes in each build identity. No Cargo manifest or lockfile is created/updated. `-O` optimizes the harness, but linked dependencies are existing debug artifacts; this is not a full release benchmark or minimum-Rust-version qualification.

| Runtime | Source/build | Binary SHA256 / actual owned process |
| --- | --- | --- |
| run-01 | build-04; exact source snapshots saved inside run-01 | `CA3F9F4B1562DEE8FF18C96C3890582A05316F8C47C257B105EDB8D6D13BD8A4`; PID 10768, native window 918932 |
| run-02 | build-05, final source; adds long fixture and deduplicates idle redraw samples | `FBFD9C5F93767E2C45A886A553DF0DC243FF2FABD4675167499B04C7F67B1481`; PID 23148, native window 395400 |

Both processes closed normally with exit 0 and ChildRemaining=false. Immediate run-02 enumeration briefly returned the closing window; a subsequent enumeration returned none. Closing was not assumed synchronous from the key press alone.

## What each observation means

Each JSONL observation records input A/B, local State::is_focused, the model String supplied to the actual widget, typed cursor anchor/active grapheme indices, logical direction, checked prefix UTF-8/UTF-16 units, Shell IME before/after with actual rectangle/purpose/preedit, widget/text-child bounds, passed viewport, elapsed microseconds and sample sequence. Values and units come from pinned Value/State APIs, never from total string length or inferred geometry. Direction means logical index order, not a claimed bidi visual direction.

`model_epoch` is the harness's checked App::update event counter. It is **not an iced layout, composition or selection generation**. Cursor-only native updates need not increment it. `accepted_model_epoch=true` therefore does not prove coherence. `layout_generation` explicitly remains UNAVAILABLE. The getter clamps cursor indices against the supplied Value; after a text-input update but before the on_input model message is processed, that value may be old. All non-redraw samples conservatively mark value_lag_possible=true. The raw phase name after_child_redraw means after the child handles RedrawRequested in update, not after painting or a presentation acknowledgement. Those samples reduce the known edit-message lag risk but do not supply an atomic stock-widget snapshot. Final deduplication retains changed payloads/epochs rather than every redraw, so adjacent A/B samples cannot be assumed to be one atomic frame.

Rectangles are direct logical window-layout API values `[x,y,width,height]`. Screen origin/physical UIA conversion, actual glyph clip and horizontal-scroll offset are UNAVAILABLE. Text-child bounds and the passed viewport are observable context, not invented selection bounds. Synchronous bounded logging may perturb event timing; no performance guarantee is inferred.

## Observation matrix

RUNTIME VERIFIED means actual stock widget/window callbacks were observed, with the driving method stated. SOURCE VERIFIED and unit tests are separate. UNVERIFIED cases are not passed by analogy.

| Requested case | Observation / status | Evidence and limit |
| --- | --- | --- |
| 1. Empty input, collapsed caret | RUNTIME VERIFIED | run-01 A sample 6: empty actual document, local focus, index 0, IME request enabled, preedit null. A rectangle `[32,203.8,1,20.8]`; placeholder is not the document. |
| 2. Insert at beginning, middle, end | RUNTIME VERIFIED with synthetic direct keys | run-01 fixture F6 then `a`, F5/public caret move then `b`, End then `c`. Values at seq 600/730/774 demonstrate each insertion. Initial sky.type_text("abcd") did not insert; retained as failed delivery, not successful typing. |
| 3. Keyboard caret navigation | RUNTIME VERIFIED with synthetic keys | End/Right/Left/Home and Shift+Left delivered to actual stock input; run-01 seq 820 collapsed at 4, run-02 seq 38 at 0, seq 50 at 1. Physical input not tested. |
| 4. Forward/reverse selections | RUNTIME VERIFIED; public selection operations and separate keyboard selection | run-01 seq 806 `(1,4)` forward; seq 812 `(4,1)` reverse. Both IME rectangles x=40. Collapsed index 4 at seq 820 gives x=66. Run-02 reverse `(4,1)` at seq 44 and collapsed 1 at seq 50 both give x=40. |
| 5. Replace/delete across graphemes | RUNTIME VERIFIED with synthetic keys | Shift+Left then x replaces selection (seq 878); subsequent BackSpace removes trailing c, whole woman-ZWJ-laptop, then e+combining acute. seq 1430/1482 show whole-unit deletions, not a complete Unicode/IME conformance test. |
| 6. Resize/input width and viewport context | RUNTIME VERIFIED partial | F7 changes real window 740→390 logical width; F9 changes padding/content positions. run-02 long fixture ends at index 280, seq 32 rectangle x=352.28125 with text bounds width 326; Home/seq 38 rectangle x=32. Exact scroll offset/effective text clip still UNAVAILABLE. |
| 7. Focus two inputs and attribute output | RUNTIME VERIFIED partial; inherited output AMBIGUOUS | With A focused, B's shared Shell contains A's already-enabled output even though B is unfocused. With B focused, A emits no request; B's Disabled→Enabled transition is attributable to B (run-01 seq 1596/1597). B accepted z without changing A. No universal ownership claim from final Shell alone. |
| 8. OS IME begin/update/commit/cancel | UNVERIFIED | No real OS composition session was driven; every captured preedit was null. Input-method configuration query returned null fields and is inconclusive, not proof that no IME exists. No synthetic InputMethod::Preedit injection or system language/settings change. A validated IME/operator session was not available in this run. |
| 9. Hide/close/reopen | RUNTIME VERIFIED with scope limit | F8 public native mode query returned Hidden then Windowed; same run resumed. F10 closed normally; a second process reopened a fresh empty window and closed normally. Same-process destroy/recreate was not tested and is UNVERIFIED. |
| 10. Reordered updates/redraw and stale samples | PARTIAL: deterministic gate + deliberate runtime replay; actual native reorder UNVERIFIED | Four unit tests include stale/closed/exhausted harness epochs. F11 references actual captured seq 1729/epoch 27 after a new app command; accepted_model_epoch=false. No naturally stale model samples occurred in captures; no supported native-event reorder facility was used. This is not an iced layout-generation proof. |

Source-level interpretation matches the runtime: stock TextInput's IME computation chooses start.min(end), while typed cursor state retains anchor/active separately. In the tested reverse LTR selection, the IME anchor coincides with the left active endpoint; in the tested forward selection it does not. Equal forward/reverse IME rectangles plus a distinct collapsed-right rectangle demonstrate that this output is unsuitable as a **general** active caret. Bidi, composition and other directions remain unverified.

Shared-Shell attribution is safe only for the observed Disabled→Enabled transition inside this exact child call. If Shell was already enabled, the final value may be inherited under first-enabled merge policy; the logger explicitly marks AMBIGUOUS_INHERITED_SHARED_SHELL. The experiment stops that attribution path rather than resetting/merging a fresh Shell to make the ambiguity disappear.

## Commands, tests and evidence provenance

Actual final commands:

```powershell
./experiments/iced-ime-observer/Build.ps1 -EvidenceDirectory docs/implementation/evidence-stage2g/build-05
./experiments/iced-ime-observer/Run.ps1 -EvidenceDirectory docs/implementation/evidence-stage2g/run-02
```

Build.ps1 records exact rustfmt/rustc/clippy-driver arguments. Final formatting, rustc test compilation, **4/4 deterministic tests**, strict Clippy (`-D warnings -D clippy::all`) and optimized harness build all exit 0. Tests cover direction preservation, checked Unicode conversions/invalid indices, stale/closed/exhausted harness epochs and JSON escaping. They do not test a mock provider or claim OS behavior. Rustc builds type-check the complete harness; no new Cargo workspace was introduced merely to change a lockfile.

Earlier build failures are preserved: build-01 used an incorrect InputMethod import and a deprecated atomic helper; corrected by the public advanced import and an owner-local checked sequence. build-final used a nonexistent get_mode function; corrected to the verified public window::mode API. build-02/build-04 passed; build-04 is the exact run-01 executable. No private API workaround or dependency patch was used.

Runtime driving used the [computer-use skill](C:/Users/swani/.codex/plugins/cache/openai-bundled/computer-use/26.1013.31256/skills/computer-use/SKILL.md) and @oai/sky synthetic keys on returned, observed owned windows. No physical keyboard or Narrator claim. [Driver notes](evidence-stage2g/driver-notes.md) distinguish key input from public F-key operations/model writes, the failed type_text delivery and one rejected state-refresh option after F5 (reobserved without retrying the key). No PowerShell UIA/SendInput client was mixed with these actions.

run-01 recorded 2048 observation rows and reached its explicit cap; command/mode/replay/exit records remain separate. All cited samples precede the cap. run-02 recorded 56 rows with idle payload deduplication; its exact source differs only by documented fixture/logging additions. Both have zero non-null preedit observations and zero naturally rejected model-epoch samples. [Runtime summary](evidence-stage2g/runtime-summary.json), [key observations](evidence-stage2g/key-observations.json), value transitions and raw JSONL remain separate. Old 29/29 UIA logs are not recopied or rerun as if they were text-pattern evidence.

The runner bounds the exact owned process to ten minutes and uses established kill/wait/dispose cleanup on timeout/error; ordinary exits were observed. The 400-ms hide/show delay is a finite test task, not an accessibility synchronization mechanism. Raw logs plus source/binary/lock identities are retained. Final tracked diff and separate new-source/document checks are recorded; raw failed build captures remain unedited.

## Coherence conclusion and remaining blockers

There is useful authoritative **partial** data: real typed cursor/selection indices, stock child layout bounds, a public viewport parameter, and an actual redraw-time IME request. It is not one coherent document/selection/layout/viewport/composition snapshot. Widget text and model messages can be between phases; Shell can be inherited; native layout/composition epochs and retained shaping/scroll data are absent; composition was not exercised. A positive harness-epoch check cannot close those gaps.

No text runs, selection geometry, visible ranges, text-provider actions or native UIA pattern were added. Existing blockers remain: complete stock-widget geometry/coherence, UIA LabeledBy compatibility, GetVisibleRanges/GetCaretRange provider disposition, live-region event observation, queue saturation/truthful rejection, oversized values, external focus restoration, representative high contrast/DPI/reduced motion, IME/complex text, refreshed physical keyboard/Narrator, and actual launcher/result-list semantics. Headless service acceptance is independent.

**Recommendation:** continue upstream-first and stop treating Shell IME output as a complete text-accessibility adapter. Its collapsed-caret request may merit a separately proposed narrow readback contract; the observed inherited-Shell and endpoint limitations must be included, not worked around or hidden.

Smallest next experiment, requiring explicit project-owner approval: a human-operated real OS IME session against this unchanged final observer, with an identified installed language/input method, begin/update/commit/cancel, resize and input-focus transitions, and per-case logs. If setup/configuration is needed, obtain separate approval rather than altering it now. This can reduce composition uncertainty but cannot fix general text-range geometry. A supported coherent upstream snapshot/provider proposal remains necessary before any future TextPattern work. No production UI or shell work is authorized by this observational PASS.

