# Stage 2C evidence index

14 October 2026. Real Windows UIA client runs against a standalone iced + released AccessKit prototype. [Report](../STAGE-2C-ACCESSKIT-REPORT.md), [matrix](../STAGE-2C-TEST-MATRIX.md), [reproduction](../../../experiments/accesskit-iced-spike/README.md).

| Artifact | Interpretation |
| --- | --- |
| [Pre-edit hashes](preservation-before.json) | 147 existing tracked/untracked files, including uncommitted Stage 2B artifacts, before edits |
| [Preservation result](preservation-after.json) | All 147 files unchanged; branch/HEAD recorded, no experiment process remains |
| [API review](api-review.txt) | Exact released source hashes, host/toolchain, license/API decisions and exception-code interpretation |
| [Dependencies](dependencies.txt) | Actual unique Windows normal package/version graph, 114 entries |
| [Quality gates](quality-gates.txt) | Initial Clippy failure retained; corrected final formatting/check/10 tests/strict Clippy/release plus root 11 tests |
| [Run 01](run-01/run.json) | Initial client selected native titlebar Close; not a passing complete run; initial Label names also needed correction |
| [Run 02](run-02/run.json) | Values/error worked; wrong Input-focus preservation assertion failed after an explicit UIA Focus-to-Validate request |
| [Run 03](run-03/run.json) | Core operations passed; stale GetCurrentPattern checks are weaker than final retained-pattern checks |
| [Run 04](run-04/run.json) | UIA LabeledBy property unavailable; preserved failed compatibility assertion; no native workaround added |
| [Run 05](run-05/run.json) | Core actions/move/resize/normal cleanup passed; cached-pattern exceptions logged only as messages |
| [Run 06](run-06/run.json) | Captured full stale-exception chain: ElementNotAvailableException, 0x80040201 despite misleading token-error message |
| [Final run](run-07-final/run.json) | 29 actual UIA checks passed, including exact stale-error assertions; overall accessibility gate explicitly not passed |
| [Final error tree](run-07-final/error-control.json) | Named Text + Dismiss/Invoke and Validate focus, with real bounds; raw tree saved alongside |
| [Run 06 value pixels](run-06/value.png) | Same final binary, real input displays UIA-set sample; own client, no generated mockup |
| [Final resized tree](run-07-final/resized-control.json) | Real layout reacts to native Transform resize; final run captures were blocked because the HWND was not foreground |

Each run has its own stdout/stderr, raw/control trees and client captures where reached. No old run is overwritten or relabeled. Diagnostics omit user input strings and foreign windows; own HWND, action kinds/IDs and model flags support test correlation. Screenshots capture only the owned foreground client. No Narrator speech, physical key delivery, TextPattern or live announcement is claimed. Root application and earlier evidence remain separate.
