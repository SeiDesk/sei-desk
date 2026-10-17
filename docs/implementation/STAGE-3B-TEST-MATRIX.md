# Stage 3B — Test matrix

15 October 2026. **PASS for headless state/concurrency scope.** Windows NT 10.0.26300.0, x64/MSVC, Rust/Cargo 1.99.0. Earlier captures are preserved and not counted as fresh execution.

Re-acceptance after worker-lifecycle repair: [report](STAGE-3B-ACCEPTANCE-REPORT.md), [new evidence](evidence-stage3b-lifecycle-fix/README.md). The original FAIL run is preserved in evidence-stage3b-acceptance. New final execution: **31 consumer tests**, **56 ordinary workspace tests passed** (5 opt-in tests enabled separately), and **50 opt-in-inclusive sei-apps tests passed** (47 library + 3 integration). Release consumer tests also passed (31). No production UI/accessibility approval is implied.

| Lifecycle re-acceptance test | Status | Evidence boundary |
| --- | --- | --- |
| Original panic-payload disposal failure | Passed | Original test retained; joined failed worker now has a fallback completion. |
| Public wait after payload-disposal panic, both lanes | Passed | Real threads and public wait; deadline is only a hang safeguard. |
| Early worker return and unwind without normal publication | Passed | Injected worker boundary uses real guard/channel/wait/finish/join; no claim of a native crash. |
| Finite nested join-payload cleanup panics | Passed | Two destructive payloads followed by ordinary panic payload; owner stays usable and disposes replacements. |
| Accepted launch followed by panic/cancel/duplicate failure | Passed | Published PID remains success; duplicate is discarded; no contradictory second event. |
| Two fallback exits during shutdown | Passed | Join both workers before draining; bounded queue does not deadlock. |
| Worker failure released from explicit service gate | Passed | Channel-controlled worker and waiter, no sleeps or lost-wakeup timing assumption. |
| Repeated cancellation, blocked fake services, ordinary launch panic | Passed | Four original acceptance tests retained, including original failure regression above. |
| Native fixtures / native discovery | Passed | Fresh unrestricted-filesystem run, non-admin Windows token; actual owned launches and read-only catalog probes. |
| First sandboxed native fixture run | Failed — environment restriction | Temporary shortcut rename denied; exact existing fixture passed outside sandbox without source changes or UAC. Failure log retained. |
| Process abort / forced termination / nonterminating destructors | Not run / outside recoverable-unwind contract | No in-process terminal or bounded join guarantee for fatal or nonreturning code. |

The following original-stage rows retain their historical evidence descriptions; the current counts and lifecycle disposition above supersede them.

| Test | Status | Evidence type / practical limit |
| --- | --- | --- |
| Refresh pending/running/cancellation overlap | Passed | Controlled service gates: redundant requests Busy, old running slot retained until completion/join. |
| Separate refresh/launch states and both completion orders | Passed | Actual worker threads with channel-controlled service replies, no sleep-based ordering. |
| Replayed/out-of-order refresh result | Passed | Synthetic private-completion injection rejects old IDs while current work/snapshot remains unchanged. Real refreshes are serialized. |
| ID overflow | Passed | Counter seeded at MAX−1 in unit test; MAX accepted once, later refresh/launch rejected without wrap/state mutation. |
| Explicit selection and unknown entry | Passed | Refresh/select produce no launch; unknown selection clears prior selection and returns error. |
| Snapshot replacement | Passed | Same-identity replacement still clears selection; pending older-generation launch fails before backend dispatch. |
| Launch intent capture | Passed | Clearing selection does not retarget an already requested launch. Explicit cancellation is separate. |
| Failed/partial refresh | Passed | Last usable snapshot/selection retained on failure; successful partial catalog exposes issues/truncation. |
| Launch failure/stale entry | Passed | Injected Missing propagation and retained selection; actual removed executable separately tested below. |
| Pending cancellation | Passed | No service call occurs, no worker creation; both lanes tested. |
| Running cancellation outcomes | Passed | Fake pre-spawn Cancelled versus accepted PID after token cancellation remain distinct; never claim process termination. |
| Stale cancellation request | Passed | Prior operation ID cannot cancel a newer worker. |
| Shutdown pending / two running / Drop | Passed | Channel gates release workers; join handles/backend references drained; Disposed state rejects commands and does not publish late results. |
| Bounded ready drain / idle behavior | Passed | Queue arranged with two real worker completions; at most two processed, no hidden dispatch. Pending-only wait returns None. |
| Service unwind | Passed | Deliberate fake panic becomes structured WorkerPanicked. Not an abort or native crash test. |
| Actual Windows consumer launch | Passed | New opt-in test: injected one-entry catalog, actual Stage 3A CreateProcess, owned child argv/cwd verification, pending cancellation, removed target Missing, explicit cleanup. Discovery in this test is a fixture, not native enumeration. |
| Actual NativeService consumer refresh | Passed | Release consumer probe uses real known folders/COM, two identical refreshes, no selection/launch, normal shutdown. |
| Existing Stage 3A/acceptance tests | Passed, freshly rerun | Real COM malformed/locked/stale links, real lock/stale/format launch errors and argument round-trips retained; injected denial mappings remain injected. |
| fmt/check/tests/strict Clippy/release/diff | Passed | Full run log. Ordinary workspace: **45 passed**, 5 opt-in tests deliberately ignored there. Separate enabled sei-apps run: **36 library + 1 consumer Windows + 2 launch integration = 39 passed**. Counts overlap. |
| Resource baseline | Passed — measured | Paired release probes, two refreshes + two-second hold, 50-ms external process sampling. Not a fixed resource budget. |
| Prior work preservation | Passed | SHA256 baseline includes uncommitted acceptance artifacts. Only lib.rs intentionally changes among pre-existing files. |
| Thread-start resource exhaustion, aborts, forever-blocked native calls | Not run | Structured spawn error implemented; no forced OOM/thread exhaustion or unsafe OS-call interruption. |
| Supported Windows build/compiler/architecture matrix, long-duration leaks | Not run | Host evidence does not qualify other Windows versions, declared Rust 1.88 or other targets. |
| UIA/Narrator/text-range behavior, production launcher integration | Not run / out of scope | Headless stage; existing accessibility spike and its gates unchanged. |

## Reproduce on Windows

From repository root, with locked dependencies cached, use a fresh directory:

```powershell
powershell.exe -NoProfile -File tools/Test-Stage3B.ps1 -OutputDirectory docs/implementation/evidence-stage3b/new-run
```

The runner delegates workspace fmt/check/test/strict Clippy/release builds and all sei-apps opt-in tests to the preserved Stage 3A runner, then measures the consumer probe. It temporarily sets process-local fixture/incremental-build variables, starts probes hidden, and uses a 30-second test-only timeout to terminate only its owned probe on failure. It does not request UAC elevation. In this agent environment a filesystem sandbox exception was used for the preserved acceptance test's owned-file rename; that is not Windows RunAs.

For state tests alone:

```powershell
cargo test -p sei-apps --locked --offline consumer::
```

Controlled fake services announce arrival and wait on channel replies. Watchdog receive timeouts fail a broken test; they do not establish ordering or simulate Windows latency. No timing sleeps are used by consumer unit tests. Old-generation replay injection is explicitly separate from worker schedule tests.

For the real fixture integration alone:

```powershell
cargo build -p sei-apps --release --example launch_fixture --locked --offline
$env:SEI_APPS_FIXTURE = (Resolve-Path target/release/examples/launch_fixture.exe).Path
cargo test -p sei-apps --locked --offline --test consumer_windows -- --ignored
```

It copies only the freshly built fixture into an owned temporary directory. A test-only backend keeps the actual Child handle, waits at most ten seconds and cleans up that exact child on timeout. Its mutex transfers ownership only; creation/waiting occur outside the lock. No arbitrary discovered target is launched. Fresh text logs may have terminal empty lines normalized for Git packaging; no historical evidence bytes or results are edited.

See [architecture](STAGE-3B-CONSUMER-ARCHITECTURE.md) and [evidence index](evidence-stage3b/README.md).
