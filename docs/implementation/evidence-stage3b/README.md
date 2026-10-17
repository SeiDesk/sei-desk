# Stage 3B evidence

New 15 October 2026 executions. Verdict/scope: [architecture](../STAGE-3B-CONSUMER-ARCHITECTURE.md); procedures and test boundaries: [matrix](../STAGE-3B-TEST-MATRIX.md).

- [consumer-initial.txt](consumer-initial.txt): first 17 controlled consumer tests passed.
- [clippy-initial.txt](clippy-initial.txt): strict Clippy initially rejected nested-if and unnecessary-lazy-evaluation style. Corrected without suppressions; final full Clippy passed.
- [consumer-tests.txt](consumer-tests.txt): final 20 consumer state/concurrency tests, controlled service/channel gates, no native accessibility claims.
- [windows-run/suite/quality-gates.txt](windows-run/suite/quality-gates.txt): full workspace formatting, all-target check, ordinary tests, strict all-target Clippy, release builds, diff check. 45 ordinary tests passed; opt-in fixtures enabled separately.
- [windows-run/suite/windows-tests.txt](windows-run/suite/windows-tests.txt): all 39 sei-apps tests passed, including the new real Windows consumer fixture test and preserved Stage 3A acceptance cases. Fake services and mappings are not presented as real OS evidence.
- [windows-run/suite/catalog-stdout.txt](windows-run/suite/catalog-stdout.txt), [resource-baseline.json](windows-run/suite/resource-baseline.json): freshly rerun Stage 3A service-only baseline for the comparison.
- [windows-run/consumer-stdout.txt](windows-run/consumer-stdout.txt), [consumer-resource-baseline.json](windows-run/consumer-resource-baseline.json): actual NativeService consumer refresh, 600-byte inline size, deterministic pair, normal disposal and whole-process counters. Neither read-only probe selects/launches installed apps.
- [preservation-before.json](preservation-before.json), [preservation-after.json](preservation-after.json): includes previously uncommitted Stage 3A acceptance report, test edits and evidence. Only lib.rs changes among pre-existing files; prior native/test edits and all historical artifacts remain intact.
- [final-review.txt](final-review.txt): final links/whitespace, process, dependency/unsafe and Git scope checks.
- [post-review-gates.txt](post-review-gates.txt): final formatting/check/strict Clippy confirmation after source review; no functional changes followed the full Windows run.

Each run directory preserves actual stdout/stderr and executable hashes. Current test/probe results were executed on Windows, not copied from earlier captures. Fresh text-log terminal empty lines are normalized for packaging, with results unchanged. No root manifest/lockfile, design, production GUI or AccessKit-spike file changes. No commit or push is performed by this stage.
