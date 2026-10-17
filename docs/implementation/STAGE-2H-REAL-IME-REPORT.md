# Stage 2H — Real OS IME validation

2026-10-17. **UNVERIFIED — no suitable real-IME session was available; no configuration changes made.** Production accessibility remains **BLOCKED**. Runtime testing stopped at readiness, as required.

## Readiness and identities

Read [Stage 2G](STAGE-2G-IME-OBSERVATION-REPORT.md), its [evidence index](evidence-stage2g/README.md), and the unchanged observer instructions and runner. HEAD is `d14d4bede2096d4bea2561de22977d181376b640`, branch `main`. Stage 2G's report, evidence and observer are uncommitted additions; they were preserved.

Machine inspection reports Windows NT 10.0.26300.0, X64. The operator answered the readiness question: **“No suitable IME or operator is available.”** No language or input-method identity was confirmed. This does not establish that no IME is installed; it establishes that the required human-operated session is unavailable. No installation, language switch, settings change or synthetic composition was attempted.

The existing binary and source hashes match Stage 2G's final run:

| Artifact | SHA256 |
| --- | --- |
| src/main.rs | `65C0F18F9933AB15009FD47F53EB6C93021721313688E1754AFEC9CAF6EB492E` |
| src/observe.rs | `3863B3625D990AA4221F21BC5B604A335B7A9C317E1C040CF016C4E2A4A384EB` |
| target/iced-ime-observer.exe | `FBFD9C5F93767E2C45A886A553DF0DC243FF2FABD4675167499B04C7F67B1481` |

Full observer/script identities and machine capture time are in [readiness-machine.json](evidence-stage2h/readiness-machine.json). Operator testimony is recorded separately in [operator-readiness.json](evidence-stage2h/operator-readiness.json). The machine capture predates the answer and correctly retains its original pending status.

## Per-case results

| Case | Result | Reason / evidence |
| --- | --- | --- |
| 1. Begin composition in empty input | UNVERIFIED | Required operator/IME session unavailable. |
| 2. Repeated pre-commit updates | UNVERIFIED | No composition session attempted. |
| 3. Commit and final document value | UNVERIFIED | No composition session attempted. |
| 4. Cancel composition | UNVERIFIED | No composition session attempted. |
| 5. Compose at beginning/middle/end | UNVERIFIED | No composition session attempted. |
| 6. Compose with selection | UNVERIFIED | Input-method capability and behavior not tested. |
| 7. Resize during/after composition | UNVERIFIED | Stage 2G resize results do not establish composition behavior. |
| 8. Move focus between inputs during/after composition | UNVERIFIED | Stage 2G noncomposition focus results do not establish composition behavior. |
| 9. Hide/restore/close at composition safe points | UNVERIFIED | No Stage 2H process or composition session started. |

There are no Stage 2H raw runtime logs, sample references, screenshots, composition timestamps or begin/update/commit/cancel observations. No physical-keyboard, Narrator or UI Automation tests were executed. No build/test suite was rerun: this stop-path changes documentation only. Checks performed are hash preservation, JSON parsing, local documentation links and `git diff --check`; outcomes are recorded in [final-checks.json](evidence-stage2h/final-checks.json).

## Cleanup, comparison and next step

No observer process was launched in this stage, so there is no Stage 2H PID or process-exit code to report. Final process inspection and preservation results are recorded separately; no owned process needed termination. No runtime source, accepted spike, dependency, lockfile, service, design or historical evidence changed. No commit or push.

Stage 2G remains unchanged: it observed stock-widget logical state and synthetic noncomposition interactions, with inherited shared-Shell output, endpoint limitations and no coherent native layout/composition generation. Its null preedit samples do not prove an IME was absent. An IME rectangle is not proof of the active caret rectangle; model epochs are not native generations; update callbacks are not paint acknowledgements. Stage 2H provides no new runtime evidence and establishes no coherent snapshot guarantee.

The next safe step is to arrange a human operator and confirm an already available composition-capable IME and its language before launching the unchanged observer into a fresh evidence directory. Any installation or configuration change requires separate explicit approval. Only cases actually captured can subsequently pass. No TextPattern/TextRange, production UI or shell work is authorized; the upstream-first strategy and production accessibility verdict remain unchanged.
