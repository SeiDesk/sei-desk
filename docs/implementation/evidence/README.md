# Stage 1 local evidence

These are real local command outputs from 14 October 2026. They are narrow observations on a restricted Windows host, not qualification results for every supported release. Interpret through the [report](../STAGE-1-WINDOWS-INTEGRATION-REPORT.md) and [matrix](../STAGE-1-TEST-MATRIX.md). Standard output and error were merged for most probes; their lines may interleave and are not a strict execution-order trace.

| Artifact | Meaning |
| --- | --- |
| [environment.txt](environment.txt) | Observed OS/toolchain/architecture and clean starting Git commit |
| [snapshot.txt](snapshot.txt) | Read-only Windows query results |
| [window.txt](window.txt) | Own fixture minimize/restore, unchanged foreground and normal cleanup |
| [observe.txt](observe.txt) | 30-second event-only native process measurement |
| [poll.txt](poll.txt) | 30-second nominal 50 ms cursor polling measurement |
| [raw.txt](raw.txt) | Post-fix requested 30-second raw-mouse registration/measurement/cleanup; binary SHA256 and independent parent elapsed time |
| [raw-pre-fix-lifetime.txt](raw-pre-fix-lifetime.txt) | Failed lifetime requirement: requested 30 s, observed 60.009 s; successful cleanup does not turn this into a pass |
| [lifetime-regression.txt](lifetime-regression.txt) | Rebuilt 1 s/2 s observer/raw/poll and close/panic regression, with independent parent elapsed time |
| [appbar.txt](appbar.txt) | Safety admission refused; **no reservation executed** |
| [hotkey.txt](hotkey.txt) | RegisterHotKey error 5, no registered key or suppression |
| [tray.txt](tray.txt) | NIM_ADD false; no successfully registered icon |
| [close.txt](close.txt) | Self-posted WM_CLOSE → WM_QUIT and cleanup |
| [panic.txt](panic.txt) | Expected panic/101 and successful unwind cleanup |
| [forced-observer.txt](forced-observer.txt) | Parent stopped/waited only its observer child; subsequent snapshot showed no spike HWND |
| [forced-observer-stdout.txt](forced-observer-stdout.txt) | Child setup before forced termination |
| [forced-observer-stderr.txt](forced-observer-stderr.txt) | Child stderr; intentionally no destructor cleanup after forced termination |
| [source-audit.txt](source-audit.txt) | iced baseline/source audit scope and native-handle evidence |
| [quality-gates.txt](quality-gates.txt) | Final formatting, lint, build/test command outcomes |
| [final-state.txt](final-state.txt) | Final read-only native snapshot, no spike HWND found and no remaining probe process |

Observed geometry changed from 1920 × 1080 to 1440 × 1080 between runs; no controlled hotplug/DPI test is inferred. Initial event/poll/lifecycle samples preceded a raw-input header read and a deadline-timer fix. The failed duration run is retained alongside rebuilt regression runs; the final raw/regression headers identify the rebuilt binary. Source revisions and chronology are disclosed rather than presenting all samples as one controlled benchmark. No input injection, keystroke capture, app-title logging, Explorer restart or persistent system change was used.
