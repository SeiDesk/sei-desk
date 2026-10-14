# Stage 2B evidence

14 October 2026. This directory contains new baseline/research evidence, not an implemented AccessKit bridge. The [report](../STAGE-2B-ACCESSIBILITY-REPORT.md) and [matrix/procedures](../STAGE-2B-TEST-MATRIX.md) keep the accessibility gate blocked.

| Artifact | Interpretation |
| --- | --- |
| [Pre-existing file hashes](pre-existing-files.json) | Pre-edit SHA256s of 120 tracked design/reference, Stage 1/2A, code, lockfile and tooling files |
| [Preservation result](preservation.json) | All 120 checked files unchanged; baseline branch/HEAD and zero remaining test processes |
| [Local source review](local-source-review.txt) | Exact cached released source hashes/excerpts, host/toolchain and baseline revision; not AccessKit execution |
| [Resolved dependencies](dependencies.txt) | Unchanged normal application graph; no AccessKit dependency |
| [Quality gates](quality-gates.txt) | Initial sandbox formatting denial and successful filesystem-exception retry; check/tests/Clippy/release build |
| [Normal regression](normal/run.json) | Correct actual HWND and native Window-only UIA raw/control tree; process/resources/lifecycle |
| [Error regression](error/run.json) | Seeded actual error; absence of semantic error/Dismiss remains a failure |
| [Keyboard regression](keyboard/run.json) | Fixed developer input probe; separate from physical input/Narrator |

Each run retains its own raw/control trees, client image and process traces. The existing Stage 2A script is reused without editing it or overwriting old output. UIA counts of zero are failures of client accessibility even if the lifecycle script exits 0. No screen-reader speech or physical-interaction success is claimed. Resource samples measure the unchanged app, not a new provider's overhead. New stdout/stderr captures may include the script's terminal newline; this does not represent an additional application event.
