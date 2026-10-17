# Stage 3B independent acceptance evidence

New executions on 2026-10-15. Verdict: **FAIL**, due to the retained worker-completion regression. See [acceptance report](../STAGE-3B-ACCEPTANCE-REPORT.md).

- `preservation-before.json`: pre-review hashes, including uncommitted Stage 3A/3B work and prior evidence.
- `initial-working-tree-run/`: fresh execution of `tools/Test-Stage3B.ps1` against the original review input, before adding review regressions. All original gates and fixtures passed. Includes source/toolchain logs and release probe hashes and resource samples.
- `acceptance-regressions.txt`: new deterministic review tests; three pass, one fails. Controlled Service implementations, not native Windows behavior. The failing test proves a terminated worker did not publish completion; it avoids hanging the public blocking wait.
- `current-tree-quality-gates.txt` and `gate-outcomes.json`: current-tree gates after adding the regressions. Compilation/Clippy/release and actual Windows fixtures pass; library/workspace tests fail. These failures are intentional retained evidence, not a successful acceptance result.
- `final-review.json`: preservation comparison, whitespace/link checks, and remaining owned-probe process check.

Original evidence directories were not overwritten. The Windows fixture uses an injected catalog with real owned process creation; native discovery evidence comes from the separate release probes. No physical keyboard, Narrator or UI Automation test was performed for this headless review. Resource sampling includes refresh work and is not a pure idle benchmark. No recorded failure has been replaced with a mock pass.
