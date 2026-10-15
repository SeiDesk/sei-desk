# Stage 2D — Accessibility closure

Date: 2026-10-15. Verdict: **PASS for the isolated validation window, with an explicitly approved limited deferral.** This permits recommending a headless Stage 3 application catalog/launch service. It does not approve production UI, general accessibility conformance, or the production use of this experimental bridge. Stage 3 has not been started.

## Evidence and decision

The updated spike passed two fresh Windows UI Automation runs, each with all 29 checks. Physical keyboard and Windows Narrator acceptance was performed by the human user against the supplied checklist and reported as `PASS`, then confirmed as `EVERYTHING WORKS` when asked specifically about both input methods, announcements, editing, and focus. These are **user-attested results**, not assistant-observed speech or synthetic keyboard results. Exact spoken utterances, audio, and duplicate-announcement counts were not supplied; none are inferred here.

The input still does not implement UIA TextPattern, programmatic caret/selection queries, or text ranges. After this limitation was stated explicitly, the user responded **“Approve that limited deferral”** to deferring those APIs **only for a headless Stage 3 catalog/launch service**, keeping them as a gate before production launcher UI. Manual editing success does not establish these APIs. The scoped approval and manual checklist are preserved in [manual-attestation.json](evidence-stage2d/manual-attestation.json).

See the [test matrix](STAGE-2D-TEST-MATRIX.md) and [new evidence index](evidence-stage2d/README.md). Earlier Stage 2C/acceptance evidence remains historical and unchanged; it is not substituted for these runs.

## Smallest changes made

- The real inline error node uses released AccessKit `Live::Polite`. No control values, focus, or error text are duplicated in a second application model. Error creation preserves initiating focus; dismissal removes the error and its action and returns focus to the input.
- Asynchronous layout and input-focus queries carry a generation. Outdated query results cannot overwrite newer application state, and native publication rejects older generations. Closing rejects subsequent actions/results.
- Three focused regression tests cover stable error focus/unchanged semantic updates, stale layout/focus results, and late messages after closing. These are unit tests, not simulated proof of Windows accessibility.
- The existing UIA harness's static metadata now correctly states that it does not evaluate live announcements or manual acceptance. Its 29 checks were retained. The experiment README links this disposition while retaining Stage 2C history.

No dependencies, lockfiles, production crates, designs, shell integrations, hooks, tray hosting, or persistent settings were changed. Unsafe code remains confined to the existing native boundary; no unsafe block was added.

## Exact API and text-editing disposition

The unchanged experiment pins iced **0.14.0**, AccessKit **0.25.1**, accesskit_windows **0.35.1**, and windows-sys **0.61.2**. These are released dependencies; no fork or Git branch was introduced. Existing licensing and maintenance findings in the [Stage 2C report](STAGE-2C-ACCESSKIT-REPORT.md) still apply. Exact cached released source and compilation, rather than latest documentation alone, established API availability.

AccessKit 0.25.1 exposes `Node::set_live(Live::Polite)`. The 0.35.1 Windows adapter raises `UIA_LiveRegionChangedEventId` for a newly added named live node and relevant name/value/live changes. Its node provider maps Polite to the Windows live setting. This is supported-source evidence, not a measurement of emitted event counts. See [AccessKit Live documentation](https://docs.rs/accesskit/0.25.1/accesskit/enum.Live.html) and [Windows UIA properties](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-automation-element-propids).

Only the error is live. Unchanged validation state produces no semantic node delta (unit-tested), and a dismissed/recreated error has a new occurrence identity. This prevents the identified redundant publication path; it does not prove that every screen reader speaks exactly once. The available managed UIA client could not resolve the live-event/property IDs, so automated live-event observation was not completed. Narrator feedback and stable focus are user-attested against the repeated-error checklist.

| Capability | Evidence / disposition |
| --- | --- |
| Input name, editable value, focus; ValuePattern read/set | Fresh native UIA checks passed. Explicit name is exposed; provider LabeledBy relationship remains absent. |
| Visual caret, selection/replacement, editing in the middle | Human physical-keyboard checklist passed; no programmatic text-range claim. |
| Narrator identification, navigation, operation, state/error feedback | Human checklist passed; exact speech was not transcribed. |
| UIA caret/selection/TextPattern/text-range geometry | Not implemented. Released iced text-input operation boundary does not provide the actual selection/caret readback needed for a truthful adapter. No guessed text runs/bounds were added. Explicit limited deferral applies only to headless Stage 3. |
| Launcher readiness | Names, focus, keyboard editing and feedback are necessary, but this spike does not qualify a production launcher. Resolve text-range APIs and validate actual launcher editing before UI implementation approval. |
| IME/grapheme edge cases, high contrast, mixed DPI, other assistive tools | Not newly qualified here; retain as production UI qualification work. |

## Dispatch, ownership, lifecycle, and residual risks

Provider callbacks enqueue bounded actions; application state changes run on the iced update path. The existing bounded channel capacity is 32, and queue overflow remains a logged/dropped-action limitation. Semantic snapshots and native resources are owned by the existing native owner; provider events are raised outside its mutable borrow. No continuous accessibility polling was added. Layout/focus queries are asynchronous and now reject obsolete generations.

Stable IDs and occurrence-specific error IDs prevent a retained dismissed action from acting on a later error. Fresh native checks confirmed `UIA_E_ELEMENTNOTAVAILABLE` (-2147220991) for retained removed Dismiss and closed-window input objects, process exit code 0, provider detachment, and no remaining child process. Unit tests separately verify late application messages. These results do not establish exhaustive race, queue-overload, forced-termination, or heap-leak safety. The bridge remains project-owned experimental widget integration with future maintenance obligations; production sliders, menus, virtualized catalogs, DPI transitions and richer text support require separate qualification.

## Execution and resource baseline

Environment: interactive Windows desktop reported by the client as `Microsoft Windows NT 10.0.26300.0`; release build, Rust 1.99.0 toolchain. These observations do not qualify every supported Windows 10/11 build. Tested executable SHA256:

`70307E37BB8F06E11FA09EA3DF07E1D164840EDCA5C2BF12E14B8BE6D719D83F`

The separate PowerShell UIA process measured process CPU-time deltas and memory counters during a 30-second idle interval after interaction. Final run: 30.0111 seconds, CPU delta 0.125 seconds (0.4165% of one core), working set 21,753,856 bytes, private commit 5,361,664 bytes, peak working set 22,454,272 bytes. First run: 30.0140 seconds, CPU delta 0 at counter resolution, working set 21,942,272 bytes, private commit 5,693,440 bytes. Short samples vary; zero measured delta is not a zero-CPU guarantee. No long-duration resource or leak claim is made.

Formatting, compilation, **13 unit tests**, strict all-target Clippy, and release build passed. An initial Clippy test-module placement failure was corrected and the full sequence rerun successfully; both attempts remain in the log. Repository whitespace and preservation checks are indexed with evidence.

## Next milestone boundary

Recommend Stage 3 **headless catalog discovery and explicit launch service**, with independently testable discovery/filtering and launch/error handling, retaining narrow Windows integration boundaries. Do not start the production launcher, taskbar, dock, AppBar, global Windows-key interception or tray integration. Before launcher UI approval, close the programmatic text-range/caret/selected-text gap and qualify the real controls with Narrator, keyboard, high contrast, DPI and supported Windows versions. This approval does not waive earlier Windows shell feasibility risks.
