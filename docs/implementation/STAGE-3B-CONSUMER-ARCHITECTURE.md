# Stage 3B — Headless consumer and application state

15 October 2026. **PASS for the headless coordination scope on the tested Windows host.** No production launcher, desktop shell, GUI integration or accessibility-spike modification is included. The [Stage 2D limited deferral](STAGE-2D-ACCESSIBILITY-CLOSURE.md) still excludes production UI until programmatic text-range/caret/selection accessibility has an explicit disposition.

## Smallest structure and service boundary

Add a public `sei_apps::consumer` module inside the existing sei-apps crate; no new crate, dependency, runtime or workspace/lockfile change is needed. It uses only std and the existing headless domain types. See [Stage 3A sources](STAGE-3A-APPLICATION-CATALOG.md), [launch semantics](STAGE-3A-LAUNCH-SERVICE.md), [acceptance review](STAGE-3A-ACCEPTANCE-REPORT.md), [Stage 2A architecture](STAGE-2A-ARCHITECTURE.md), and the [current test matrix](STAGE-3B-TEST-MATRIX.md).

```text
crates/sei-apps/src/consumer.rs        single-owner state, finite worker dispatch
crates/sei-apps/src/consumer/tests.rs  controlled service/channel gate tests
crates/sei-apps/src/lib.rs             extracted blocking refresh body
crates/sei-apps/tests/consumer_windows.rs  opt-in actual fixture process test
crates/sei-apps/examples/consumer_probe.rs count-only read-only resource probe
tools/Test-Stage3B.ps1                 existing full-suite runner + consumer probe
```

The sole pre-existing Stage 3B source edit is lib.rs: export the module and extract the original refresh_async closure into blocking `refresh(&Cancellation, Option<&SyncSender<Progress>>)`. The original async wrapper delegates to it. Native source rules, COM ownership, launch semantics and existing public async callers are preserved. Consumer workers call this blocking body directly, avoiding a wrapper worker that waits on another worker.

`Service` is a narrow blocking, Send + Sync boundary with `refresh` and `launch`, used by NativeService and controlled test services. NativeService is stateless: it uses the Stage 3A native refresh and launch paths on operation-owned threads. No COM object crosses a thread boundary. A custom backend must honor the same bounded/blocking service contract; it is not an untrusted plugin mechanism or evidence of actual Windows behavior.

Native launch returns acceptance/PID, then releases its Child handle. Closing that handle does not terminate the application or wait for it. Window/process lifetime tracking is outside this consumer. The Windows test backend retains its exact Child solely for fixture cleanup; its catalog is injected, while its process creation and target validation use the real Stage 3A service.

## API and ownership

Consumer is owned and mutated by one headless caller. No shared application-state mutex or permanent thread exists. Immutable snapshot/selection getters cannot mutate the selected launch request behind the owner. State transitions do no filesystem enumeration or process creation.

| API | Contract |
| --- | --- |
| `Consumer::new()` | Native backend, no workers until dispatch, both lanes Idle. |
| `with_service(Arc<dyn Service>)` | Explicit injection boundary for deterministic tests or another qualified backend. |
| `request_refresh()` | Reserve the refresh slot and checked operation ID; set Pending. Reject Busy if pending/running/cancelling/unconsumed-complete work still occupies it. |
| `snapshot()` | Borrow the one current Snapshot, including published generation, entries and per-source issues. |
| `select(entry_id)` / `clear_selection()` | Select only a member of the current snapshot. Unknown selection clears any previous selection and returns UnknownEntry. Neither starts a launch. |
| `request_launch()` | Require selection and matching current snapshot membership; latch that entry's request and generation under a new ID. Reject NoSelection, StaleSelection or Busy without creating a process. |
| `start_pending()` | Explicitly start existing Pending requests in finite workers. Recheck the pending launch's snapshot membership before dispatch. No implicit refresh or launch is synthesized. |
| `refresh_status()` / `launch_status()` | Independent typed Status values, with operation IDs and separate refresh summary / launch PID payloads. Only the latest state per lane is retained. |
| `cancel(operation, id)` | Target this exact occupied operation. Return false for a stale ID/no work, rather than cancelling a newer request. |
| `wait()` | Blocking receipt of one completion on the headless caller thread. Return None if no worker runs; Pending alone is not a reason to block. Never use on a GUI thread. |
| `drain_ready()` | Caller-driven drain of at most two queued completions. Each receipt joins its worker, so this is not a hard nonblocking GUI operation. No internal polling schedule. |
| `last_discarded()` | One bounded diagnostic notice for an obsolete completion, without changing current operation state. |
| `begin_shutdown()` | Nonblocking terminal disposal: cancel requests, remove unstarted work, invalidate snapshot/selection and expose Disposed. |
| `shutdown()` / Drop | Idempotently join retained workers, discard queued private results and release resources. Must run off a GUI thread; may wait for an uninterruptible OS call. |

## Bounded concurrency and generations

Policy is **reject redundant requests**, not coalesce: one refresh slot and one launch slot, at most two workers total. A completed worker still occupies its slot until its event is received and joined. Running cancellation does not free a slot early; otherwise a blocked old scan could coexist with unbounded replacements. Pending cancellation removes work immediately without a service call.

The bound is per Consumer. A host should share one owner for its catalog operations rather than create one consumer per surface; no process-wide singleton or global locking is introduced.

Refresh and launch may overlap intentionally. Each has its own status and ID. A refresh result cannot overwrite launch state or vice versa. There is no unbounded queue, timer, background subscription, automatic retry or history buffer. The private completion channel has capacity two: each occupied worker sends once, and no additional worker starts in that lane until receipt/join. This capacity also allows shutdown to join without a worker being stuck waiting for an undrained completion queue.

IDs use checked u64 addition and begin at 1. They are monotonically increasing across both lanes for this Consumer; refresh generations therefore need not be contiguous. u64::MAX is valid once. Every later request returns IdExhausted without changing the snapshot, selection, terminal state or counter. Never wrap or reset a live owner. A fresh owner has a separate private channel and is created only after the old owner has been disposed/joined.

The consumer accepts a completion only if its lane still owns that exact ID and a running handle. Others produce a stale-discard notice, leaving the current snapshot/status intact. Legitimate refresh completion reordering is prevented by the serial policy; tests additionally inject replayed/out-of-order completion messages to prove the defensive ID guard. Those injections are state-boundary tests, not claimed native asynchronous races.

## Selection and launch rules

Successful refresh publishes a new generation and clears selection **even if the same entry identity remains**. A failed or cancelled refresh retains the last usable snapshot and its selection. A partially successful catalog publishes its entries and source issues/truncation metadata; source issues are not silently converted into total failure.

A launch intent captures the explicitly selected entry/request when request_launch succeeds. Changing or clearing selection afterwards cannot retarget that already requested operation; cancel it explicitly by ID to retract it. Before dispatch, if a new snapshot has replaced its generation, reject it as Failed(StaleSelection) without calling the service. Selection changes alone do not create work.

Once dispatched, an explicitly requested launch may finish while a refresh replaces the catalog and clears selection. Its accepted result is still reported against its original operation ID; discarding acceptance merely because the catalog changed could encourage a duplicate launch. Consumers should correlate returned IDs with the explicit commands they issued. No unbounded launch history is maintained here.

Stale files between selection and creation remain authoritative service errors. Missing/access failures retain selection while exposing Failed(Service(error)); retry or refresh requires another explicit request. The consumer does not reread shortcuts, add shell parsing, invoke another protocol handler, change cwd rules, alter privilege policy or bypass Stage 3A validation.

## State transitions

Both lanes use `Status<T>`; RefreshSummary and Accepted are different payloads.

| Trigger | Result |
| --- | --- |
| Accepted explicit request | Idle/terminal → Pending(id). |
| Successful worker creation | Pending → Running(id, cancellation_requested=false). |
| Thread-start failure | Pending → Failed(ThreadStart), slot released; no effect ran. |
| Refresh succeeds | Running → Succeeded(summary), snapshot replaced, selection cleared. |
| Service fails / caught unwind | Running → Failed(Service(error) / WorkerPanicked), old snapshot retained. |
| Pending cancel | Pending → Cancelled, work removed; no backend call. |
| Running cancel | Remains Running with cancellation_requested=true until worker receipt/join. |
| Cancelled refresh completes | Cancelled; its result never replaces the snapshot, even if the backend returned success. |
| Launch confirms pre-creation cancellation | Cancelled. |
| Launch reports accepted creation after cancellation request | Succeeded(PID). The accepted application is not killed or mislabelled cancelled. |
| Obsolete completion | Stale-discard notice; newer snapshot and statuses unchanged. |
| Disposal | Both lanes → Disposed, snapshot/selection removed; no subsequent completion can publish state. |

Catch_unwind converts service unwinds to WorkerPanicked under unwind builds. It cannot catch aborts, native fatal errors or a stuck OS call. A panicked custom launch backend might have accepted a process before failing; WorkerPanicked is not permission for automatic retry or proof that no application started.

### Worker lifecycle repair and re-acceptance

The independent acceptance review found that dropping a caught panic payload could itself panic before the completion send, leaving an exited worker's slot Running. The [updated acceptance report](STAGE-3B-ACCEPTANCE-REPORT.md) and [fresh evidence](evidence-stage3b-lifecycle-fix/README.md) supersede the original test counts below; the original FAIL evidence is retained.

Every dispatched worker now installs a `TerminalPublisher` before entering service/payload code. It owns one fallback WorkerPanicked completion. Normal publication replaces that fallback with the actual result and consumes the guard; return or Rust unwind otherwise drops the guard and sends the fallback. Exactly one guard is created per occupied slot and Option::take sends at most once. No second operation-state model, timer, polling loop, reaper thread or extra channel is introduced. With at most two occupied slots, the existing two-event queue permits both sends even while shutdown joins without receiving.

This lifetime protocol makes completion independent of the normal catch/convert/send continuation: a payload-disposal unwind runs the guard before worker exit. The owner then joins and applies the matching ID once; duplicate/obsolete events remain stale. An accepted launch result already published remains authoritative even if later worker cleanup panics. A missing result means WorkerPanicked with an unknown external side-effect outcome, never permission to retry automatically. Refresh cancellation and Disposed state retain their existing precedence.

`join_worker` disposes an unsuccessful join's panic payload outside unwinding. It narrowly catches disposal panics and disposes each replacement payload until cleanup returns, avoiding owner unwind and payload leaks for finite nested cleanup failures. No panic hook is suppressed. A nonterminating destructor or infinitely generated panic-payload chain is uninterruptible code, and can prevent joining just as a nonreturning service can. Process abort, panic=abort, panic-hook failure, double panic during unwinding and forced termination cannot provide in-process terminal guarantees. No Rust code or timeout can safely preempt those cases; they are outside this recoverable-unwind contract.

`wait()` still blocks on the existing completion channel on a headless thread; it now receives the guard fallback when normal publication is skipped. Joining/Drop can still wait for blocked external calls or thread teardown. A future adapter needs a host wake/command-delivery design and must keep these blocking paths off the GUI thread. The existing production accessibility gate remains closed.

## Shutdown and lock review

Workers own only backend/token/request/channel clones; the mutable model is never shared. Enumeration and process creation occur before sending completion. Receiving/joining holds no shared model lock. New consumer/runtime code adds **no unsafe blocks**; existing native integration remains in its original module. The Windows test's Child mutex is held only to transfer ownership, never during process creation or waiting.

begin_shutdown freezes the public state and rejects all commands/completion-drain APIs as Closed. Workers may finish and queue private results during shutdown; these are discarded, never applied or externally notified. Pending work is removed, running tokens are cancelled and handles remain owned until shutdown/Drop joins them. Tests release blocked services via channel gates and verify both lanes join with no late publication or detached backend references.

There is deliberately no detached reaper or helper service. If Windows blocks in filesystem/COM/CreateProcess, cancellation cannot preempt it; shutdown/Drop can wait indefinitely for that external call. This is an existing Stage 3A limitation, not a promise of bounded shutdown latency. Accepted applications may continue through disposal; they are not leaked consumer workers. Forced termination and hostile backends remain outside this qualification.

## Actual evidence and resource comparison

See [test matrix](STAGE-3B-TEST-MATRIX.md) and [new evidence](evidence-stage3b/README.md). On Windows NT 10.0.26300.0, x64/MSVC, Rust 1.99.0: **20 deterministic consumer tests** passed; the new opt-in Windows consumer test passed against an owned fixture; all **39 opt-in-inclusive sei-apps tests** passed, including preserved Stage 3A acceptance cases. All workspace quality gates passed. Prior Stage 3A acceptance and accessibility artifacts remain intact.

Read-only probes were run consecutively on the same host/release toolchain, externally sampled at 50 ms during two explicit refreshes plus a two-second idle hold:

| Probe | Refresh times | Sampled max working set / private commit | Last sampled CPU / observation |
| --- | --- | --- | --- |
| Stage 3A service, freshly rerun | 230.725 / 167.575 ms | 19,329,024 / 3,829,760 bytes | 0.390625 s / 2.5912 s |
| Stage 3B consumer | 215.261 / 166.106 ms | 19,935,232 / 3,391,488 bytes | 0.375 s / 2.4763 s |

Both found 78 entries / 252 visited nodes / 83 reported issues, and each pair of refreshes returned identical entries. No real installed app was selected or launched by either probe. Consumer inline size was **600 bytes** on this build; that excludes heap/channel/catalog storage. Sampled working-set difference was 606,208 bytes, while private commit was lower. These noisy process-level samples include DLLs, catalog data and benchmark comparison copies; they do not isolate fixed consumer overhead, establish cold-cache behavior, prove leaks absent, or promise improved performance.

## Next gate and scope

Recommend an independent Stage 3B acceptance review before host integration. After closing the production launcher's accessibility gate, the smallest UI integration would map explicit host commands to these APIs and marshal completion notices from a headless owner without blocking the GUI. Do not add new discovery sources, shell hooks, AppBar, global Windows-key interception or production launcher widgets as part of this headless milestone.

All Stage 3B changes remain uncommitted on main. The previously uncommitted Stage 3A acceptance edits/evidence are preserved, not absorbed into an automatic commit.
