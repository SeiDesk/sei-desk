# Stage 2I research and evidence integrity

2026-10-17. [Architecture decision](../STAGE-2I-ACCESSIBILITY-ARCHITECTURE-DECISION.md): **REASSESS**. Production accessibility remains BLOCKED; headless acceptance is unchanged.

- `baseline.json`: actual HEAD/main/OS and pre-task working-tree status. G/H additions are existing work, not Stage 2I implementation.
- `preservation-before.json`: hashes of every existing tracked and nonignored untracked file before I artifacts. Includes prior reports, experiment source and historical failures/evidence.
- `locked-stack.json`: exact relevant package versions/checksums read from the accepted standalone spike lock. Transitive Windows binding versions are separate entries, not mistaken for a direct upgrade.
- `registry-check.json`: initial sandbox DNS failures for five official read-only API requests. No successful version claim is based on these failures.
- `registry-recheck.json`: fresh successful official crates.io read-only responses, using a network sandbox exception; no OS elevation, package installation or dependency resolution. Includes URLs, capture times, newest/stable versions, publication times, checksums and yanked fields.
- `archive-registry-check.json`: 5/5 existing cached archives match those fresh official published checksums. No extraction/install performed in I.
- `source-recheck.json`: 11/11 inspected source files remain byte-identical to F's audit. F's original archive-to-source verification remains historical; not represented as rerun here.
- `published-identities.json`: actual package VCS paths/revisions, license and declared Rust versions. Empty per-package RustVersion means no declaration found there, not an invented minimum-version pass.
- `source-excerpts.txt`: line-numbered excerpts from exact cached published widget/Shell/provider source. Constitutes source inspection, not runtime text/IME support.
- `historical-evidence-audit.json`: fresh integrity/count inspection of five historical C/CA/D/E UIA captures, each 29/29. No runtime rerun; physical-input/Narrator harness metadata remains separate from D's human attestation.
- `research-notes.md`: official web sources and failure/interpretation boundaries, including the native-control alternative hypothesis.
- `final-checks.json`: baseline preservation, JSON and local-link/whitespace checks, protected manifest/source identities, `git diff --check` outcome and final Git status. Review-only, no Cargo quality gates or runtime tests claimed.

Historical evidence is linked in the decision instead of duplicating large logs. D human attestation is preserved separately from UIA success and G synthetic observations. H provides operator unavailability, not an IME failure or proof that no IME is installed. This task contains no fresh screenshots, events, physical keys, Narrator speech, UIA patterns, IME sessions or performance results.

No existing source/design/dependency/lock/evidence file changed, no production UI or shell work, no commit or push.
