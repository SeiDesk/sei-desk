# AccessKit spike acceptance review

**Verdict: BLOCKED — do not proceed to production UI development.**

Review date: **15 October 2026** (Asia/Calcutta). Reviewed revision: `5b4085595227cd349404f755fef2fcab07fe8bb7`, branch `main`, initially clean. The exact unchanged spike builds and its core Windows UIA checks pass again. A separate synthetic keyboard sequence also succeeds. Required human physical-keyboard and actual Narrator speech/operation checks remain blocked; neither compilation nor automation substitutes for them. There is no evidence-based production accessibility sign-off.

Sources: [Stage 2C report](STAGE-2C-ACCESSKIT-REPORT.md), [test matrix/manual procedure](STAGE-2C-TEST-MATRIX.md), [prior evidence index](evidence-stage2c/README.md), [spike instructions](../../experiments/accesskit-iced-spike/README.md), and [new acceptance evidence](evidence-stage2c-acceptance/README.md). Historical runs remain untouched and are not counted as newly executed tests.

## Exact build and new runtime evidence

Windows NT `10.0.26300.0`, x64/MSVC; Rust/Cargo `1.99.0`. The standalone committed Cargo.lock is used with `--locked --offline`. iced `0.14.0`, iced_widget `0.14.2`, iced_winit `0.14.1`, winit `0.30.13`, accesskit `0.25.1`, accesskit_windows `0.35.1`, accesskit_consumer `0.39.1`, windows-sys `0.61.2`; tiny-skia software renderer. No upgrades or code changes.

The rebuilt release executable SHA256 is **`4F4265B39FBB47A16B280D622A62899E94C2B3EF9D99261DF8E793ECD9B15CE5`**, matching the previously tested binary. The new run has its own process, window, UTC timestamp, raw/control trees, actions, captures and shutdown evidence. Matching bytes establish identity; they do not make old captures new evidence.

Executed commands, from repository root:

```powershell
cargo fmt --manifest-path experiments/accesskit-iced-spike/Cargo.toml --check
cargo check --manifest-path experiments/accesskit-iced-spike/Cargo.toml --locked --offline
cargo test --manifest-path experiments/accesskit-iced-spike/Cargo.toml --locked --offline
cargo clippy --manifest-path experiments/accesskit-iced-spike/Cargo.toml --all-targets --locked --offline -- -D warnings
cargo build --manifest-path experiments/accesskit-iced-spike/Cargo.toml --release --locked --offline
powershell.exe -NoProfile -File experiments/accesskit-iced-spike/Test-UIA.ps1 -OutputDirectory docs/implementation/evidence-stage2c-acceptance/uia-01 -IdleSeconds 30
```

Every Cargo command passed; all **10 unit tests** passed. These are pure model/semantic policy tests, not OS accessibility proof. Build commands used a filesystem sandbox exception as required by local tooling; runtime UIA used ordinary session rights without RunAs/UAC. No mocked provider was used. For another reproduction, use a new/empty output directory rather than the recorded path.

### Actual cross-process Windows UIA run

The unchanged repository test client passed **29/29 checks** against one fresh owned process; its exact PID/HWND are recorded in [uia-01/run.json](evidence-stage2c-acceptance/uia-01/run.json), not reusable targets. Selection checks PID, title, native Window type, HWND ownership and the matching pre-show attach trace. Client AutomationIds distinguish real controls from native titlebar controls. This run's pixel captures were correctly refused because the HWND was not foreground at capture points; no historical screenshot is substituted. The later synthetic run supplied a separately observed fresh window capture.

| Requirement | Acceptance observation | Status |
| --- | --- | --- |
| Raw/control view discovery | Sample text Edit, Validate Button, Motion CheckBox, conditional error Text/Dismiss Button and Close Button exposed with expected names/IDs, enabled/focusable states and positive bounds | PASS for this run |
| ValuePattern | Initially editable/empty, actual SetValue to `sample`, read back, clear and read back | PASS for whole-string value updates |
| InvokePattern | Validate with nonempty and empty input, Dismiss error, and actual client Close operated the model/window | PASS |
| TogglePattern/state synchronization | Off→On→Off, UIA state and model diagnostics agree | PASS |
| Error insertion/removal | Error text and Dismiss appear after validation and disappear after dismissal in recorded raw/control views | PASS |
| Local focus | Explicit UIA Focus-to-Validate takes effect; error preserves initiating Validate focus | PASS for tested sequence |
| Bounds | Native move/resize affects real layout bounds; restoration checks pass at current DPI | PASS at this configuration |
| Stale elements | Retained Invoke after removal and Value after close return ElementNotAvailableException, HRESULT `0x80040201` | PASS for tested normal lifecycle |
| Shutdown | Exit code 0, provider detached, no owned child remains | PASS |
| UIA LabeledBy | Not exposed by this provider despite authored AccessKit relation; accessible input name exists | LIMITATION |
| TextPattern/caret/selection | Not advertised; ValuePattern is not complete accessible editing | UNSUPPORTED in this spike |
| Live error announcement | No live/notification semantics implemented; visible/exposed text is insufficient evidence of speech | NOT IMPLEMENTED / speech untested |

The native exception's message is “Could not open the process token,” but its actual inner type and HRESULT are UIA element-unavailable; this is not classified as an elevation failure. This review does not infer external foreground restoration, all shutdown races or cross-build compatibility from the normal close test.

### New synthetic keyboard run, distinct from physical input

The installed Computer Use runtime (`@oai/sky`) was available this session. A second instance of the same executable was launched; one returned window was selected, and focus/state were observed after each individual action. No Windows-key shortcut or shell modification was used. [Synthetic keyboard log](evidence-stage2c-acceptance/synthetic-keyboard.json) records the actual observations.

- Typed `abcd`, moved Left, typed `X`; actual value became **`abcXd`**, demonstrating edit-in-middle on this synthetic path.
- Ctrl+A/Backspace cleared text; Tab focused Validate; Enter created the error without changing Validate focus.
- Tab focused Motion; Space produced a visibly checked checkbox and reduced local preference. A fresh computer-use screenshot showed its blue focus outline. Reverse Shift+Tab returned to Validate.
- Forward Tab reached the conditional Dismiss action; Enter removed the error/action and UIA focus returned to Input.
- Escape requested exit. Immediate enumeration still returned the closing window; subsequent enumeration and process check found none. Asynchronous close was not mislabeled as an immediate disappearance.

These are executed synthetic input observations, not human physical-keyboard results, UIA text-range support or Narrator evidence. The computer-use accessibility formatter does not show a checked-state field here; the Space check used fresh visible pixels/local preference text. The independent UIA run verifies ToggleState directly. No previous-run capture is used as proof of today's keyboard sequence.

## Essential blocked checks

| Required test | Status and exact limitation |
| --- | --- |
| Physical keyboard-only navigation/editing/operation | **BLOCKED:** no human-operated physical-keyboard session or human attestation was available. Synthetic keys work now but are not a physical test |
| Narrator identifies and operates controls | **BLOCKED:** no audio/speech observation channel or human Narrator operator was available. Narrator executable exists (file version 10.0.26100.8972); installation is not operation evidence. Narrator was not launched merely to imply a test |
| Error speech/non-disruptive announcement | **BLOCKED / absent implementation:** no live error semantics or spoken result; requires an explicit tested disposition |
| Screen-reader caret/selection/edit-in-middle | **BLOCKED:** no TextPattern implementation and no Narrator editing result; synthetic editing does not establish assistive editing support |
| External app focus restoration/switching, high contrast, mixed/non-default DPI | **NOT RUN:** local focus and current-DPI geometry checks do not cover these scenarios |

The computer-use skill was read and its runtime initialized rather than assuming the desktop was unavailable. This session can automate native input; it cannot manufacture a physical operator or evaluate unheard Narrator speech. Previous stages' input-tool availability statements are historical and unchanged. Follow the existing [interactive acceptance procedure](STAGE-2C-TEST-MATRIX.md#pending-interactive-acceptance) with a human operator, recording actual speech/input observations and binary/environment identity.

## Unsafe code, lifecycle and maintenance review

The spike's **two own unsafe blocks** are confined to `native.rs`: query HWND thread/visibility before attachment and query DPI while the thread-local owner retains the handle. HWND originates in iced's live-window callback, ownership is checked, and normal detach runs on that thread. No direct arbitrary pointer dereference, global hook or foreign-window manipulation was found in the experiment. AccessKit and Windows binding internals still contain unsafe platform code; this is not a formal memory-safety proof.

Positive boundaries: hidden creation before subclass attachment; thread-local non-Send provider owner; immutable derived initial-tree snapshot; action callbacks queue rather than waiting on the UI; model mutation on iced's thread; event raising after snapshot/owner borrows release; removed-control IDs not reused for a new error instance; normal teardown demonstrated again. Forced termination, callback reentrancy/overload and recovery remain untested. A normal close does not validate panic/forced-exit cleanup.

Production concerns from source review, kept separate from observed failures:

| Concern | Source / implication | Disposition |
| --- | --- | --- |
| Overload silently loses UIA operations | `native.rs:33`, `try_send` logs/drops full/closed requests; no failure acknowledgement or recovery to the client | Needs an explicit bounded command/error policy before production; no stress result claimed |
| Value updates have an unadvertised byte limit | `semantics.rs:65`, SetValue over 4096 UTF-8 bytes is ignored while ValuePattern remains editable | Resolve capability/error semantics; large-value behavior not runtime-tested in this review |
| Async layout/focus results lack generation checks | `main.rs:124` combines returned Bounds with current state; `main.rs:177` accepts InputFocused without a request generation | Potential stale snapshot/focus race during rapid interaction; not reproduced by the low-rate tests |
| Initial hidden tree has no measured bounds | `main.rs:101` constructs with empty Bounds before the first published layout | Visible startup passed; early hidden-window queries/provider reactivation remain unqualified |
| Attachment failure can look like successful process exit | `main.rs:120` logs then calls iced::exit; main maps successful iced return to success | Source-level diagnostic concern; failing startup was not induced |
| Essential semantic gaps | No TextPattern, live error event, UIA LabeledBy mapping; local motion flag only, no high-contrast adaptation | Human acceptance and widget/provider-level remediation needed; no conformance claim |

References are to the reviewed unchanged revision. These observations are engineering risks, not fabricated runtime failures or a claim that every race has occurred.

Maintenance recheck: official [AccessKit releases](https://github.com/AccessKit/accesskit/releases) still list the pinned released provider/core versions, and [iced releases](https://github.com/iced-rs/iced/releases) list 0.14.0. The [Windows adapter contract](https://docs.rs/accesskit_windows/latest/accesskit_windows/struct.SubclassingAdapter.html) still requires owning-thread, pre-show initialization. Recent releases establish active upstream work, not a maintained iced semantic integration or product support guarantee. No dependency update, fork or patch was introduced. AccessKit is MIT OR Apache-2.0 with Chromium-derived BSD notices; prior license observations remain applicable, not a complete license/security audit. The graph remains 114 Windows normal package/version entries (including the experiment), four direct dependencies and 240 all-target locked packages. Generic shell/menu/text support still requires project-owned mapping and tests beyond these fixed controls.

## Resource observation

New release sample after exercising the provider: **30.006136 s**, CPU delta **0.015625 s**, **0.052073% of one logical CPU core**, working set **23,609,344 bytes**, private commit **5,156,864 bytes**. Refreshed process counters and Stopwatch wall time, after 3 s settling; the separate UIA client is excluded. No ETW/wakeup, screen-reader-attached, sustained-load or hidden-window profile was executed. This short sample does not supersede the prior nonzero samples or establish a budget. No app accessibility polling loop was found; test-client bounded polling is separate.

## Smallest viable alternative and release decision

Do **not** use a discovered UIA tree or whole-string ValuePattern as the production text-control architecture. Keep Rust + iced and the narrow owned-HWND boundary. The smallest candidate alternative to expanding the current manual tree is a **widget-owned semantic text adapter** driven by the real text widget's editing/layout state, using released AccessKit text-range/caret/selection facilities where the pinned iced boundary permits them, plus an explicit error-status notification path. Reuse the verified Button/Toggle mapping; avoid invisible native duplicate controls, a parallel renderer or an unreviewed fork.

That alternative is a recommendation to investigate only if human acceptance proves those capabilities essential and missing; its viability is not established by this review. First run Narrator and physical keyboard on this exact window. If Value-only input is unusable, establish access to real caret/selection/layout before implementing a small adapter. If that boundary cannot be supported maintainably, revisit a maintained upstream accessibility strategy or the framework decision explicitly; do not grow a fragile parallel UI framework to conceal the gap.

**Stage 3 production catalog/launch-service planning is deferred because acceptance is BLOCKED.** No production UI, launcher, taskbar, dock, AppBar or Windows-key integration was added. This review adds only the acceptance report and fresh evidence. All pre-existing files/evidence are preserved; final hash/diff/link and process-cleanup results are recorded in the new evidence index. Work is uncommitted on `main`.
