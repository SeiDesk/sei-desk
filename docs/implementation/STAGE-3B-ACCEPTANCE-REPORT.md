# Stage 3B — Worker lifecycle fix and re-acceptance

Date: 2026-10-15. Verdict: **PASS for the headless consumer on the tested Windows host.** The confirmed completion-loss defect is fixed and fresh required tests pass. This does not approve production UI integration, waive accessibility requirements, or qualify every Windows build.

## Review input and preserved history

Tested the uncommitted working tree on `main`, HEAD `e11af2cf6baea5c20e1edc7d48c603c6ab77397c`, including the pre-existing Stage 3A acceptance and Stage 3B implementation/review work. No commit or push was performed. The earlier FAIL report is preserved verbatim as [acceptance-report-before.md](evidence-stage3b-lifecycle-fix/acceptance-report-before.md); all earlier execution evidence remains unchanged. The previous 42-pass/1-fail result is historical, not a fresh test result.

Read consumer/runtime/tests, lib exports, Windows fixture, PowerShell runner, architecture/matrix/evidence, Stage 3A acceptance, and current Stage 2D accessibility disposition before modifying code. [New evidence](evidence-stage3b-lifecycle-fix/README.md) records exact tested source hashes, gates, both successful and failed runs, preservation checks and resource observations.

## Root cause and repaired invariant

Invariant: each dispatched, occupied worker slot must enqueue exactly one terminal result on normal return or recoverable Rust unwind, even when its normal completion continuation is skipped. The old implementation caught a service panic, then disposed its payload during `unwrap_or`, before sending completion. Payload disposal could panic. The worker then exited without an event, while the owner retained a channel sender: public `wait()` could remain blocked and the slot remained Running.

Rust documents this payload-disposal behavior in [catch_unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html). The original failure regression remains unchanged and now passes: `worker_join_panicked=true`, **`terminal_completion_present=true`**.

The targeted repair adds a private `TerminalPublisher` with a fallback WorkerPanicked result. It is installed at worker entry, before service or payload code. Normal completion replaces the fallback and consumes the guard; otherwise normal return or unwinding drops the guard and sends the fallback. Normal catch/convert/send code is no longer the sole route to notification. The guard owns only an operation result, not a second model of operation state.

The owner uses `join_worker` for completion and shutdown joins. If join returns a panic payload, disposal happens outside unwinding; narrow catch_unwind around that disposal consumes replacement payloads until a finite nested cleanup sequence ends. This avoids unwinding the owner or leaking caught payloads. Service/state code is not wrapped in a new broad catch, and panic hooks remain active (expected panic messages are in test logs).

### Publication, wakeup and race argument

- One guard per worker, one Option::take send per guard. No normal send plus fallback send. No additional worker, event stream, polling or timeout is introduced.
- At most one worker per lane, two occupied slots and channel capacity two; unconsumed results keep their slots occupied. Both workers can publish while shutdown joins without draining. No consumer/state lock is held during joining, native calls or sends.
- The same blocking receive wakes for a real result or fallback. Tests cover failure before receipt and explicitly gated failure after the waiter is started; either scheduling order works without sleeps.
- Completion identity and occupied-handle checks still reject stale/duplicate messages. Single-owner state mutation linearizes cancellation and completion. Existing checked IDs, explicit selection, snapshot-generation rejection and bounded concurrency remain intact.
- Once a PID/result is published, later worker cleanup failure cannot replace it with fallback failure. Cancellation does not hide accepted launch success. Failure without a result means the external outcome is unknown; never automatically retry a possibly accepted launch.
- Shutdown marks Disposed before joining; queued fallback or normal events are discarded, never published after disposal. Pending operations retain their existing cancellation/spawn-failure terminal paths.

The guarantee applies while the process is alive and Rust unwinding/cleanup terminates. It is not a claim that arbitrary code is preemptible. Process abort, panic=abort, forced termination, fatal native errors, a panic hook that aborts, or a double panic during unwinding cannot be recovered in-process. A destructor that never returns or endlessly creates a new panicking payload may prevent join, like a forever-blocked Service/OS call. These are explicit remaining limits, not hidden timers or leaked/detached workers. Finite nested payload failures are executed and verified.

## Focused regression coverage

All four previous acceptance tests are retained; seven additional tests extend coverage. **11 focused tests passed**; **31 consumer tests passed** in both ordinary execution and a separate release-mode run.

| Test | What it establishes |
| --- | --- |
| Original terminated-worker payload-drop regression | Worker exits with panic, but fallback completion exists. Test body unchanged. |
| Public wait after payload-cleanup panic | Refresh and launch both return defined Failed(WorkerPanicked), clear slots and leave no duplicate event. |
| Exit without normal publication | Private worker-boundary fixture exercises real guard/receive/join on early return and unwind. |
| Published acceptance followed by panic/cancel/duplicate | PID remains success; no second queued terminal result; injected duplicate failure is stale. |
| Nested join-payload destruction failure | Owner safely disposes finite successive panicking payloads and returns failure. |
| Shutdown with fallback exit | Idempotent joins and Disposed state, no late publication. |
| Both fallback exits before shutdown drains | Two-event queue bound holds without deadlock or duplicate publication. |
| Explicitly gated service failure / waiter | Failure notification completes the public wait without polling or timing-dependent sleeps. |
| Existing repeated cancellation, blocked-service shutdown and ordinary launch panic | Ownership, cancellation truth and worker-failure behavior remain correct. |

Injected Service/worker tests use real threads and channels, but are not proof of actual Windows API failure or abort recovery. Ten-second test deadlines are hang safeguards only; runtime uses no deadline or polling. Existing blocked-call tests release simulated blocking services through channels; no real uninterruptible Windows call was forcibly interrupted.

## Fresh validation and environment

Windows NT **10.0.26300.0**, x64/MSVC, `rustc 1.99.0 (b940084d7 2026-09-28)`. Release resource metadata confirms **AdministratorRoleInCurrentToken=false**. Filesystem sandbox exceptions were used for owned temporary-file operations, not Windows RunAs/UAC.

Final established runner:

```powershell
./tools/Test-Stage3B.ps1 -OutputDirectory docs/implementation/evidence-stage3b-lifecycle-fix/windows-final
```

Overall runner exit **0**. Its preserved Stage 3A runner executes:

| Command | Fresh final result |
| --- | --- |
| `cargo fmt --all --check` | Exit 0 |
| `cargo check --workspace --all-targets --locked --offline` | Exit 0 |
| `cargo test --workspace --locked --offline` | Exit 0: 56 passed, 5 deliberately ignored opt-in fixtures |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Exit 0 |
| `cargo build --workspace --release --locked --offline` | Exit 0 |
| `cargo build -p sei-apps --release --examples --locked --offline` | Exit 0 |
| `cargo test -p sei-apps --locked --offline -- --include-ignored` | Exit 0: **47 library + 1 consumer Windows + 2 launch integration = 50 passed**, no failures or ignored tests |
| `git diff --check` | Exit 0; final consistency check recorded separately |

Additional executions:

```powershell
cargo test -p sei-apps --lib consumer::tests::acceptance --locked --offline -- --nocapture
cargo test -p sei-apps --release --lib consumer:: --locked --offline
```

Both exit **0**, respectively 11 and 31 passed. Counts overlap and must not be summed as unique tests. The workspace's 5 ordinary ignored fixtures were explicitly enabled in the service suite.

### Investigated failure, preserved separately

The initial sandboxed runner passed workspace gates but its enabled native suite returned **44 passed, 1 failed**: the pre-existing shortcut test's own Duplicate.lnk → Renamed.lnk operation failed at native.rs:376 with OS error 5 (Access denied). No worker regression failed. The same fixture and full runner passed outside the filesystem sandbox with no source change and without administrator privileges. That failure remains in `windows-run/suite/windows-tests.txt`; it is not erased or counted as a pass. An intermediate unrestricted run passed 48 service tests before the final two regressions were added; the definitive final run above passed all 50.

## Actual Windows behavior and resource evidence

Fresh Windows fixtures execute the existing real COM/shortcut and CreateProcess paths. The consumer Windows fixture injects discovery data but launches only its owned real executable, verifying structured arguments, working directory, explicit selection, pre-dispatch cancellation, stale target errors and cleanup. Separate NativeService probes perform real known-folder/COM enumeration and do not launch discovered installed apps. None of these tests is a production UI or accessibility test.

Final release consumer probe: two refreshes, **220.431 / 166.261 ms**, identical **78 entries / 252 visited / 83 issues**, no truncation, 600-byte inline Consumer. Observation **2.4721092 s**, 50-ms samples, max working set **19,189,760 bytes**, max private commit **3,584,000 bytes**, last sampled CPU **0.375 s**, exit 0 and owned child remaining false. The consecutive direct-service probe used 19,443,712 / 3,661,824 bytes and 0.328125 s CPU over 2.4803733 s. This includes refresh work and a two-second hold, not a pure idle benchmark or guaranteed overhead. No new allocation-heavy service, dependency, unsafe code, GUI or background polling was introduced. Long-duration leak/resource qualification remains unrun.

## Changed files and remaining boundaries

Runtime: `crates/sei-apps/src/consumer.rs` (terminal guard and join-payload disposal). Tests: `crates/sei-apps/src/consumer/tests/acceptance.rs` (seven added regressions; original four retained). Documentation: this report, Stage 3B architecture and matrix. New evidence: `docs/implementation/evidence-stage3b-lifecycle-fix/`. Baseline hashes verify other user work, original evidence, manifests, designs and accessibility code remain unchanged. No test was removed, ignored or weakened to get a pass.

No remaining failed required headless acceptance check exists on this host. Other Windows builds, real spawn exhaustion, fatal crashes, indefinite native blocking, adversarial nontermination and long-duration leaks are not qualified. The static test-runner exceptional-metric-collection cleanup limitation from the earlier review remains a follow-up; ordinary and timeout ownership are not a guarantee for every observer exception. `drain_ready()` joins and must not be treated as a hard nonblocking GUI call.

Recommend an independent review of this narrow lifecycle patch next. A later headless adapter still needs command/completion wakeup design to avoid blocking a future UI thread. **Do not begin production UI integration. The separate production accessibility gate remains closed**, including the approved headless-only text-range/caret/selection deferral. No Stage 3 UI or accessibility workaround was started.
