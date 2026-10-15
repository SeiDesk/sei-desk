# Stage 2C acceptance evidence — 15 October 2026

[Acceptance report](../STAGE-2C-ACCEPTANCE-REPORT.md). Verdict **BLOCKED** for production adoption despite newly passing UIA/synthetic keyboard checks. This is a new review of unchanged revision `5b40855`, not a rewrite of Stage 2C evidence.

| Artifact | Evidence class and scope |
| --- | --- |
| [quality-gates.txt](quality-gates.txt) | New formatting/check/10 tests/strict Clippy/release outputs and exact executable SHA256; build evidence, not accessibility conformance |
| [uia-01/run.json](uia-01/run.json) | New cross-process Windows UIA run: 29 checks pass, exit 0, pre-show attach/detach verified, resource sample, stale HRESULTs; screenshot captures refused when not foreground |
| [Initial raw tree](uia-01/initial-raw.json) / [control tree](uia-01/initial-control.json) | Actual correct-PID/HWND client names/roles/patterns/bounds |
| [Error raw tree](uia-01/error-raw.json) / [control tree](uia-01/error-control.json) | Actual error insertion/Dismiss and retained Validate focus |
| [Final raw tree](uia-01/final-raw.json) / [control tree](uia-01/final-control.json) | Error/Dismiss removed after action; other controls retained |
| [Runtime trace](uia-01/stderr.txt) | Real provider action/model transitions and detach; no typed values or foreign window identities |
| [Synthetic keyboard log](synthetic-keyboard.json) | Separate @oai/sky input/observations: typing, edit-in-middle, traversal, validation, visible checkbox/focus, removal and asynchronous Escape close; **not physical input or Narrator** |
| [Pre-review hashes](preservation-before.json) | SHA256 baseline for all 234 existing tracked files before this acceptance task |
| [Preservation/cleanup](preservation-after.json) | Final comparison and no remaining spike process |

No prior-run capture or mock is presented as a new test. The synthetic window screenshot was observed through the Computer Use runtime in this session; its pixel payload is not duplicated in this directory. No human physical-keyboard or audible Narrator evidence was available. Narrator installation/version was checked read-only; Narrator was not launched. See the report's separate blocked-test table.
