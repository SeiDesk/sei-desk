# Stage 2J review-policy and feasibility evidence

2026-10-17. [Upstream policy](../STAGE-2J-UPSTREAM-REVIEW-POLICY.md), including the current-baseline API capability matrix; [non-editable feasibility](../STAGE-2J-NONEDITABLE-UI-FEASIBILITY.md). **CONTINUE — bounded upstream policy defined; non-editable UI remains separately gated.** Production accessibility remains BLOCKED.

- `baseline.json`: actual HEAD/main/status and OS; previous uncommitted G/H/I reports/evidence/observer are preserved.
- `preservation-before.json`: existing tracked/nonignored untracked file hashes before J artifacts, including historical failures.
- `initial-network-check.json`: sandbox DNS failures, retained separately from successful retrieval.
- `review-00-metadata.json`: fresh official registry metadata for five packages and GitHub issue/PR statuses, each with URL/time/result/version or revision and hash of received UTF-8 response text. Stores selected fields, not full raw responses; response hashes are identity records, not replayable raw captures.
- `release-check.json`: official iced 0.14.0/Windows-provider 0.35.1 release metadata, draft/prerelease flags, dates, URLs, response/release-note text hashes. Released notes are context, not API/runtime proof.
- `archive-registry-check.json`: five cached published archives match fresh registry checksums, no installation/resolution.
- `source-recheck.json`: 11 audited published files still match F/I source hashes. F's original archive-to-source audit is historical, not relabeled as fresh.
- `additional-source-identities.json`, `additional-archive-source-check.json`: consumer node/provider node/changelog provenance, three exact in-memory archive member comparisons; no registry source edited/extracted.
- `list-source-excerpts.txt`: actual line-numbered released selection/action/ScrollItem predicates and consumer container logic; no compiled or runtime list claim.
- `scroll-provider-search.json`: all 12 Windows Rust source files searched; no container ScrollPattern tokens found, while ScrollItem exists. Source finding, not fresh UIA query.
- `research-notes.md`: official web-source interpretations and failed retrieval boundaries; no external communication or branch adoption.
- `checksums.json`: SHA256 of stored review evidence and new reports, excluding itself and the final check artifact to avoid circular hashes.
- `final-checks.json`: preservation, checksum, JSON, local-document-link and whitespace checks, tracked diff result and final Git status. No build or runtime gate claimed.

Review-00 dates are recorded actual UTC observation timestamps, with policy dates interpreted in Asia/Calcutta. Future monthly dates/horizon are recommendations requiring owner ratification, not automations or an upstream promise. No reminder, scheduled task, polling process or repository scheduling change was made.

C–I evidence is linked rather than duplicated as new results. Historical real UIA, synthetic driving, source inspection and human attestation retain their separate scopes. H's absent session is not an IME test/failure; no J IME, physical keyboard, Narrator, UIA, GUI or build ran.

All prior source, designs, manifests, locks and historical evidence are unchanged. No commit or push.
