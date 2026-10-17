# Stage 2J — Bounded upstream-first review policy

2026-10-17. **CONTINUE — bounded upstream policy defined; non-editable UI remains separately gated.** Option A is the owner's selected strategy. The cadence below is a project recommendation requiring ratification, not an upstream delivery promise or a scheduled job. Production accessibility remains **BLOCKED**. No dependency adoption, UI development, alternative architecture or recurring automation is authorized by this report.

## Baseline and preserved evidence

HEAD `d14d4bede2096d4bea2561de22977d181376b640`, `main`; no tracked changes at entry. Existing untracked work comprised the Stage 2G/H/I reports and evidence plus `experiments/iced-ime-observer/`. Preserve it all. [New evidence](evidence-stage2j/README.md) records actual status, timestamps, checksums and final integrity results.

Read the authoritative [Stage 2I decision](STAGE-2I-ACCESSIBILITY-ARCHITECTURE-DECISION.md) and [evidence](evidence-stage2i/README.md), [gate reassessment](ACCESSIBILITY-GATE-REASSESSMENT.md), [C report](STAGE-2C-ACCESSKIT-REPORT.md)/[acceptance](STAGE-2C-ACCEPTANCE-REPORT.md), [D closure](STAGE-2D-ACCESSIBILITY-CLOSURE.md), [E investigation](STAGE-2E-TEXT-INPUT-CAPABILITY-REPORT.md), [F decision](STAGE-2F-UPSTREAM-COMPATIBILITY-DECISION.md), [G observations](STAGE-2G-IME-OBSERVATION-REPORT.md), [H readiness stop](STAGE-2H-REAL-IME-REPORT.md) and their evidence indexes. Their verdicts and historical failures are unchanged.

Evidence interpretation remains explicit: C/D/E UIA captures prove their named runtime assertions; D physical-keyboard/Narrator results are aggregate human attestations, not transcribed speech or inferred automation; E source/API tests establish limited public readback; G synthetic-key/public-operation observations establish partial stock-widget behavior; H supplies no IME session. J performs research and integrity checks only. Headless Stage 3B acceptance and the headless-only text deferral do not approve UI.

## Recommended cadence and finite horizon

Use **one review per calendar month, with a maximum initial waiting period of three calendar months** from 2026-10-17, in Asia/Calcutta. This is a finite review policy, not a promise to complete accessibility in three months.

| Review | Recommended date | Required disposition |
| --- | --- | --- |
| 00, current baseline | 2026-10-17 | Record current releases, development status and unsatisfied contract; completed here |
| 01 | 2026-11-17 | One bounded metadata/source review; no-change, candidate-for-review, blocked-evidence or unsuitable |
| 02 | 2026-12-17 | Repeat and identify whether there is a concrete released candidate, not merely discussion |
| 03, mandatory owner checkpoint | **2027-01-17** | Stop routine reviews under this policy; owner chooses a new explicitly finite policy, headless-only continuation, separately scoped B/C research, or project pause |

Recommended effort limit: **half an engineer-day per scheduled check**, with at most one network retry on a different day within seven days. A full candidate audit requires separate scope/budget approval; do not hide it inside monthly monitoring. An unscheduled owner-requested review can replace a scheduled check but never extend the horizon. A missed review is recorded as missed, not passed, and does not move 2027-01-17. If the final checkpoint is not ratified or attended, stop automatic continuation of review work; keep the gate blocked pending an owner decision. No recurring work runs until requested, and no automation/scheduled task/reminder was created.

At the horizon, a candidate still being evaluated does not silently extend waiting: owner must explicitly authorize a finite evaluation or renewed review period. No response, a stale issue, an unreleased merge, or a newly published version alone authorizes anything.

## Roles and decisions to ratify

Recommended roles, not assignments: a Rust/Windows maintainer conducts source and license review; an independent reviewer checks evidence/contract coverage; a human Windows operator performs physical keyboard, Narrator and real IME cases when approved; the project owner approves scope, budget, exact dependency experiments and UI-entry decisions. One person may conduct research, but do not label their own review independent.

Owner decisions still needed: ratify cadence/horizon and name reviewers; approve supported Windows builds/languages/modalities; decide whether the [non-editable catalog candidate](STAGE-2J-NONEDITABLE-UI-FEASIBILITY.md) has enough product value to fund separate qualification. Option B/C remain unapproved. Reconsider them only after the horizon, a failed adequate upstream candidate, or an explicit owner decision that upstream timing is unacceptable, with a maintenance owner/budget and isolated stop criteria. Expiration of the wait is not approval of an alternative.

## Exact baseline and official sources

| Component | Locked version / published revision | Official review entry |
| --- | --- | --- |
| iced / iced_core | 0.14.0 / 0.14.0; `3997291f318a8bc06fa522f5579836fb3feb94df` | [Registry](https://crates.io/api/v1/crates/iced), [release](https://github.com/iced-rs/iced/releases/tag/0.14.0), [core source](https://github.com/iced-rs/iced/tree/3997291f318a8bc06fa522f5579836fb3feb94df/core) |
| iced_widget | 0.14.2; `54abf81d13fec06d4d9ac754b03ce2c3313e8a1f` | [Registry](https://crates.io/api/v1/crates/iced_widget), [TextInput source](https://github.com/iced-rs/iced/blob/54abf81d13fec06d4d9ac754b03ce2c3313e8a1f/widget/src/text_input.rs) |
| accesskit | 0.25.1; `ce8164ba92995cfa86005b6259115e08c8244253` | [Registry](https://crates.io/api/v1/crates/accesskit), [semantic source](https://github.com/AccessKit/accesskit/blob/ce8164ba92995cfa86005b6259115e08c8244253/accesskit/src/lib.rs) |
| accesskit_windows | 0.35.1; same AccessKit revision | [Registry](https://crates.io/api/v1/crates/accesskit_windows), [release](https://github.com/AccessKit/accesskit/releases/tag/accesskit_windows-v0.35.1), [provider](https://github.com/AccessKit/accesskit/blob/ce8164ba92995cfa86005b6259115e08c8244253/adapters/windows/src/node.rs) |
| accesskit_consumer | 0.39.1; same AccessKit revision | [Registry](https://crates.io/api/v1/crates/accesskit_consumer), [consumer](https://github.com/AccessKit/accesskit/blob/ce8164ba92995cfa86005b6259115e08c8244253/accesskit_consumer/src/node.rs) |
| Window/native dependencies | iced_winit 0.14.1, winit 0.30.13; direct windows-sys 0.61.2; provider windows 0.62.2 | Actual [I lock inventory](evidence-stage2i/locked-stack.json); inspect official registry/released source for these as well if a candidate changes the graph |

Immutable URLs identify source locations matching published VCS metadata; conclusions use actual cached published bytes, not an unread page. Iced is MIT; AccessKit declares MIT OR Apache-2.0, with inherited notices retained. Workspace/spike declare Rust 1.88; inspected AccessKit packages declare 1.87; historical execution used 1.99.0 and does not qualify minimum compiler compatibility.

Current J check: all five queried registry newest/stable versions equal the existing pins, selected current releases not yanked. Official iced/Windows-provider release records are neither draft nor prerelease. Eleven audited sources still match F/I hashes; five cached crate archives match current official checksums. No new complete release candidate was identified. Current metadata and its response hashes are in [review-00-metadata.json](evidence-stage2j/review-00-metadata.json); release/checksum provenance is indexed separately. This does not claim every unqueried crate is latest.

Read-only development watch list, checked separately from release capabilities:

| Item | Current official metadata result | Meaning |
| --- | --- | --- |
| [iced issue 552](https://github.com/iced-rs/iced/issues/552) | Open | Request/discussion, not implementation evidence |
| [PR 3111](https://github.com/iced-rs/iced/pull/3111) | Open, draft, merged=false; head `2a693ddc9eddd71e0ca635f832301e4ab765b8ed` | Development only; not a dependency candidate |
| [PR 1849](https://github.com/iced-rs/iced/pull/1849) | Open, draft, merged=false; head `bf0deae0f291698eef2dc1330131bd2984ba56aa` | Development only; API page succeeded even though web page retrieval failed |
| [PR 3281](https://github.com/iced-rs/iced/pull/3281) | Closed, merged=false; head `1fcaca78f8ffd8d5a4790cd22e93cb56b46c8f8e` | Not a released upstream fix or authorization to use its fork |

An eventual merged PR only moves to “merged, unreleased” until the relevant code is included in a verified stable published artifact. No branch/PR was installed, built or runtime-tested. No upstream contact was sent. Track additional provider issues only when their relevance and official URL are established; do not invent issue numbers or claim that these iced threads cover AccessKit provider fixes.

## Repeatable read-only review procedure

1. Record UTC observation time and Asia/Calcutta review date, actual Git HEAD/status, protected file hashes, locked component/features and prior unresolved contract rows. Use a fresh `review-NN` directory; never overwrite evidence. Check current work before reading an arbitrary dependency cache variant.
2. Read official crates.io JSON (`/api/v1/crates/<name>`): newest/max-stable, exact version publication date, yanked status and checksum. Read official GitHub releases/tags/changelog at a fixed tag/revision. A moving `latest` documentation page or master branch is insufficient. If metadata fails, record failure and stop conclusions about new versions; use the single bounded retry, then UNVERIFIED.
3. Read the relevant GitHub issue/PR API and page without commenting. Record state, draft, merged flag, head/base revisions and dates. Distinguish stable artifact, official release, merged-unreleased, development branch, draft and speculation in the ledger. Re-scope the watch list if links are superseded, retaining old entries.
4. For a plausible **stable** candidate, inspect published `.crate` source and official versioned API contract without installing/resolving it into the application. Hash archive bytes against registry checksum; inspect `.cargo_vcs_info.json`, exact relevant files and their checksums. Any new archive download must be an approved read-only research artifact, never a silent cache/manifest patch. Changelog claims direct inspection but cannot replace it. Identify all cooperating package versions, features, windows support and source revisions, not just iced's top-level version.
5. Fill every contract row below with version, public API/source path and lines, index/coordinate/lifetime semantics, supported actions, limitation and evidence class. Do not mark runtime acceptance from source. Review license/notices, actual minimum Rust/transitive graph implications, Windows/build compatibility and upstream maintenance; separate an estimated maintenance burden from a support promise.
6. Produce one disposition: NO CHANGE; UNSUITABLE with failed rows; UNVERIFIED with failed evidence retrieval; or CANDIDATE FOR SEPARATE EXPERIMENT APPROVAL. Record owner checkpoint if horizon/budget reached. All but a separately approved experiment leave pins and production gate unchanged. Source success alone cannot produce “production eligible.”

Each check ledger must retain: check ID, date/time, exact URL, retrieval success/failure, release/version/revision, result, API requirement IDs and evidence paths/SHA256. Hash stored files separately from hashes of received UTF-8 response text; if raw response is not retained, say so. Do not present an unretained response hash as a replayable artifact. J stores selected JSON fields plus response-text hashes, with a [file checksum manifest](evidence-stage2j/checksums.json) for reproducible local evidence integrity. Source hashes refer to actual inspected bytes. Future checks should retain minimal relevant raw metadata when practical, excluding unrelated conversation content.

Stop immediately if evidence requires private fields, parsed Debug state, modified registry source, proxy reshaping, competing editing/rendering authority, a fork, unstable unpublished dependencies, Windows settings changes, or unauthorised upstream communication. Record the limitation instead of finding an unapproved workaround. No automated polling, build, runtime test, manifest/lock change or PR contact is part of this review procedure.

## Future candidate acceptance contract

Statuses below refer to the **current baseline**, not a hypothetical candidate. **PARTIAL** means supported released constituent APIs exist but do not satisfy the complete contract. **MISSING** means no defensible public complete path is established or the provider method is absent/incomplete. **UNVERIFIED** means evidence is absent. Historical runtime verification is named explicitly. E/F/I source references and G samples remain linked; no new runtime was executed in J.

| ID | Required supported public semantics | Current stable/source finding | Runtime evidence / remaining acceptance |
| --- | --- | --- | --- |
| A1 | Actual document value with explicit units, excluding placeholder | PARTIAL: application String and Value exist; generic Operation text can be placeholder; no coherent State document snapshot | D/E actual whole-value UIA success, G value readback; joined snapshot still missing |
| A2 | Selection anchor/active endpoint and direction | PARTIAL: State::cursor + Cursor::state preserve logical endpoints; sorted selection helper loses direction | E API tests/G actual forward/reverse readback; no actual UIA Select bridge |
| A3 | Caret-only/selection-only invalidation without value edits | PARTIAL: stock editing updates cursor, no universal snapshot generation exposed | G actual navigation observed, caller epochs not native generations; candidate must prove every selection-only change |
| A4 | Checked UTF-8/UTF-16/grapheme/provider-unit conversion and invalid-bound policy | PARTIAL: Value prefixes and TextRun units exist, no automatic end-to-end contract | E/G sample conversions only; invalid/large clusters, RTL and provider actions must be tested |
| A5 | One editing state drives painting, selection, hit-testing and accessibility operations | MISSING supported complete stock-widget readback/action contract | No competing input/proxy allowed; demonstrate exact candidate single authority |
| B1 | Retained shaping/layout for displayed text, including placeholder/preedit distinctions | MISSING: stock retained paragraphs private | E negative compile/source F/I; public Paragraph methods do not grant the retained instance |
| B2 | Character/range rectangles, active caret and real hit-testing | MISSING at stock readback boundary; Shell IME anchor is not arbitrary range geometry | G equal forward/reverse rectangles and inherited output; candidate needs exact real geometry tests |
| B3 | Visible ranges, effective clip/viewport/alignment/horizontal scroll and transforms | MISSING complete editor/provider path; widget layout bounds/viewport are partial context | G resize/long text not exact clip/scroll; candidate must agree with actual visible text after scroll/DPI |
| B4 | Document/selection/layout/composition generation or equivalent coherent invalidation | MISSING native supported joined snapshot contract | D protects some app async messages; G model epoch/redraw callback is not atomic layout or paint proof |
| B5 | Coordinate space, lifetime and display-change correlation documented | PARTIAL: logical bounds and native conversion constituents; full range/coherence unverified | D current-DPI widget bounds only; actual candidate resize/mixed-DPI/display invalidation required |
| C1 | Actual preedit/composition state and unambiguous editor ownership | PARTIAL: Shell Enabled carries preedit; private retained preedit; inherited shared output ambiguous | G all preedit null, H no session. Candidate cannot infer owner from final shared Shell |
| C2 | Begin/update/commit/cancel and unit semantics | PARTIAL stock IME event handling, not a coherent supported accessibility snapshot | UNVERIFIED real OS lifecycle; human installed-IME session mandatory, no synthetic preedit |
| C3 | Candidate placement from real active editing/composition state | PARTIAL actual IME request exists; selection uses min endpoint rather than general active endpoint | G noncomposition endpoints only; actual OS candidate placement unverified |
| C4 | Focus switch, hide/show, dismissal and teardown during/after composition | UNVERIFIED complete IME/lifecycle contract | G hide/show/normal close without IME; H none. No accidental submit/dismiss midcomposition |
| D1 | ValuePattern and TextPattern backed by same real editor | PARTIAL released provider, conditional on valid TextRuns | D/E Value works; E real TextPattern=false; no inferred text acceptance |
| D2 | DocumentRange/GetText/GetSelection/Select/RangeFromPoint | PARTIAL released provider implementations, missing authored editor data and action mapping | No real UIA text-range operations passed; test each on exact candidate |
| D3 | Range rectangles and visible ranges | MISSING complete path; pinned GetVisibleRanges returns null | No geometry/visibility runtime proof; provider fix and editor snapshot are separate |
| D4 | TextPattern2/GetCaretRange, or explicit approved evidence-backed client coverage | MISSING in inspected provider; degenerate GetSelection is a different contract | Unresolved owner/provider/client decision; no waiver here |
| D5 | LabeledBy references actual visible associated label | MISSING provider mapping; authored semantic relation alone insufficient | C/E actual absent relationship; candidate needs real property-target assertion |
| D6 | Text/selection changed events | PARTIAL provider source has text event machinery; actual stock text model unintegrated | Runtime event sequence/selection-only updates unverified; names of events are not emitted evidence |
| D7 | Error exposure/removal and live-region events with stable focus | PARTIAL supported Live semantics and occurrence IDs | D UIA removal and attested speech; independent native event counts remain unverified |
| D8 | Honest invalid/oversized/stale/closed/overloaded action rejection | PARTIAL removed/closed checks; existing queue drops/4096-byte limit unresolved | D stale-element pass; overload/oversize failure policy and runtime observation missing |

An equivalent public contract need not use any proposed method name, but must establish all semantics and lifetimes above. Do not request merely a private paragraph getter. A5/B1–B5/C1–C4 require correlation with the editor's actual state, including selection-only changes and composition. Text-state APIs alone cannot repair provider D3–D5; a provider alone cannot invent editor geometry. No row is deferred for an editable production launcher.

## Candidate adoption sequence

**Gate 1, research:** exact stable cooperating versions and published revisions, matching checksums, public documented contract and source implementation satisfy every applicable row without workaround. Report remaining runtime rows as NOT RUN. No upgrade yet.

**Gate 2, owner approval:** license/notices, Rust/MSRV, features/transitives, Windows support and maintenance reviewed; approve a separate isolated candidate experiment and its exact dependency/lock scope. Approval of a review, a PR or an IME session is not Gate 2. Nothing in J grants it.

**Gate 3, isolated qualification:** only after Gate 2, retain the 29 baseline checks and add actual cross-process text patterns/ranges/labels/actions/event traces, coherent geometry after edits/selection/scroll/resize/DPI, real IME, rejection/stale/teardown cases, and human physical keyboard/Narrator. Record hashes/environment and speech separately from UIA. Required unavailable cases block, not pass. UIA/provider gaps get their own per-method acceptance results.

**Gate 4, independent UI-entry decision:** apply I's full Entry rows, including focus restoration, queue/oversize policy, high contrast/nondefault DPI/reduced motion and supported Windows scope. Only a separate explicit UI-entry review can authorize a bounded feature. Actual production controls then need release qualification; no source-only gate or headless PASS substitutes.

## Current disposition

Bounded policy is defined; owner ratification/role assignment remains outstanding. Keep Option A and current pins. A useful non-editable catalog is a conditional product hypothesis, not an approved fallback or a way to bypass shared provider risks; see the [separate feasibility assessment](STAGE-2J-NONEDITABLE-UI-FEASIBILITY.md). Continue headless work only within existing authorizations until the relevant UI-entry gate passes.

Only documentation/research integrity checks ran in J; exact results are in [final-checks.json](evidence-stage2j/final-checks.json). No builds or runtime tests ran. Production source/designs/manifests/lockfiles and all historical evidence remain unchanged. No Windows configuration change, automation, commit or push.
