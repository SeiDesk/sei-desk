# Stage 5E — Upstream-First Owner Architecture Decision

Recorded 2026-10-17, Asia/Calcutta. **Owner architecture decision recorded; implementation and qualification are not authorized.**

## Decision source and scope

Authority for this record is the owner's direct instruction in this chat, titled **“Stage 5E — Record Upstream-First Architecture Decision”**, specifically its seven-item “Owner decision to record” section. This is an explicit architecture decision, rather than another recommendation. No personal approver identity, signature or separate approval timestamp is invented; the date above is the recording date.

This narrow new record preserves the [Stage 5B report](STAGE-5B-ACCESSIBILITY-ARCHITECTURE-DECISION.md), [Stage 5C packet](STAGE-5C-OWNER-DECISION-PACKET.md), [Stage 5D preflight](STAGE-5D-ACCESSIBILITY-CANDIDATE-PREFLIGHT.md) and historical [2K owner record](STAGE-2K-OWNER-DECISIONS.md)/[2M package](STAGE-2M-OWNER-DECISION-PACKAGE.md) without rewriting their then-unresolved status or evidence.

| Subject | Owner decision |
|---|---|
| UI architecture | **Retain Rust + iced** as Sei-Desk's UI architecture. |
| Accessibility strategy | **Upstream-first.** Prefer a supported, source-auditable upstream solution exposing real widget semantics and satisfying the required Windows UI Automation contracts. |
| Current candidate | **Reject Stage 5D C0 for qualification** because it fails source preflight. Do not integrate it into production. This is the exact pinned combination identified in Stage 5D, not a rejection of all possible future upstream solutions. |
| Production status | Accessibility remains **BLOCKED**. No production accessibility acceptance or release-readiness claim is permitted by this decision. Existing acceptance gates remain unchanged. |
| Experiments and dependencies | **No new provider prototype, dependency change, fork, download, installation or runtime experiment is authorized.** |
| Native Windows controls | Remain a possible **separately authorized fallback investigation** only. No automatic reconnaissance, prototype, migration, GUI replacement or shell replacement follows. |
| Checkpoint | Record the need for a finite upstream review checkpoint and a subsequent owner decision. Exact dates, review effort and execution remain unapproved as described below. |

## Rationale and qualification boundary

Stage 5D found independent missing boundaries in C0: authoritative coherent editor/layout data and required Windows provider behavior, including label relationships, container scrolling and text/range support. Component APIs and an older isolated bridge do not supply complete production semantics. Retaining Rust + iced and preferring a supported upstream solution does not turn that failed candidate into an eligible one or waive human/platform qualification.

A future candidate must first have exact source/API evidence satisfying the existing preflight contracts. Qualifying or implementing it then needs separate explicit owner approval of the exact candidate, scope, cost, acceptance criteria and stop/rollback conditions. Production adoption and accessibility/release acceptance remain later, separate decisions based on applicable semantic, platform and human-operator evidence. No work at those boundaries is started by recording this decision.

## Checkpoint status and unrelated decisions

| Checkpoint proposal from Stage 5C | Status after this decision |
|---|---|
| November 17, 2026 — finite upstream evidence review checkpoint | **PROPOSED ONLY; date not approved or committed.** |
| January 17, 2027 — subsequent owner decision checkpoint | **PROPOSED ONLY; date not approved or committed.** |

Dates use Asia/Calcutta. The owner has affirmed the need for finite review and a subsequent decision, but has not ratified these proposed dates, a cadence, reviewer effort, budget or scheduled task. No reminders, automation, monitoring or continuing review commitment is created. A missed proposed date does not extend waiting, authorize a candidate or activate the native-control fallback.

For Stage 5C traceability, this records the **architecture/upstream-first portion of B01**, while checkpoint details remain pending. It does **not** constitute blanket approval of B01 or approval of unrelated **B02–B07**: staffing, support configurations, contract ratification/API dispositions, experiment funding/scope, exact dependency adoption/remediation or fallback investigation triggers. Those historical records/templates remain unchanged. The express C0 rejection and non-authorizations above apply regardless of any future response to those questions.

No people, budgets, support commitments, technical replacement candidate or implementation deadlines are assigned. There is no permission to modify application code, dependencies, tests, scripts, UI behavior, existing gates or evidence, or to begin shell integration, global hooks, Explorer changes or privileged optimization.

## Repository integrity and validation

Repository: `X:\Project\SeiDesk\sei-desk`. Starting/final branch: `main`. Starting/final HEAD: `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`.

Before editing, complete working-tree status was inspected with `git status --short --untracked-files=all`. All **725 pre-existing tracked/nonignored untracked files**, including **49 Rust source files**, **nine Cargo manifests/lockfiles**, the Stage 5A fixes and Stage 5B/5C/5D documentation/evidence, remain unchanged. Preservation compares path + tab + uppercase file SHA256 records in `git ls-files --cached --others --exclude-standard` order, joined with LF without a final LF and UTF-8 hashed, excluding only this new record at completion. Entry/final aggregate SHA256:

`7AF7125F7ED05C6B13775C25D593CA7FD5D6C52A5B43FD2091AC3BB4DAF75591`

`git diff --check` passed. This untracked record's whitespace and local links were checked separately and passed. No build, test, upstream research, runtime, UIA, Narrator or IME session was run. No source, manifest, lockfile, dependency, test, script, existing report or evidence file was edited. No reset, clean, restore, stash, automation, experiment, commit or push occurred.

**Exact changed path:** added `docs/implementation/STAGE-5E-UPSTREAM-FIRST-OWNER-DECISION.md` only. Final short Git status:

```text
 M README.md
 M crates/sei-desk/src/applications/preferences.rs
 M crates/sei-desk/src/applications/preferences/tests.rs
 M crates/sei-desk/src/main.rs
 M tools/Test-Stage2A.ps1
?? docs/implementation/STAGE-5A-STABILIZATION-ACCEPTANCE.md
?? docs/implementation/STAGE-5B-ACCESSIBILITY-ARCHITECTURE-DECISION.md
?? docs/implementation/STAGE-5C-OWNER-DECISION-PACKET.md
?? docs/implementation/STAGE-5D-ACCESSIBILITY-CANDIDATE-PREFLIGHT.md
?? docs/implementation/STAGE-5E-UPSTREAM-FIRST-OWNER-DECISION.md
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

All entries except the new Stage 5E record predate this task. The architecture decision is recorded; production accessibility remains BLOCKED and no further implementation or qualification action is authorized.
