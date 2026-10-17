# Stage 3B — Independent lifecycle patch review

Date: 2026-10-15. Verdict: **PASS for the headless lifecycle patch on the tested Windows host.** No material correctness/regression defect was found within the documented recoverable-unwind contract. This does not authorize UI integration or accessibility approval.

## Exact scope and preservation

Reviewed the uncommitted working tree on `main`, HEAD `e11af2cf6baea5c20e1edc7d48c603c6ab77397c`. HEAD is the base, not the complete reviewed implementation: the consumer, tests and reports are untracked, and earlier Stage 3A/lib/native-test edits remain uncommitted. Inspected the full tracked diff and the untracked consumer, acceptance tests, architecture/matrix/report, evidence index, related state tests and Windows fixture/runner. All source hashes listed in the lifecycle-fix `tested-tree.json` match this review's input; see [input verification](evidence-stage3b-patch-review/input-verification.json).

This review changes no runtime, tests, existing reports, design assets, dependencies or accessibility code. Adds only this report and [separate review evidence](evidence-stage3b-patch-review/README.md). A pre-review hash manifest includes tracked and untracked work; the final comparison verifies preservation, including original FAIL evidence. Nothing was reset, staged, committed or pushed.

## Lifecycle findings

| Source reference | Finding |
| --- | --- |
| `consumer.rs:132–156` | TerminalPublisher starts with a correctly typed failure for its operation ID. `publish(self, result)` replaces only the inert fallback, consumes the guard and sends through Drop. The single Option::take prevents a second send from this guard. |
| `consumer.rs:353–391` | Worker entry installs the guard before entering Service/caught-payload code. Ordinary success/error reaches publish; panic-payload destruction escaping unwrap_or instead unwinds through the live guard and sends its fallback. There is no return/forget/detach path bypassing this guard after dispatch. |
| `consumer.rs:158–170` | join_worker catches only destruction of an unsuccessful join's payload. Each replacement payload is moved into the next disposal attempt, not leaked. Finite nested cleanup panics cannot unwind the owner from this path. The loop has no finite iteration guarantee for malicious/nonterminating payload code, which is expressly documented. |
| `consumer.rs:464–530` | Only the current lane's occupied ID/handle may finish. The owner removes the slot, joins without a state lock, then applies one terminal state. Duplicate/stale messages cannot overwrite it. Refresh cancellation retains precedence; accepted launch PID retains precedence over cancellation or later worker-cleanup panic. |
| `consumer.rs:543–559` | wait receives a channel event, not a condition flag checked separately from its notification. The guard's fallback is received through the same path as a normal result. A completed worker cannot leave this receive waiting merely because its ordinary completion continuation was skipped under the supported unwind contract. |
| `consumer.rs:563–590` | Shutdown freezes state as Disposed, cancels requests, retains running ownership, joins and drains private completions. No snapshot or terminal public result is republished after disposal. It joins workers, not launched applications. |

The ordinary Result/publication path no longer needs to survive panic-payload disposal. The [Rust destructor rules](https://doc.rust-lang.org/reference/destructors.html) support the guard's scope cleanup, and [catch_unwind explicitly warns that caught payload destruction may panic](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html). The fresh original regression actually printed `worker_join_panicked=true` and `terminal_completion_present=true`; this is not inferred from compilation.

The guarantee is a scope-lifetime protocol, not an independent OS worker watcher. A living process with normal Rust return/recoverable unwind and terminating cleanup is required. Process abort, panic=abort, forced termination, foreign fatal failures, a failing panic hook, or a destructive double panic during unwinding may prevent in-process cleanup altogether. A service, destructor, thread-local teardown or infinite replacement-payload chain can block joining indefinitely. Notification can precede complete thread teardown; [join waits for termination, including thread-local destructors](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html). The reports correctly keep these limits explicit. No claim of bounded external-call cancellation or bounded wait/shutdown is accepted here.

## Concurrency, cancellation and ownership

One refresh and one launch slot bound concurrency to two workers. A worker sends once, and its slot remains occupied until receipt/join. Consequently the capacity-two channel has room even when shutdown joins both before receiving. A disconnected receiver is not reachable through normal public shutdown, because the owner retains it until workers are joined. This proof depends on the private protocol; arbitrary private-message injection in tests is not a production caller capability.

The mutable model has one owner. Channel synchronization carries result ownership; there is no consumer mutex or condition-variable lost-wakeup race. Cancellation and completion application are serialized by that owner. Thread completion may happen before start_pending stores the handle, but the owner cannot drain while inside that method, so the pending send remains available for later receipt. No consumer lock is held during native calls, send or join. Backend-private lock poisoning may cause a recoverable service panic and failure; it cannot poison a consumer state mutex because none exists. Recovery does not certify the internal consistency of an arbitrary custom backend; NativeService is stateless and custom backends remain qualified/injected boundaries.

Checked u64 operation IDs still exhaust without wrapping. Explicit selection and generation checks are unchanged. Refreshes remain serial with redundant requests rejected; refresh and launch may overlap, with separate states. Refresh/selection never launch automatically. A cancelled pending launch never reaches creation; an accepted PID is success, not process completion or proof that cancellation can terminate it. A WorkerPanicked result without a published PID has unknown external side effects and must not trigger automatic retry. No unsafe code, new dependency, extra worker, shared state authority, production polling or arbitrary timeout recovery was added by the patch.

## Test quality

Inspected all eleven acceptance tests and related consumer cases. They cover the original payload-destructor panic, public wait in both lanes, exit without normal publication on return/unwind, finite join-payload destruction failures, acceptance followed by panic/cancel and a stale duplicate, two fallback exits during shutdown, explicitly gated service failure, repeated cancellation, ordinary launch panic and retained blocked-worker ownership. Existing generation/overflow/selection and cross-lane completion-order tests remain intact.

The original failure test deterministically joins before checking the queue, and does not hang the public wait. Its missing-completion assertion would fail on the previous implementation. Public wait tests complement it rather than treating the private receiver test as sufficient. Exit-only fixtures deliberately bypass normal publication but use the actual guard/channel/wait/finish/join; they are not native-crash claims and some do not set the production Running status. Public-path tests cover that production transition separately.

The gated-wait test starts a waiter before releasing the service but does not prove the receiver is already parked in the kernel; its own comment accurately permits either receive ordering. Channel semantics and the queued-before-wait/public-failure cases establish absence of a lost condition-flag notification. Tests do not depend on scheduling sleeps. Receive deadlines are failure safeguards, not runtime cancellation or timing assumptions. On a failed deadline an unrecoverably stuck Rust test worker cannot be safely force-killed; the passing executions have completed and joined. This is a test-harness limit, not a bounded production cleanup guarantee.

The two-exit shutdown test detects extra sends that would exhaust the queue, and explicit drain/duplicate tests check exactly-once observable results. Out-of-order same-lane completions are intentionally private-message injections because native refresh concurrency is serial. Blocked-call tests use explicitly released fake services; they do not establish interruption of an actually stuck Windows call. No meaningful uncovered recoverable-path gap required adding a test during this review; no test was weakened, removed or ignored.

## Fresh execution

Environment: Windows NT **10.0.26300.0**, x64/MSVC, Rust **1.99.0 (b940084d7 2026-09-28)**; probe metadata reports a non-administrator token. All results below were executed during this review, not copied from the fix's logs.

Established full runner (exit **0**):

```powershell
./tools/Test-Stage3B.ps1 -OutputDirectory docs/implementation/evidence-stage3b-patch-review/windows-run
```

| Check | Observed result |
| --- | --- |
| cargo fmt --all --check | Exit 0 |
| cargo check --workspace --all-targets --locked --offline | Exit 0 |
| cargo test --workspace --locked --offline | Exit 0: 56 passed, 5 opt-in ignored |
| cargo clippy --workspace --all-targets --locked --offline -- -D warnings | Exit 0 |
| cargo build --workspace --release --locked --offline | Exit 0 |
| cargo build -p sei-apps --release --examples --locked --offline | Exit 0 |
| cargo test -p sei-apps --locked --offline -- --include-ignored | Exit 0: 47 library + 1 consumer Windows + 2 launch integration = 50 passed; none ignored |
| cargo test -p sei-apps --lib consumer::tests::acceptance --locked --offline -- --nocapture | Exit 0: 11 passed |
| cargo test -p sei-apps --release --lib consumer:: --locked --offline | Exit 0: 31 passed |
| git diff --check | Exit 0 |

Counts overlap and are not summed. The enabled native tests use real COM and owned CreateProcess fixture execution. Consumer discovery is injected in its Windows fixture; actual discovery comes from the separate NativeService release probe. No installed application is launched by enumeration. The previously failing shortcut-rename fixture passed in this fresh run outside the filesystem sandbox, without Windows elevation.

Fresh release consumer probe: 218.904 / 164.840 ms refreshes, 78 entries / 252 visited / 83 issues, no truncation, identical snapshots, shutdown_closed=true. Over 2.5053137 s with 50-ms sampling and two refreshes plus a two-second hold: max working set 19,185,664 bytes; max private commit 3,624,960 bytes; last sampled CPU 0.375 s. Exit 0, ChildRemaining=false. This reproduces deterministic local enumeration; it is not a pure idle benchmark or leak guarantee.

## Evidence findings and concrete issues

The input source hashes match the lifecycle-fix evidence's recorded working tree. Its log counts support the reported 11 / 31 / 50 / 56 passes. The fix's first sandboxed native run really records Access denied and 44 passed / 1 failed; later unrestricted and final runs are separate. The exact denial cause was not re-created during this review; filesystem-restriction attribution is supported by the unchanged fixture's successful outside-sandbox reruns, not a diagnosis of Windows ACLs. The final source comment clarification after the earlier functional run is explicitly disclosed; this review tests the current source including that comment. Original failure captures are still present and hash-preserved.

**No material patch defect found.** No P0/P1/P2 finding blocks the scoped headless PASS.

One pre-existing, disclosed test-runner follow-up remains: **P3, tools/Test-Stage3B.ps1:15–37**. If process metric collection throws before the timeout path, finally disposes the Process handle without explicitly stopping/waiting its still-running owned probe. This is a static exceptional-path observation, not an observed leaked process or a consumer failure. Harden and inject that observer-failure path before relying on the runner for unattended cleanup guarantees. This review leaves it unchanged as outside the lifecycle patch.

## Required next steps and limits

Assess the separate production accessibility requirements and outstanding validation next. The approved text-range/caret/selection deferral applies only to headless work; no accessibility conformance or production launcher approval follows from this PASS. No UIA/Narrator/manual keyboard checks were run in this headless review.

Before a later UI adapter, design event-driven host command/completion delivery so wait, join, drain_ready's join and shutdown/Drop never block the GUI thread. Qualification across additional Windows builds, real thread-start exhaustion, fatal native failures and long-duration leak/idle measurements remains unrun. These limits and the runner follow-up remain explicit. Do not start production launcher, taskbar, dock or shell integration from this review.
