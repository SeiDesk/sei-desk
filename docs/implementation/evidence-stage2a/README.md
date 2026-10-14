# Stage 2A evidence

Actual local observations, 14 October 2026, Windows NT 26300 x64. Read the [accessibility report](../STAGE-2A-ACCESSIBILITY-REPORT.md) and [architecture](../STAGE-2A-ARCHITECTURE.md). No result here certifies Windows 10/11 release builds or screen-reader conformance.

| Artifact/run | Evidence and interpretation |
| --- | --- |
| [environment.txt](environment.txt) | Toolchain/host plus a read-only Stage 1 native snapshot in this session; Stage 1 evidence files are not modified |
| [dependency-scope.txt](dependency-scope.txt) | Resolved Windows normal dependency subset; software renderer, no AccessKit integration |
| [quality-gates.txt](quality-gates.txt) | Actual formatting/lint/check/unit/release build outputs |
| [preservation.json](preservation.json) | 29 Stage 1 files unchanged; branch/HEAD and no remaining test processes |
| [Run 01](run-01/run.json) | Invalid client target: Process.MainWindowHandle picked winit's internal event window; no client accessibility/key success may be inferred |
| [Run 02](run-02/run.json) | Failed diagnostic-read fixture; exact child stopped before testing. Not a passing lifecycle test |
| [Run 03](run-03/run.json) | Correct iced HWND and Window-only UIA tree; WScript key requests produced no transitions. Old full-frame pixel capture is not final client evidence |
| [Run 04](run-04/run.json) | SendInput returned 0; early native-stderr fixture failure. App closed normally; resource baseline not collected |
| [Run 05](run-05/run.json) | Correct target/iced HWND confirmation, input Blocked, 30 s baseline, exit 0/no owned child |
| [Run 05 raw/control trees](run-05/uia-control.json) | Correct native Window, no client descendants; raw view saved alongside control view |
| [Run 05 keyboard probe](run-05/keyboard-probe.txt) | `send_input_count=0 expected=2`; no successful Tab delivered; cause not established |
| [Run 06 error fixture](run-06-error/run.json) | Explicit seeded error, real error pixels, missing UIA client tree, 30 s sample and exit 0 |
| [Run 06 UIA](run-06-error/uia-control.json) | No error text, Dismiss action or other client controls exposed |
| [Run 06 client image](run-06-error/window.png) | Own client-only pixels; visible Validate focus and inline error, not a design mockup or keyboard-delivery proof |
| [Run 07 final normal build](run-07-final/run.json) | Final application baseline/target/cleanup results; screenshot and raw/control trees alongside JSON |

Raw app output is retained in each run directory. Diagnostics omit typed values and foreign window identities; the own validation HWND is logged only for target verification. UIA names belong only to the identified test process. Final pixel capture uses the guarded client rectangle, without desktop shadows/neighboring app content. Early invalid captures are retained with explicit invalid status; do not use their filenames as proof of a rendered error.

An exit-0 developer script means its lifecycle/measurement procedure completed, not that client accessibility passed. UIA counts are zero; Narrator speech/physical keyboard/IME checks remain Not run. Stage 1's original evidence directory is separate and preserved.
