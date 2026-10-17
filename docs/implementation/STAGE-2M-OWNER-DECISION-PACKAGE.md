# Stage 2M — Owner decision package

2026-10-17. **READY FOR OWNER INPUT — decision package prepared; no authorization granted.**

Stage 2K experiment: **NO-GO**. Production accessibility: **BLOCKED**. Production UI/shell integration: **unauthorized**. O01–O07 remain **UNRESOLVED**. This package offers choices to the actual authorized owner; it does not impersonate that person, assign staff, approve spend or commit support.

## Baseline, sources and limits

Actual HEAD `d14d4bede2096d4bea2561de22977d181376b640`, branch `main`; no tracked modifications at entry. Prior untracked G–L reports/evidence and the G observer are preserved. [Baseline](evidence-stage2m/baseline.json) records full pre-task status and exact existing untracked file list; [preservation](evidence-stage2m/preservation-before.json) hashes all 620 existing tracked/nonignored untracked files. No reset, clean, stash, commit or push occurred.

Read [I architecture decision](STAGE-2I-ACCESSIBILITY-ARCHITECTURE-DECISION.md)/[evidence](evidence-stage2i/README.md), [J policy](STAGE-2J-UPSTREAM-REVIEW-POLICY.md)/[noneditable feasibility](STAGE-2J-NONEDITABLE-UI-FEASIBILITY.md)/[evidence](evidence-stage2j/README.md), [K owner record](STAGE-2K-OWNER-DECISIONS.md)/[plan](STAGE-2K-NONEDITABLE-QUALIFICATION-PLAN.md)/[evidence](evidence-stage2k/README.md) and [L provider review](STAGE-2L-UPSTREAM-PROVIDER-CAPABILITY-REVIEW.md)/[evidence](evidence-stage2l/README.md). The **exact existing data structure** is [K owner-decision-record.json](evidence-stage2k/owner-decision-record.json): schema_version 1, seven id/subject/recommendation/required_record/status/decision/approved_by/approved_at/authority_reference records, four null role assignments and experiment flags false. K is not rewritten.

Existing decisions are limited: Option A upstream-first is selected; D's text-capability deferral permits headless work only. Neither constitutes cadence ratification, staffing, product/supported-platform commitments or experiment consent. Previously accepted headless service/consumer results remain scoped historical acceptance; no test was rerun in M.

L's recorded source/release review found no demonstrated stable provider path for either authored visible-label **LabeledBy** or container **ScrollPattern**. That is dated archived evidence, not a claim that M searched today's registry again. A correct Name is not LabeledBy; ScrollItem is not container ScrollPattern. The open proposal recorded in L is not a stable candidate or runtime proof.

## Decision matrix for the owner

Questions below are copied **verbatim from K's `subject` fields**, not new paraphrased questions or presumed decisions. Each detailed record carries the original K `required_record` verbatim in [decision-traceability.json](evidence-stage2m/decision-traceability.json), with exact JSON pointers, evidence paths, options, unknowns, consequences, authority and closure information. Every new owner-value/approver/date/authority field is null.

| ID | Exact Stage 2K question | Current status | Information not yet settled |
| --- | --- | --- | --- |
| O01 | Review cadence and finite horizon | **UNRESOLVED** | Whether owner ratifies cadence, effort cap, checkpoint, missed reviews and renewal; no approved calendar or continuing review budget. |
| O02 | Named accountable roles | **UNRESOLVED** | Actual owner/delegates, named willing maintainer/reviewer/operator, independence, availability and authority boundaries. |
| O03 | Supported configurations and accessibility modalities | **UNRESOLVED** | Exact supported builds/architectures, layouts/languages, AT versions, display profiles, tester access and numeric resource/geometry thresholds. |
| O04 | Product value of a limited noneditable catalog | **UNRESOLVED** | Whether no-search browsing has enough user value, target users and acceptable usability trade-offs; no user validation or scope exception approval. |
| O05 | Authorize an isolated experiment with bounded scope | **UNRESOLVED** | No actual authorization, exact candidate versions/paths, approved effort/run limits, operator/profile/resource budgets or real-fixture phase permission. |
| O06 | Approved upstream resolution of label/container scrolling gaps | **UNRESOLVED** | No adequate stable candidate or approved candidate-adoption scope; native target/lifecycle/scroll properties/actions remain untested. |
| O07 | Triggers for reconsidering Options B or C | **UNRESOLVED** | Whether and when owner would fund B/C investigation, who maintains it, acceptable scope/cost and stop rules. |

No checked/default option is an owner response. APPROVE, REJECT and DEFER describe possible future responses to an exact scope. An approval to assess value or review a candidate is not approval to change dependencies, run a GUI or build production UI.

### O01 — Review cadence and finite horizon

**Exact K question:** “Review cadence and finite horizon”. Source: [K decision schema](evidence-stage2k/owner-decision-record.json), `/decisions/0/subject`. **Current status: UNRESOLVED.**

**Why now:** Further reviews need a finite effort/checkpoint policy; prior dates are recommendations and no review automation is authorized.

**Available evidence:** [STAGE-2J-UPSTREAM-REVIEW-POLICY.md](STAGE-2J-UPSTREAM-REVIEW-POLICY.md), [STAGE-2K-OWNER-DECISIONS.md](STAGE-2K-OWNER-DECISIONS.md), [retrieval-ledger.json](evidence-stage2l/retrieval-ledger.json). Evidence is historical or planning, not new native acceptance.

**Unknown:** Whether owner ratifies cadence, effort cap, checkpoint, missed reviews and renewal; no approved calendar or continuing review budget.

**Realistic owner choices, not decisions recorded here:**

- **A.** APPROVE J's proposed bounded policy explicitly, with an accountable reviewer and recorded scope. Manual requested reviews follow the ratified limits; no scheduling or dependency approval.
- **B.** APPROVE a different explicit finite interval/horizon and effort limit. New written policy required; does not erase historical review dates.
- **C.** REJECT the recurring review proposal or DEFER the decision; keep research request-by-request/no-go. No recurring commitment; pins and experiment gates unchanged.

**Consequences/effort basis:** J recommends half an engineer-day per monthly review and a three-calendar-month horizon; these are planning recommendations, not approved time/spending or upstream promises.

**Authority and prerequisites:** Actual project owner or explicitly authorized delegate for review scope/time; name and delegation are not established here. Name reviewer, ratify bounded dates/effort, specify failure/retry/checkpoint handling without creating automation.

**Information required to close:** An attributable APPROVE/REJECT/DEFER for O01, exact start/interval/end or no-recurring policy, reviewer effort limit, missed/failed review treatment and owner checkpoint/extension rule.

### O02 — Named accountable roles

**Exact K question:** “Named accountable roles”. Source: [K decision schema](evidence-stage2k/owner-decision-record.json), `/decisions/1/subject`. **Current status: UNRESOLVED.**

**Why now:** No candidate experiment can rely on invented staff or unavailable physical/AT operators; independent review is a distinct role.

**Available evidence:** [owner-decision-record.json](evidence-stage2k/owner-decision-record.json), [STAGE-2K-NONEDITABLE-QUALIFICATION-PLAN.md](STAGE-2K-NONEDITABLE-QUALIFICATION-PLAN.md), [manual-attestation.json](evidence-stage2d/manual-attestation.json), [operator-readiness.json](evidence-stage2h/operator-readiness.json). Evidence is historical or planning, not new native acceptance.

**Unknown:** Actual owner/delegates, named willing maintainer/reviewer/operator, independence, availability and authority boundaries.

**Realistic owner choices, not decisions recorded here:**

- **A.** APPROVE named role assignments and delegation boundaries after confirming availability. Assignments create responsibility only, not experiment or spending approval.
- **B.** APPROVE a limited research-only team while leaving operator-dependent work no-go. No native qualification without appropriately assigned available testers and separate approval.
- **C.** REJECT proposed staffing or DEFER until people/authority are available. No personnel assigned by this package; runtime remains no-go.

**Consequences/effort basis:** No staffing capacity, hiring cost or budget is established in the record; no new estimate.

**Authority and prerequisites:** Actual owner or authorized personnel/resource delegate; independent reviewer must actually be independent of the implementation under review. Verify authority and willing availability; do not derive owner or staff from local username, Git identity or previous aggregate test attestation.

**Information required to close:** Named Rust/Windows maintainer, independent evidence reviewer, human accessibility operator and project owner/delegate; responsibilities, availability, independence and authority evidence.

### O03 — Supported configurations and accessibility modalities

**Exact K question:** “Supported configurations and accessibility modalities”. Source: [K decision schema](evidence-stage2k/owner-decision-record.json), `/decisions/2/subject`. **Current status: UNRESOLVED.**

**Why now:** Current native evidence covers a specific host; proposed catalog acceptance needs explicit build/display/input/AT scope and measurable limits.

**Available evidence:** [STAGE-2I-ACCESSIBILITY-ARCHITECTURE-DECISION.md](STAGE-2I-ACCESSIBILITY-ARCHITECTURE-DECISION.md), [STAGE-2K-NONEDITABLE-QUALIFICATION-PLAN.md](STAGE-2K-NONEDITABLE-QUALIFICATION-PLAN.md), [qualification-cases.json](evidence-stage2k/qualification-cases.json), [README.md](evidence-stage2d/README.md). Evidence is historical or planning, not new native acceptance.

**Unknown:** Exact supported builds/architectures, layouts/languages, AT versions, display profiles, tester access and numeric resource/geometry thresholds.

**Realistic owner choices, not decisions recorded here:**

- **A.** APPROVE a versioned entry-test matrix plus distinct release matrix, with all mandatory K cases retained. A scoped qualification target, not a claim those configurations already pass.
- **B.** APPROVE only a research-host inventory; reserve product support commitments. Inventory may inform planning but cannot close release or experiment profile requirements.
- **C.** REJECT support commitment or DEFER until hosts/operators are available. No unsupported Windows/AT coverage claims; qualification blocked.

**Consequences/effort basis:** Historical one-host short measurements are not performance budgets or support cost estimates; broader qualification effort is not quantified.

**Authority and prerequisites:** Actual owner or expressly delegated product/support authority, informed by Windows/accessibility reviewers; none is assigned here. Available approved test environments/operators, documented entry/release split; separate consent for any configuration changes.

**Information required to close:** Versioned exact Windows edition/build/architecture, keyboard/layout/language, monitor/origin/DPI, contrast/motion and AT matrix; mandatory versus excluded cases, numeric queue/metadata/resource/geometry limits and evidence/change-consent plan.

### O04 — Product value of a limited noneditable catalog

**Exact K question:** “Product value of a limited noneditable catalog”. Source: [K decision schema](evidence-stage2k/owner-decision-record.json), `/decisions/3/subject`. **Current status: UNRESOLVED.**

**Why now:** The noneditable catalog is a conditional product hypothesis, distinct from the approved searchable catalog design.

**Available evidence:** [STAGE-2J-NONEDITABLE-UI-FEASIBILITY.md](STAGE-2J-NONEDITABLE-UI-FEASIBILITY.md), [STAGE-2K-NONEDITABLE-QUALIFICATION-PLAN.md](STAGE-2K-NONEDITABLE-QUALIFICATION-PLAN.md), [SEI-DESK-UI-UX-SPEC.md](../design/SEI-DESK-UI-UX-SPEC.md). Evidence is historical or planning, not new native acceptance.

**Unknown:** Whether no-search browsing has enough user value, target users and acceptable usability trade-offs; no user validation or scope exception approval.

**Realistic owner choices, not decisions recorded here:**

- **A.** APPROVE the limited catalog's value and exact qualification-scope exception, not implementation. Allows consideration of O05; does not approve the experiment or replace approved designs.
- **B.** DEFER product-value decision pending a separately requested owner/user assessment. No user research, feature build or design change is started here.
- **C.** REJECT the limited catalog and retain headless scope. No reduced product UI; editable launcher and shell remain independently blocked.

**Consequences/effort basis:** J records modest conditional benefit and large-catalog usability risk, not validated benefit, ROI or a delivery estimate.

**Authority and prerequisites:** Actual product/project owner or explicitly delegated scope authority. Acknowledge no editable input/search/type-ahead/global activation and existing grid/search design distinction.

**Information required to close:** Explicit value decision, intended users/use case, scope and constraints, reason for any isolated design exception, and whether O05 should even be considered.

### O05 — Authorize an isolated experiment with bounded scope

**Exact K question:** “Authorize an isolated experiment with bounded scope”. Source: [K decision schema](evidence-stage2k/owner-decision-record.json), `/decisions/4/subject`. **Current status: UNRESOLVED.**

**Why now:** A request to prepare or approve plans is not permission to implement a dependency-changing experiment; known provider prerequisites still fail.

**Available evidence:** [STAGE-2K-NONEDITABLE-QUALIFICATION-PLAN.md](STAGE-2K-NONEDITABLE-QUALIFICATION-PLAN.md), [qualification-cases.json](evidence-stage2k/qualification-cases.json), [STAGE-2L-UPSTREAM-PROVIDER-CAPABILITY-REVIEW.md](STAGE-2L-UPSTREAM-PROVIDER-CAPABILITY-REVIEW.md), [owner-decision-record.json](evidence-stage2k/owner-decision-record.json). Evidence is historical or planning, not new native acceptance.

**Unknown:** No actual authorization, exact candidate versions/paths, approved effort/run limits, operator/profile/resource budgets or real-fixture phase permission.

**Realistic owner choices, not decisions recorded here:**

- **A.** APPROVE a precisely bounded conditional Phase A authorization only after named prerequisites and source compatibility evidence exist. No execution while O06/provider/operator/profile gates remain unmet; fixed nonpersonal data and zero real app launches initially.
- **B.** Approve only a separately requested read-only candidate audit; DEFER experiment authorization. No manifests/locks/runtime changed and no native tests.
- **C.** REJECT or DEFER any experiment; remain no-go. Existing services/evidence retained; no spend or implementation inferred.

**Consequences/effort basis:** K's ten-minute per-owned-run cap and fixed fixture sizes are proposals, not approved total effort/budget; no experiment cost is established.

**Authority and prerequisites:** Actual owner or expressly delegated authority for the exact code/dependency/time/test scope; not automatic authority of a tester or reviewer. Applicable O01-O04/O06 decisions; stable provider support, license/MSRV/transitive review, available testers, bounded command/cleanup design and exact allowed paths/dependency changes.

**Information required to close:** Explicit phase A decision with exact source/dependency/lock scope, named reviewers/operator, dates or engineer-hour/run cap, approved numeric limits/stop conditions and cleanup; Phase B exact harmless-fixture launch requires separate authorization.

### O06 — Approved upstream resolution of label/container scrolling gaps

**Exact K question:** “Approved upstream resolution of label/container scrolling gaps”. Source: [K decision schema](evidence-stage2k/owner-decision-record.json), `/decisions/5/subject`. **Current status: UNRESOLVED.**

**Why now:** L found neither required Windows-provider path in the pinned stable stack; owner approval cannot turn an absent mapping into a verified capability.

**Available evidence:** [STAGE-2L-UPSTREAM-PROVIDER-CAPABILITY-REVIEW.md](STAGE-2L-UPSTREAM-PROVIDER-CAPABILITY-REVIEW.md), [labeledby-audit.json](evidence-stage2l/labeledby-audit.json), [scrollpattern-audit.json](evidence-stage2l/scrollpattern-audit.json), [pr-816-metadata.json](evidence-stage2l/pr-816-metadata.json), [STAGE-2J-UPSTREAM-REVIEW-POLICY.md](STAGE-2J-UPSTREAM-REVIEW-POLICY.md). Evidence is historical or planning, not new native acceptance.

**Unknown:** No adequate stable candidate or approved candidate-adoption scope; native target/lifecycle/scroll properties/actions remain untested.

**Realistic owner choices, not decisions recorded here:**

- **A.** APPROVE the evidence requirements for a future upstream-supported candidate, retaining both blockers until satisfied. Policy acceptance is not adoption; requires exact published source then separate isolated authorization/runtime evidence.
- **B.** Request a bounded read-only recheck only if a concrete relevant artifact/change justifies it. Requires its own research request; no monitoring or PR contact created.
- **C.** REJECT adoption or DEFER remediation decision and retain pins/no-go. No label removal, ScrollItem substitution, native override or silent pagination.

**Consequences/effort basis:** L supplied focused source evidence, not remediation cost or release timing; no engineering estimate for an absent candidate.

**Authority and prerequisites:** Actual owner or explicitly delegated dependency/compatibility authority, with source and native evidence review. Exact stable cooperating versions/revisions/checksums, truthful label-target and complete container-scroll semantics, license/MSRV/transitive/Windows compatibility audit.

**Information required to close:** Explicit remediation/adoption-policy decision identifying required evidence and review authority; exact candidate if one exists, isolated change scope and native acceptance prerequisites. Unresolved technical capability must remain blocked regardless of approval.

### O07 — Triggers for reconsidering Options B or C

**Exact K question:** “Triggers for reconsidering Options B or C”. Source: [K decision schema](evidence-stage2k/owner-decision-record.json), `/decisions/6/subject`. **Current status: UNRESOLVED.**

**Why now:** Finite waiting needs a decision trigger, but no architecture alternative or implementation is authorized by expiry or an open PR.

**Available evidence:** [STAGE-2I-ACCESSIBILITY-ARCHITECTURE-DECISION.md](STAGE-2I-ACCESSIBILITY-ARCHITECTURE-DECISION.md), [STAGE-2J-UPSTREAM-REVIEW-POLICY.md](STAGE-2J-UPSTREAM-REVIEW-POLICY.md), [STAGE-2K-OWNER-DECISIONS.md](STAGE-2K-OWNER-DECISIONS.md), [STAGE-2L-UPSTREAM-PROVIDER-CAPABILITY-REVIEW.md](STAGE-2L-UPSTREAM-PROVIDER-CAPABILITY-REVIEW.md). Evidence is historical or planning, not new native acceptance.

**Unknown:** Whether and when owner would fund B/C investigation, who maintains it, acceptable scope/cost and stop rules.

**Realistic owner choices, not decisions recorded here:**

- **A.** APPROVE explicit reconsideration triggers only, retaining A until a new scoped decision. A trigger requests a decision; it does not authorize custom input/native boundary/framework work.
- **B.** REJECT B/C for now and continue only bounded upstream-first/headless choices. No automatic framework switch or workaround.
- **C.** DEFER reconsideration policy; retain current no-go and seek owner input at an explicitly chosen checkpoint. No new checkpoint or review is scheduled here; no response is not approval.

**Consequences/effort basis:** I has historical rough A/B/C planning estimates with assumptions; they are not measured quotes, current budgets, spending consent or guaranteed timelines. No fresh estimate.

**Authority and prerequisites:** Actual architecture/product/resource owner or explicit delegate; identity/authority not inferred here. Named maintenance ownership, approved finite investigation budget/scope, gate-preserving acceptance criteria; any dependency/framework/implementation change needs its own explicit authorization.

**Information required to close:** Trigger conditions, decision authority, optional evidence/review date, permitted research-only scope, maintenance/budget/stop prerequisites; separate future approval needed to start B or C.


## Authorization ladder — separate decisions, no automatic promotion

| Boundary | What the record permits or establishes | Still required / specifically not authorized |
| --- | --- | --- |
| 1. Completed read-only research | I–L decisions, source audits and historical evidence can inform owner choices. M checks documentation integrity only. | No fresh upstream/native/manual test claim; no presumption that an implementation passed. |
| 2. Further bounded read-only upstream review | A separately requested review may follow J's official-source, exact-artifact and bounded-retry procedure. Current M request covers package preparation only. | O01's recurring dates/effort remain unratified. No autonomous monitoring, reminders, polling, scheduling or upstream messages. A concrete change or explicit review request is needed. |
| 3. Dependency-changing isolated experiment | No current authorization. A future exact stable cooperating candidate can be considered under J/K after source/compatibility evidence. | Actual owner scope approval **before** any manifest/lock/cache/implementation change; named reviewer, budget, allowed paths/versions, cleanup/stop criteria. L currently leaves both prerequisites unmet. |
| 4. Native Windows accessibility qualification | No current authorization or assigned testers. K specifies fixed nonpersonal data and zero real launches initially; later harmless-fixture phase is distinct. | Suitable approved host/build/display/AT profiles, willing qualified physical-keyboard/Narrator testers, native UIA/event observer, exact binary identity and phase approval. Native events, physical input and speech need their own observed results. |
| 5. Non-editable UI entry | No current UI-entry permission. K's applicable R01–R22/case matrix is the independent acceptance contract. | Source plus native/manual evidence for label/list/scroll/actions/events/focus/display/rejection/lifecycle/resource cases, then separate explicit UI-entry decision. Passing an experiment alone is insufficient. |
| 6. Production editable launcher and shell | Still unauthorized; headless deferral never approves them. | Editable text/caret/ranges/IME and actual-control acceptance; independently approved product scope and Windows shell coexistence/recovery/focus/fullscreen/display tests. A noneditable slice does not close those gates. |

O05 approval without a supported provider, operator/profile availability or its stated prerequisites remains **non-executable conditional scope**, not GO. O06 policy agreement cannot make absent APIs appear. Staffing/support/product decisions do not themselves approve spend or implementation. Native qualification success cannot silently authorize either production construction or release.

## Bounded recommendation and change conditions

**Recommended action now:** hand this prepared package to the actual owner for attributable O01–O07 responses. No message is sent on their behalf. Until responses and technical prerequisites exist, keep current pins, preserve evidence and stay within previously authorized headless scope. No additional broad source survey is needed merely to prepare those responses.

If the owner wants a finite upstream-first policy, J's proposed monthly reviews on **2026-11-17**, **2026-12-17** and checkpoint **2027-01-17**, Asia/Calcutta, are available for O01 consideration. They remain **recommendations, not scheduled commitments**. No owner response, missed review or proposed release automatically extends the horizon or approves Options B/C.

The recommendation changes only with concrete evidence/authority:

- A relevant exact stable published candidate addressing **both** provider contracts may justify a separately requested bounded source/compatibility audit, not immediate adoption. An open PR, merge without release, two named methods or a newer version number alone is insufficient.
- Actual owner consent to a precisely limited experiment matters only after its named prerequisites and compatible-source/host/operator conditions are met. Then seek that distinct authorization gate; M does not start it.
- A REJECT or DEFER on catalog value, staffing, support or experiment funding keeps the relevant work no-go; it does not authorize a framework switch or a narrower UI that evades the blockers.
- Explicit owner approval of B/C reconsideration triggers allows a new scoped **decision**, not an automatic custom widget/native boundary implementation. I's historical estimates must be reviewed under actual scope and maintenance ownership.

No new budget, staffing estimate, support commitment, release date or upstream delivery expectation is supplied. The only effort recommendations referenced are already in I/J/K, with their original uncertainty.

## Blank owner response template

Use one response per decision. Allowed decision values are `APPROVE`, `REJECT`, `DEFER`; choose the exact intended option/scope and rationale, not an ambiguous blanket approval. Dates are supplied by the actual owner, never prefilled from this package's preparation date. Review dates do not schedule automation.

| Owner-supplied field | Value |
| --- | --- |
| Decision ID | |
| Decision | |
| Rationale | |
| Scope and constraints | |
| Authorized person/role | |
| Approval date | |
| Required evidence, if applicable | |
| Review date, if applicable | |
| Authority/delegation reference | |

Equivalent blank JSON is [owner-input-template.json](evidence-stage2m/owner-input-template.json); every owner input value is null. Template preparation, field examples, permitted enum values and the words APPROVE in the options are **not consent**. No role or person is assigned by the form.

On actual input, preserve the original response, attributable authority/delegation, scope, date and evidence references in a separately authorized follow-up; retain K's historical unresolved record. Confirm completeness and remaining blockers before recording any prospective authorization. DEFER/REJECT resolve only the stated decision's disposition; they never grant experiment or UI authority. Do not automatically run actions after importing a response.

## Evidence, validation and exact changed scope

Added this package and [fresh M evidence](evidence-stage2m/README.md) only. [Checksums](evidence-stage2m/checksums.json) and [final checks](evidence-stage2m/final-checks.json) enumerate exact added paths, stored-file hashes, baseline preservation, JSON parsing, exact K schema/subject/required-record matching, null template fields, local links and whitespace/`git diff --check` results.

These are **documentation checks**, not independent reruns of prior acceptance, native UIA, source compatibility experiments or manual AT tests. No new upstream fetch/contact, builds, GUI, UIA, physical keyboard, Narrator, IME, runtime, settings, automation, source/dependency/manifest/lock/design/prior-decision change, commit or push.

**READY FOR OWNER INPUT — decision package prepared; no authorization granted.** Experiment NO-GO; production accessibility BLOCKED; production UI/shell unauthorized.
