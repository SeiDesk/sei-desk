# Stage 3B — Independent acceptance review

Date: 2026-10-15. Verdict: **FAIL**. The current headless consumer is not accepted as a foundation for UI integration until the worker-completion defect below is fixed and the regression passes. Normal-path Windows tests and Rust compilation passed; the expanded test suite did not. No production UI is authorized by this review.

## Scope and provenance

Reviewed the working tree on `main`, based on HEAD `e11af2cf6baea5c20e1edc7d48c603c6ab77397c`, including the uncommitted Stage 3A acceptance and Stage 3B work. This is not a review solely of that committed revision. Read the consumer, its original tests, exports, Windows fixture, runner, architecture, matrix, evidence index, Stage 3A acceptance, and current Stage 2 accessibility disposition. Existing work and evidence were preserved.

A fresh run of `tools/Test-Stage3B.ps1` preceded review changes. Four deterministic regression tests were then added, and quality gates and real Windows fixtures rerun. See the [new evidence index](evidence-stage3b-acceptance/README.md). Historical evidence is context only; none is relabeled as a new execution. This review adds tests and documentation only; the consumer runtime is unchanged.

## Confirmed defect

### P2 — A terminated worker can leave its operation permanently Running

Location: `crates/sei-apps/src/consumer.rs`, the worker closure at lines 308–344, and `Consumer::wait` at line 497.

The worker converts a caught unwind with `unwrap_or`, then sends its completion. Dropping the caught panic payload can itself panic, escaping that conversion before the send. Rust explicitly documents this behavior in [catch_unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html). The owner retains a channel sender, so the receiver cannot infer worker termination from channel disconnection. The operation remains Running, repeated requests remain Busy, and `wait()` can block indefinitely despite the worker having exited. Cancellation does not provide a terminal event in this condition.

The new `terminated_worker_with_panicking_payload_drop_must_report_failure` test uses an injected Service whose panic payload destructor panics. It deterministically joins the worker, then checks the private completion receiver: `worker_join_panicked=true`, `terminal_completion_present=false`. The test fails. It deliberately avoids calling the public blocking `wait()` on the known-empty queue. Worker termination and missing completion were executed; the indefinite-wait consequence follows from the inspected channel ownership, rather than a deliberately hung test process. This is not evidence that NativeService's ordinary Windows calls produced this panic.

Reproduce with:

```powershell
cargo test -p sei-apps --lib --locked --offline terminated_worker_with_panicking_payload_drop_must_report_failure -- --nocapture
```

Required fix: ensure an unexpectedly exited worker produces a terminal failure that the owner can observe without relying solely on a successful send from that worker. Cover failure while converting/disposing panic payloads in both refresh and launch lanes. Preserve resource ownership and honest launch outcomes; do not solve this by leaking payloads, silently detaching workers, or claiming a process was cancelled after creation. Re-run the full suite after the fix. Runtime changes are intentionally left to that follow-up, rather than accepting an implementation with a failing regression.

## Verified state and concurrency behavior

| Area | Independent finding and evidence |
| --- | --- |
| Refresh lifecycle | Idle → Pending → Running → success/failure/cancel is covered. Redundant same-lane requests are rejected while the slot is occupied, including cancellation pending. Failed/cancelled refresh retains the previous snapshot. |
| Bounds | At most one refresh worker and one launch worker; two completion slots. No permanent worker or background polling. The ordinary-path bounded-send argument is sound, but it does not establish terminal notification for every worker exit, as the defect demonstrates. |
| Stale generations | Completion IDs must match the current running operation. Replayed and out-of-order refresh completions are discarded without snapshot mutation. Because real refresh work is serialized, same-lane out-of-order completion is injected into the private completion path; it is not a claim of two simultaneous native scans. Real controlled refresh/launch workers complete in both cross-lane orders. |
| IDs | Checked monotonic allocation shared across lanes; the maximum ID is issued once, then IdExhausted. Tests verify no wrap or mutation on exhaustion. |
| Selection | Explicit membership in the current snapshot is required. Invalid selection clears the prior selection. Successful replacement invalidates selection even with the same entry ID. Refresh/selection never launches an app. |
| Launch intent | Request captures the selected request and generation. A replacement before dispatch rejects stale intent; changing selection alone does not retarget accepted pending intent. Native launch revalidates its target. |
| Outcomes | Pending cancellation prevents dispatch. Cooperative pre-creation cancellation is distinct from success. Acceptance/PID is reported even if cancellation or refresh follows creation; it does not mean the launched app completed, remains healthy, or can be cancelled. Failures are structured, and selection survives ordinary launch failure. |
| Locks and callbacks | No consumer shared-state lock is held across enumeration or process creation. State has one owner; Service calls execute on owned workers. Native calls are outside the state-transition methods. No unsafe block or new dependency was added by this review. |
| Worker panics | Original ordinary refresh panic and new ordinary launch panic tests pass. The latter preserves selection and does not retry. The unusual panic-payload cleanup path fails as described above. |
| Shutdown | Pending work is removed, cancellation signalled, and the owner becomes Disposed. Running handles are joined; late private completions are drained without publishing snapshots. The new barrier/channel test verifies both blocked fake services remain owned until released, then shutdown completes with no retained handles. |
| Repeated cancellation | Three repeated cancellation/dispatch attempts create no extra workers; slots remain Busy until completion. Accepted launch success remains visible and cancelled refresh does not replace the snapshot. |

The original 20 consumer tests passed; the four new review tests produced **3 passed, 1 failed**. These controlled Service tests exercise real Rust threads and channels, but are not Windows API evidence.

## Windows execution and fixture validity

Environment: Windows NT 10.0.26300.0, Rust 1.99.0 (`b940084d7`, 2026-09-28), non-administrator token. This validates the available Windows build only; it is not a Windows 10/11 compatibility matrix or minimum-Rust-version qualification.

The independently rerun `consumer_windows` fixture passed. Its catalog is injected: it does not prove native discovery. Its launch backend calls the real launch service and owns the real fixture child; the test exercises actual Windows process creation, structured arguments, working directory, lifecycle and stale-target behavior. The two existing `windows_launch` tests also passed. They launch only owned fixtures, not arbitrary installed applications. Native discovery was separately exercised by the release consumer probe against real Start Menu sources, without launching discovered entries.

Source inspection confirms the consumer delegates structured requests to the existing launch service, performs no shell command construction or automatic elevation, and does not execute entries while refreshing. No installed-app launch, UIA test, Narrator test, accessibility workaround, shell replacement, or persistent Windows setting change was part of this review.

## Reproducible gates and results

Initial fresh documented runner (before adding regressions): exit 0; ordinary workspace tests **45 passed, 5 ignored**; opt-in service run **39 passed** (36 library, one consumer fixture, two launch fixtures). All original quality gates passed. These results are newly executed, not copied from the prior Stage 3B evidence.

Current review tree:

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo check --workspace --all-targets --locked --offline` | Passed |
| `cargo test --workspace --locked --offline` | Failed: new worker-completion regression; sei-apps library 37 passed, 1 failed, 2 ignored. Cargo stopped before later workspace targets. |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Passed |
| `cargo build --workspace --release --locked --offline` | Passed |
| `cargo build -p sei-apps --release --examples --locked --offline` | Passed |
| `cargo test -p sei-apps --lib --locked --offline -- --include-ignored` | 39 passed, 1 failed |
| `cargo test -p sei-apps --test consumer_windows --locked --offline -- --ignored` | 1 passed, actual Windows fixture |
| `cargo test -p sei-apps --test windows_launch --locked --offline -- --ignored` | 2 passed, actual Windows fixtures |
| `git diff --check` | Passed; final preservation/whitespace checks recorded in the new evidence directory. |

The enabled current-tree service aggregate is **42 passed, 1 failed** across the library and three integration tests. Repeated runs are not additional unique test cases. The failing test is retained, not ignored or marked as passed.

## Resource observations

Fresh release probes, 50 ms sampling, two refreshes plus a two-second idle hold:

| Observation | Stage 3A direct service probe | Stage 3B consumer probe |
| --- | --- | --- |
| Observed duration | 2.5879945 s | 2.5674895 s |
| Peak sampled working set | 19,578,880 bytes | 19,423,232 bytes |
| Peak sampled private commit | 3,727,360 bytes | 3,813,376 bytes |
| Last sampled process CPU time | 0.40625 s | 0.421875 s |
| Exit / owned child remaining | 0 / false | 0 / false |

Consumer inline size: 600 bytes; this excludes heap allocations, catalog storage, thread stacks, channels and native allocations. Consumer refreshes took 239.307 ms and 182.124 ms; both returned 78 entries, 252 visited items, 83 issues, no truncation, and identical entries. These are observed local results, not guarantees or a pure idle CPU measurement. The short runs include discovery, uncontrolled filesystem cache effects and sampling uncertainty; the differences cannot establish a meaningful overhead regression or steady-state idle budget. Long-duration resource and repeated-cycle leak tests remain unrun.

## Runner and evidence review

`Test-Stage3B.ps1` requires a fresh directory, invokes the existing Stage 3A quality/Windows fixture runner, then measures the release consumer probe. It records timestamp, OS, compiler, executable hashes, token elevation status, metrics, exit code and owned-process status. The probe has an explicit 30-second timeout; ordinary shutdown and timeout cleanup are separate from consumer cancellation. The normal fresh run exited cleanly. Static follow-up: if metric collection itself throws before timeout, the runner's finally block disposes the process handle but does not explicitly terminate/wait the still-running owned probe. That exceptional observer path was not injected or observed here and should be hardened/tested before relying on it for unattended failure cleanup.

The initial preservation manifest records hashes of pre-existing tracked and untracked work. The final comparison allows only the appended review-test module declaration in the existing consumer test file; the new test module, report and evidence are additions. Runtime, manifests, Stage 3A acceptance work, designs, accessibility spike and prior evidence remain unchanged. No changes were staged, committed or pushed by this review.

## Limitations and required follow-up

1. **Acceptance blocker:** repair worker-exit notification and make the retained failing regression pass, with equivalent launch-lane coverage. Repeat every quality gate and Windows fixture.
2. **Blocked native calls:** cancellation is cooperative and cannot interrupt an arbitrary synchronous Windows call. `shutdown` and Drop join and may wait indefinitely. New tests use bounded simulated blocking services released by explicit gates; indefinitely blocked real Windows calls, forced termination and process-aborting panics were not tested. Graceful ownership is not a bounded shutdown guarantee.
3. **Future UI adapter:** the owner needs event-driven completion wakeup and command delivery so cancellation can be serviced while work is running. Blocking `wait`, shutdown or Drop must not run on the GUI thread. `drain_ready` joins received workers and should not be treated as a hard nonblocking GUI operation: [thread join also waits for thread-local destructors](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html). Design this adapter after the liveness fix, without adding continuous polling.
4. **Unexecuted failure/compatibility cases:** real thread-spawn exhaustion, fatal native failures, hostile/hung Service implementations, long-duration leaks, additional Windows builds and real OS-call stalls need separate qualification. No new unsafe/native/COM boundary was introduced; existing Stage 3A limitations still apply.
5. **Accessibility remains gated:** Stage 2D allows only headless Stage 3 work under the approved limited deferral. Production launcher text-range/caret/selection semantics and other production accessibility requirements remain unresolved; this review neither retests nor waives them.

The smallest next step is a focused headless worker-lifecycle fix and acceptance rerun. After that passes, an event-driven headless adapter test can establish host command/completion delivery. Do not begin production UI integration from this FAIL verdict.
