# Stage 6A — Dashboard Reliability Audit

2026-10-17, Asia/Calcutta. **Audit completed; no new concrete, reproducible monitoring reliability defect was established.** No application code or tests were changed. Existing edge-case tests and workspace checks pass. This result is limited to the inspected implementation and executed cases, not a guarantee against hardware/driver stalls or untested configurations.

## Baseline and repository state

Repository `X:\Project\SeiDesk\sei-desk`; starting/final branch `main`; starting/final HEAD `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`.

Complete initial status was inspected with `git status --short --untracked-files=all`. The existing five tracked modifications were `README.md`, `crates/sei-desk/src/applications/preferences.rs`, `crates/sei-desk/src/applications/preferences/tests.rs`, `crates/sei-desk/src/main.rs` and `tools/Test-Stage2A.ps1`. Existing untracked work comprised Stage 5A–5E reports and all Stage 5A/5B evidence. All were treated as the baseline to preserve. No reset, clean, restore, stash or overwrite was used.

Reviewed relevant Stage [4A](STAGE-4A-DASHBOARD-ACCEPTANCE.md), [4B](STAGE-4B-NETWORK-MONITORING-ACCEPTANCE.md), [4C](STAGE-4C-APPLICATIONS-ACCEPTANCE.md), [4D](STAGE-4D-LAUNCHER-USABILITY-ACCEPTANCE.md), [5A](STAGE-5A-STABILIZATION-ACCEPTANCE.md) and [5E](STAGE-5E-UPSTREAM-FIRST-OWNER-DECISION.md) records. Stage 5A's bounded recovery and selection-visibility fixes remain intact. Stage 5E's Rust + iced/upstream-first decision and production accessibility block remain unchanged. This is a monitoring reliability audit, not launcher development, visual redesign or accessibility qualification.

## Inspected data paths

Common path: `Monitor::start` performs native work on its sampler thread → publishes owned `Update::Sample` through the bounded channel → `monitor::stream` feeds the stable iced subscription → `App::update` reconciles samples/history → existing Overview view formats the state. A separate one-second worker supplies freshness/local-clock pulses. Native metric queries do not execute in the iced update or draw functions.

| Data / location | Actual calculation and consumer behavior |
|---|---|
| CPU — `crates/sei-windows/src/metrics.rs:15`, native `sample:170`; `crates/sei-desk/src/monitor.rs:85` | GetSystemTimes supplies 64-bit idle/kernel/user counters. Utilization is `100 × (delta(kernel + user) − delta(idle)) / delta(kernel + user)`; kernel already includes idle. Checked subtraction/addition rejects reset/overflow, zero total and idle greater than total. Sampler requires two valid counter samples and breaks the baseline on CPU query failure or caught sampler panic. The existing view labels the processor-group scope. |
| Memory — `metrics.rs:34`, native `sample:170`; `crates/sei-desk/src/view.rs:83` | GlobalMemoryStatusEx returns physical total/available bytes. Used = total − available; zero total/available greater than total is invalid. View renders decimal GB (`1e9`) and percent, or loading/unavailable with native error; no invalid percentage becomes a zero-usage reading. |
| System-drive capacity — same native sample and capacity view | GetDiskFreeSpaceExW uses the drive prefix of SystemRoot; unexpected root syntax produces an explicit error. Caller-visible total/available are paired consistently, including quotas. Disk title follows the returned root. Capacity is explicitly distinguished from disk activity; no activity rate is invented. |
| Battery — native `decode_power:252`; `view.rs:253` Overview | GetSystemPowerStatus is decoded into absent, unknown or present with optional percent/AC state. Unknown sentinel flags are handled before the absent bit; percent above 100 is unavailable. Charging/plugged-in/on-battery/unknown source are distinguished without inventing charge. |
| Clock — `metrics.rs:87`, native `clock:242`; `monitor.rs:121`, `main.rs:171` | GetLocalTime provides Windows local wall time independently of the sampler. A pulse updates the header clock and monotonic freshness/history pruning. Wall time is not used for rate division or chart positioning. |
| Network acquisition — `crates/sei-windows/src/network/native.rs:53` | GetIfTable2-owned table is freed by RAII. SDK layout/offsets are asserted by the native test. At most 256 copied interfaces are accepted; larger tables error and still free the native allocation. InOctets/OutOctets are used once, not added to their subcategories. |
| Network eligibility and rates — `network.rs:32`, `Tracker::observe:72`, aggregation near 143 | Physical Ethernet/Wi-Fi, hardware/admin/operational/connected flags, and GUID+LUID identity define the cohort. Duplicate/invalid IDs fail closed. Deltas use u64 checked subtraction, u128 aggregate totals and actual monotonic elapsed seconds; output is bytes/second. Cohort changes, reset/wrap and invalid timing rebaseline; failures break the baseline. Valid elapsed interval is 100 ms through 10 s; longer gaps do not produce a spike or a fabricated zero. |
| UI reconciliation/freshness — `crates/sei-desk/src/main.rs:142–176`, `paused:489`; `view.rs:168–232`, `overview:253` | Older/equal sample timestamps cannot replace current metrics or alter navigation. Latest snapshot, optional CPU and network status remain owned UI state. Five-second age marks retained data paused; stale network rates are hidden. Retained CPU/capacity/power values have the existing paused-data notice rather than being represented as fresh replacements. Native errors and loading states remain distinct. |
| History — `crates/sei-desk/src/traffic.rs:7–98`, `traffic_chart.rs:15` | History has a 60 s window and 32-point cap. Only finite, nonnegative live rates enter it; failed/baseline readings create gaps rather than zero samples. Cohort replacement clears incompatible history. Positions use monotonic sample age and a shared receive/transmit scale; decimal B/s, KB/s, MB/s and GB/s formatting matches rate units. Chart draws discrete paired bars, with no line interpolating across missing data. |

The sampler's two-second wait starts after its previous task completes: two seconds is nominal cadence, not a hard fixed acquisition deadline. Actual network elapsed time, rather than a presumed two-second denominator, accounts for this. Windows calls within a snapshot are sequential, not a simultaneous multi-hardware measurement.

## Findings and evidence classification

**New verified defects: none established.** No narrow fix had both a demonstrated failure and a justified correction. No speculative rewrite, extra polling, new dependency or implementation-mirroring regression test was introduced. There is therefore no Stage 6A code change/regression-test pair to report. Earlier Stage 5A defects remain fixed and are not relabeled as new work.

| Classification | Finding / supporting check | Scope or remaining gap |
|---|---|---|
| Verified by passing focused tests | CPU reset/zero/invalid counters; invalid capacities; native ABI sizes and unknown/absent battery decoding; actual host snapshot clock/capacity validity. | The actual Windows snapshot path ran in unit tests; no physical battery removal, quota change or native permission failure was induced. |
| Verified by passing focused tests | Actual IP Helper table layout/query; measured delayed interval and true zero rate; u64 reset/wrap; cohort add/remove/identity replacement; duplicates; physical-link exclusion; disconnect/reconnect/error baselines; invalid times, long gaps and oversize tables. | Pure tracker fixtures establish algorithm behavior, not a physical unplug or driver-hang scenario. |
| Verified by passing focused tests | History count/time eviction, empty/zero geometry, irregular endpoint positioning, shared scale, gaps, cohort changes, replay/nonfinite rejection and decimal formatting. | No long-running rendered chart/performance or all-display qualification was attempted. |
| Verified by passing focused tests | Bounded monitor publication under undrained two-sender load; monitor start/drop wakes sleepers and joins; out-of-order UI sample rejection and paused/navigation independence. | These do not inject an indefinitely blocked native API or prove a worst-case shutdown deadline. |
| Source-reviewed behavior | Caught panic in the native sampling closure clears CPU/network baselines and publishes failure; later sampling attempts continue. Errors in individual metric Results are retained as explicit unavailable/error state. | No new injected monitor-panic/recovery run was performed. Do not count the source review as an executed native-failure test. |
| Source-reviewed behavior | Partial monitor startup failure drops the already-created worker; owners live outside iced and are dropped after its event loop returns. History pruning uses existing pulses instead of extra timer threads. | OS thread-creation exhaustion and event-loop startup failure were not injected. |
| Existing broader regression coverage | Workspace tests pass for bounded application adapter mailboxes, cancellation, stale refreshes, blocked-service shutdown, persistence replacement and both Stage 5A fixes. | They are existing tests run for preservation confidence, not launcher changes or monitoring-driver evidence. |
| Unverified risk | A synchronous native metrics/filesystem/IP Helper call can block. Shutdown joins must wait for that call to return; one blocked sampler also delays other fields in its batch. | There is no hard interruption/deadline contract. No hang was reproduced, so no timeout thread, forced termination or process isolation was introduced. |
| Unverified risk / measurement limit | A batch has a completion timestamp, not separate per-field acquisition epochs; freshness during a prolonged intra-batch delay is not independently qualified. | No induced multi-second native-call delay. This is a measurement/coverage limit, not a newly reproduced stale-reading defect. |
| Untested configuration | Hardware removal, unsupported/very large processor configurations, mixed-DPI/minimized/occluded GUI operation and lengthy idle/AT-attached resource stability. | No claims beyond the actual native tests and existing source policies. Prior runtime observations remain historical. |
| Cosmetic suggestions | No palette, layout, typography, card or chart redesign was proposed or implemented. | Visual changes would not establish a reliability defect. |

## Worker/resource lifecycle

`monitor.rs:52–79` owns each worker JoinHandle and a wakeable stop receiver. The sampler and freshness ticker are **two fixed threads**, not a thread per tick. Publication uses the existing futures channel configured for three entries plus two sender reservations; the bounded-load test accepts at most five undrained updates. Full-channel publication drops a monitoring update instead of blocking the UI or growing an unbounded catch-up queue. Dropped intervals remain unmeasured; they do not generate replacement readings.

`Monitor::drop:135` signals both workers before their individual Drop joins. Its stop channels use an unbounded type but the owned shutdown code sends only a fixed finite set of signals, not an accumulating runtime request stream. `main.rs:625` drops/join owners only after iced returns. The existing applications owner sends stop/joins and its worker shuts down Consumer; preferences owner flushes an accepted save and joins its worker. Existing workspace tests cover these adapter cancellation/join cases without changing their implementation.

Interface maps/vectors are bounded by 256 rows, history by 32 points/60 s and chart position allocation by that history size. API-owned GetIfTable2 allocation occurs before the application's count check, as already documented; no total OS allocation bound is claimed. The one-second pulse is intentional freshness/clock work. The pure view performs no native queries, creates no worker threads and does not launch commands. No resource-soak or allocation benchmark was run, so this audit does not certify leak-free behavior.

## Commands and actual results

All commands used existing locked/offline dependencies with process-local `CARGO_INCREMENTAL=0`. Focused tests ran first under the normal restricted tool environment. Broader checks ran with normal filesystem access outside that restriction because the existing Windows atomic-replacement fixtures have a documented sandbox access limitation. No UAC/administrator launch, dependency change or test weakening was used.

| Command | Actual result |
|---|---|
| `cargo test -p sei-windows --locked --offline` | PASS, 16 tests; 0 failures/ignored. Includes real host snapshot and IP Helper table queries; doc tests 0. |
| `cargo test -p sei-desk --locked --offline monitor::tests` | PASS, 2 tests; 56 filtered out. |
| `cargo test -p sei-desk --locked --offline traffic::tests` | PASS, 6 tests; 52 filtered out. |
| `cargo test -p sei-desk --locked --offline tests::out_of_order_sample` | PASS, 1 test; 57 filtered out. |
| `cargo test -p sei-desk --locked --offline tests::paused_readings` | PASS, 1 test; 57 filtered out. |
| `cargo fmt --all --check` | PASS, exit 0; no formatting edits. |
| `cargo check --workspace --all-targets --locked --offline` | PASS, exit 0. |
| `cargo test --workspace --locked --offline` | PASS: 119 passed, 0 failed, 5 intentionally ignored; 45 service + 58 dashboard/model/worker/persistence + 16 Windows tests. Doc tests 0. |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | PASS, exit 0. |
| `git diff --check` | PASS, exit 0; separate new-report whitespace/local-link check also passed. |

The focused checks overlap the workspace run; their 26 test executions are not 26 additional unique tests. No executed check failed in Stage 6A. Git emitted pre-existing LF→CRLF working-copy notices, not whitespace-check failures. No new GUI session, release build, performance/soak test, hardware fault injection, screen-reader/IME session or five opt-in process-launch fixture tests were run. Their absence is not a pass or failure. The fixture tests require separate controlled launch setup and are not needed to validate this audit-only monitoring change scope.

## Preservation, final diff and next step

Before work, SHA256 inventory covered **726 pre-existing tracked/nonignored untracked files**. Final comparison confirms all 726 unchanged, including all application/Windows/service source, manifests/lockfiles, scripts/tests, approved designs, existing reports/evidence and Stage 5A fixes. The final tracked diff remains exactly the entry Stage 5A diff. **Exact Stage 6A addition:** `docs/implementation/STAGE-6A-DASHBOARD-RELIABILITY-REPORT.md` only. Existing working-tree changes were neither staged nor committed.

Final `git status --short`:

```text
 M README.md
 M crates/sei-desk/src/applications/preferences.rs
 M crates/sei-desk/src/applications/preferences/tests.rs
 M crates/sei-desk/src/main.rs
 M tools/Test-Stage2A.ps1
?? docs/implementation/STAGE-5A-STABILIZATION-ACCEPTANCE.md
?? docs/implementation/STAGE-5B-ACCESSIBILITY-ARCHITECTURE-DECISION.md
?? docs/implementation/STAGE-5C-OWNER-DECISION-PACKET.md
?? docs/implementation/STAGE-5D-ACCESSIBILITY-CANDIDATE-PREFLIGHT.md
?? docs/implementation/STAGE-5E-UPSTREAM-FIRST-OWNER-DECISION.md
?? docs/implementation/STAGE-6A-DASHBOARD-RELIABILITY-REPORT.md
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

Known limitations are the explicitly unverified native-hang, per-field timing, hardware/configuration and long-running resource cases above. If an actual stall or incorrect reading is reported, the next useful step is a bounded reproduction capturing affected API durations, sample times, status and shutdown outcome before choosing a remedy. No speculative instrumentation or additional stage was started here. Accessibility/release acceptance remains blocked independently of these reliability checks.

**Audit-only outcome: retain the existing implementation.** No source fix was justified by this stage's evidence. No launcher change, visual redesign, accessibility experiment, dependency change, shell/taskbar modification, commit or push occurred.
