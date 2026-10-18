# Stage 5C — Owner Decision Packet

Prepared 2026-10-17, Asia/Calcutta. **For owner response only. B01–B07 remain UNRESOLVED.** No approver, authority, signature or approval date is recorded. This preparation request and the recommended choices below are not formal approval.

## Proposal presented for decision

Retain Rust + iced and the upstream-first holding strategy. Propose an evidence checkpoint on **2026-11-17** and an owner decision checkpoint on **2027-01-17**, both in Asia/Calcutta. These are proposed decision dates, not scheduled jobs, delivery promises or commitments. The older December review proposal in Stage 5B/2J remains historical and unratified; it is not silently included in this two-checkpoint proposal.

Do not begin an AccessKit/provider experiment until an exact cooperating candidate passes source/API preflight and the owner explicitly authorizes its scope, cost and acceptance criteria. Keep native Windows controls as a conditional fallback needing separate approval; do not automatically research by prototyping, migrate or replace the GUI. Keep production accessibility and release acceptance blocked until all applicable semantic, platform and human-operator gates pass. Passing an isolated experiment would still require separate production integration and release decisions.

## Facts, recommendations and unknowns

| Classification | Decision-relevant statement | Trace / limit |
|---|---|---|
| Verified repository fact | Branch `main`, HEAD `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`. Stage 5A changes and Stage 5B documentation are uncommitted and must be preserved. | Entry Git inspection and final integrity checks below. |
| Recorded production evidence | Stage 5A observed only a native Window in the dashboard UIA raw/control trees; actual client controls/text/selection/status semantics were absent. Its final ordinary suite passed 119 tests; the explicit 50-test Windows fixture suite overlaps it. | [Stage 5A](STAGE-5A-STABILIZATION-ACCEPTANCE.md), [evidence](evidence-stage5a/README.md). Historical tests, not rerun in 5C; reliability success is not accessibility acceptance. |
| Recorded technical evidence | Stage 5B established no adequate released candidate for both coherent iced text state/geometry and required Windows label/scroll/text-provider behavior in its inspected snapshot. | [Stage 5B](STAGE-5B-ACCESSIBILITY-ARCHITECTURE-DECISION.md), [dated metadata/source checks](evidence-stage5b/README.md). No new upstream search or current-status claim in 5C. |
| Recorded human-test limit | Current production Narrator, physical-keyboard and real composition-capable IME qualification was not run; the Stage 5A operator said unavailable. | Prior isolated bridge testimony is not production evidence and does not establish present staffing or IME availability. |
| Existing limited decision | Upstream-first is the recorded holding strategy. Earlier text-capability deferral applies to headless services only. | [2I](STAGE-2I-ACCESSIBILITY-ARCHITECTURE-DECISION.md), [2K](STAGE-2K-OWNER-DECISIONS.md), [2M](STAGE-2M-OWNER-DECISION-PACKAGE.md). No experiment, dates, staffing or support commitments follow from that deferral. |
| Recommendation | Use the proposal above and retain the full Stage 5B acceptance gates. | Offered for response, not selected on the owner's behalf. |
| Unknown | Actual authorized owner/delegation, willing personnel, support profiles, exact adequate candidate, cost/run limits, resource thresholds and adoption timetable. | Must be supplied or explicitly deferred; no values, people, candidate or budgets invented here. |

## B01–B07 decision choices

For every ID, choose **APPROVE**, **REJECT** or **DEFER** and state the exact scope. No option is checked or preselected. Approval of a policy does not approve implementation. Missing prerequisites keep dependent execution stopped even after a policy response. Rejection or deferral is a disposition of that question, not permission to pursue another route.

### B01 — Strategy and finite checkpoints

Trace: Stage 5B B01; historical O01/O07.

- **APPROVE:** retain Rust + iced/upstream-first with the proposed 2026-11-17 evidence checkpoint and 2027-01-17 decision checkpoint, or record explicitly revised finite dates and scope.
- **REJECT:** reject this waiting/checkpoint policy and state the objection or request an alternative decision proposal. No architecture switch follows.
- **DEFER:** leave checkpoint ratification open; retain the blocked gate and request-by-request documentation scope without a continuing review commitment.

**Prerequisites and cost:** owner-supplied review responsibility, effort cap and treatment of missed/failed checkpoints or extensions. Review effort and upstream waiting cost remain unapproved and uncertain; no delivery estimate is promised. Approval establishes a bounded policy only. It creates no reminders, automation, standing job, upstream contact, dependency adoption or experiment. At the final checkpoint, absent a new explicit decision, stop routine continuation rather than silently extending the wait or activating a fallback.

### B02 — Accountable roles and available operators

Trace: B02 / O02.

- **APPROVE:** accept an owner-supplied role/availability plan naming the Rust/Windows maintainer, independent evidence reviewer, project decision authority and willing human Narrator/physical-keyboard/IME operator.
- **REJECT:** reject the supplied plan and identify missing responsibility, independence or availability.
- **DEFER:** keep staffing unresolved; experiments and mandatory human qualification cannot proceed on assumed availability.

**Prerequisites and cost:** actual accepted names or role identifiers, delegation where applicable, responsibilities and availability; none is assigned here. Adapter and future compatibility maintenance require sustained ownership; operator/reviewer time has no approved budget. Role approval authorizes accountability within the stated scope, not spending, Windows setting changes, a test session, provider construction or release. Git identity and the local Windows account do not identify an authorized owner.

### B03 — Support and qualification profile

Trace: B03 / O03.

- **APPROVE:** accept an owner-supplied versioned support/test profile and numeric bounds.
- **REJECT:** return that profile for revision; the observed host must not become an implied support promise.
- **DEFER:** leave supported configurations and resource acceptance unresolved; dependent qualification remains stopped.

**Prerequisites and cost:** exact Windows builds/architectures, AT versions, keyboard layouts/languages, real IME, prepared high-contrast/motion profiles, supported DPI/monitor transitions, geometry tolerance and latency/CPU/memory/handle/run limits. Stage 5B's display and fixture examples are proposals, not ratified values. Broader coverage increases host, operator and regression effort; no support commitment or budget is made here. Approval defines test coverage and limits only; it does not authorize configuration changes, purchases, dependencies, implementation or deem unrun cases passed.

### B04 — Preserve the searchable-dashboard acceptance contract

Trace: B04 / O04/O06. This is not approval of the older noneditable catalog proposal.

- **APPROVE:** retain Stage 5B's full searchable-dashboard contract and all applicable gates, with an explicit evidence-backed TextPattern2/GetCaretRange disposition still required.
- **REJECT:** state the product/contract objection and request a separately reviewed revision. Existing requirements remain in force; this packet changes none.
- **DEFER:** leave ratification unresolved while keeping the existing gates and accessibility/release block.

**Prerequisites and cost:** agreement on actual control inventory and evidence needed to close each gate. Coherent text/ranges, labels, focus, list/grid actions and scrolling, announcements, metrics/history, platform/lifecycle and human qualification require work beyond a basic bridge. Historical estimates are not an approved quote. Approval preserves acceptance scope only; it neither waives missing patterns nor authorizes a reduced no-search product, experiment, production integration or release.

### B05 — Exact, bounded experiment authorization

Trace: B05 / O05.

- **APPROVE:** only an exact future candidate-specific qualification proposal after source/API preflight passes, with owner-authorized scope, cost, acceptance criteria and stop/rollback rules.
- **REJECT:** decline the proposed experiment; preserve the baseline and report the decision without running it.
- **DEFER:** leave experimentation unapproved until technical, staffing and budget prerequisites are available.

**Prerequisites and cost:** candidate versions/revisions/hashes; supported APIs proving real editor/layout/composition authority and required provider methods; provenance/license/MSRV/compatibility audit; B02/B03 personnel/profiles; allowed paths, fixtures, finite total effort/run budget and stop date. No adequate candidate or approved cost is supplied by this packet. A blanket approval with these fields absent is non-executable. A project adapter entails ongoing lifecycle, action-queue, identity and compatibility maintenance.

An exact later approval permits only its isolated qualification phase, using nonpersonal fixtures and the agreed gates. It does not permit production dependency changes, installed-application enumeration, user pin writes, real launches, a fork/custom input, GUI replacement, shell work or release unless separately and expressly scoped. Stage 5B's proposed new experiment directory and fixture bounds are not permission to create or run them. Candidate success produces evidence for another decision, not automatic adoption.

### B06 — Upstream remediation and adoption policy

Trace: B06 / O06.

- **APPROVE:** require supported released widget/provider remedies and exact artifact review before considering adoption; approve a concrete later change scope separately.
- **REJECT:** request a separately bounded alternative policy/proposal, explicitly identifying any patch, fork, project-owned input or native boundary under consideration.
- **DEFER:** keep remediation/adoption unresolved and the current source/dependency baseline intact.

**Prerequisites and cost:** source evidence for coherent text/selection/geometry/composition and Windows label, container-scroll and text-provider contracts; exact dependency graph, notices, compiler and compatibility review. A newer version, open PR, Name string, ScrollItem or IME anchor is not sufficient. Upstream waiting is uncertain; patches/forks/custom editing create additional permanent maintenance and requalification costs. Policy approval alone authorizes no upgrade, download/install, patch/fork, copied widget, provider override or production adoption. No candidate is invented or selected here.

### B07 — Conditional native Windows fallback

Trace: B07 / O07.

- **APPROVE:** establish criteria for requesting a separate bounded native-control investigation decision if waiting cost is explicitly rejected or an adequate iced/provider candidate fails agreed gates.
- **REJECT:** decline that fallback policy; retain the blocked release without substituting a framework or weaker gates.
- **DEFER:** leave fallback reconsideration unresolved; no automatic alternative follows from a missed checkpoint or failed candidate.

**Prerequisites and cost:** owner-specified trigger/evidence, proposed research scope, maintenance ownership, cost cap and new decision checkpoint. Visible native controls may shift editing work to Windows but introduce host/focus/IME/tree/DPI/styling integration and migration regression costs; compatibility is unqualified. Policy approval permits preparation of a separate approval request only. It authorizes no native-control prototype, mixed host, new GUI framework, migration, invisible duplicate editor or custom COM provider. Research execution, any prototype and production architecture change each require their own explicit scope approval.

## Authorization boundaries and release gate

| Owner response concerns | Maximum effect of a complete attributable response | Still separately required |
|---|---|---|
| B01/B02/B03/B04/B06 policy, roles and contract | Record the exact approved/rejected/deferred decision; constraints remain binding. | Technical prerequisites, resources and candidate-specific execution approval. No automatic scheduled or implementation work. |
| B05 exact qualification proposal | After preflight and all named prerequisites pass, only the explicitly approved isolated phase may be considered executable. | A separate authorized task to execute it; no assumption from this documentation request. Successful qualification does not approve product integration. |
| B07 fallback trigger | Prepare a concrete request for a separate fallback decision. | Explicit research/prototype/migration approvals; nothing automatically switches architecture. |
| Accessibility or release acceptance | No B01–B07 policy response supplies it. | Applicable Stage 5B semantic G01–G11, platform/lifecycle G12–G13 and human G05/G14–G15 gates on candidate and actual product, independent review and explicit acceptance decision. G16 is currently N/A and reopens if sliders/menus appear. |

The scoped synthetic visible-focus PASS does not substitute for physical or semantic focus qualification. Historical isolated bridge UIA and human testimony cannot be counted as current production passes. Owner agreement cannot supply an absent API or turn NOT RUN into PASS. Unresolved mandatory conditions keep production accessibility and release acceptance blocked. No existing acceptance criterion is weakened here.

Deferral avoids premature architectural commitments but leaves the observed dashboard inaccessible through client semantics and postpones qualification. More UI work can increase later mapping/regression cost. Keeping the Stage 5A reliability fixes does not close those gaps. No budget, staffing, release date or support promise is inferred from either approval language or deferral.

## Blank owner response record

Copy this block for each B01–B07 response. It reuses [Stage 2M's fields](evidence-stage2m/owner-input-template.json) and the [Stage 5B template](evidence-stage5b/owner-input-template.json), adding an explicitly blank signature/equivalent attestation field. All value cells below are intentionally empty; no decision has been imported or signed.

| Owner-supplied field | Value |
|---|---|
| Decision ID | |
| Decision: APPROVE / REJECT / DEFER | |
| Rationale | |
| Exact scope and constraints | |
| Authorized person/role (approver) | |
| Authority/delegation reference | |
| Signature or equivalent attributable attestation reference | |
| Approval/response date | |
| Required evidence and prerequisites | |
| Candidate, allowed paths/phases and cost/run limits, if applicable | |
| Review/checkpoint date and stop conditions, if applicable | |

Record the owner's original wording and authority reference in a separately authorized follow-up. Check completeness and conflicts across B01–B07; ambiguous blanket approval is not an executable scope. A response can resolve its question as REJECT/DEFER without granting any implementation authority. Retain all historical unresolved records as history; do not overwrite them or trigger actions automatically when recording a response. No upstream/third-party message is sent on the owner's behalf.

## Baseline, preservation and validation

Starting/final branch: `main`. Starting/final HEAD: `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`. Entry status retained five Stage 5A tracked modifications (`README.md`, preferences implementation/tests, `main.rs`, `Test-Stage2A.ps1`) and the untracked Stage 5A/5B reports/evidence. No reset, clean, restore, stash or overwrite occurred.

Read Stage 5A, Stage 5B, Stage 2I/2K/2M and the existing K decision record/M and 5B blank templates. Source facts above are attributed to those records, not refreshed upstream or runtime claims. Before adding this packet, SHA256 inventory captured **723 existing tracked/nonignored untracked files**, including the existing reports/evidence. Final comparison confirms all 723 unchanged, including the five Stage 5A modified-file hashes and all **nine Cargo manifests/lockfiles**. Existing decision/template inputs remain byte-identical, with blank owner fields intact.

`git diff --check` passed for the working tree. Because this packet is untracked, its whitespace and local Markdown links were checked separately and passed. B01–B07 are each present once as decision sections; each offers APPROVE/REJECT/DEFER, and the blank response record contains no filled owner/authority/signature/date values. This is documentation/integrity validation only: no build/test, GUI, UIA, Narrator, physical-keyboard or real IME test was run in Stage 5C.

**Exact Stage 5C repository file change:** added `docs/implementation/STAGE-5C-OWNER-DECISION-PACKET.md` only. No existing repository file changed in this stage. An ignored `target/stage5c/` baseline/check receipt is local validation data, not a tracked deliverable. No source, manifest, `Cargo.lock`, UI behavior, accessibility architecture, acceptance gate, design, shell integration, settings or existing evidence changed. No provider experiment, GUI switch, automation/reminder, commit or push occurred.

**READY FOR OWNER RESPONSE**
