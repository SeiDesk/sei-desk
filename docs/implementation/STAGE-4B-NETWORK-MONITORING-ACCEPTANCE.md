# Stage 4B — Network monitoring acceptance

17 October 2026, Windows x64/MSVC, Rust/Cargo 1.99.0. Starting and final branch: `main`. Starting and final HEAD: `f9f334bfb65bb57a6afc2047d47baad4738df52e`. No commit or push.

## Preflight and preservation

Read the Stage 4A acceptance report, recorded the complete tracked Stage 4A diff, inspected its untracked source, monitoring/freshness/shutdown paths, view and tests, and checked the locked Windows/iced sources before editing. The starting tree was the expected uncommitted Stage 4A implementation:

```text
 M README.md
 M crates/sei-desk/src/main.rs
 M crates/sei-desk/src/view.rs
 M crates/sei-windows/src/lib.rs
 M tools/Test-Stage2A.ps1
?? crates/sei-desk/src/monitor.rs
?? crates/sei-desk/src/validation.rs
?? crates/sei-desk/src/validation_view.rs
?? crates/sei-windows/src/metrics.rs
?? docs/implementation/STAGE-4A-DASHBOARD-ACCEPTANCE.md
```

Preflight hashes and the starting diff/status were saved under ignored `target/stage4b-*`. Hash checks confirm that all manifests and Cargo.lock, the Stage 4A report, original metrics.rs, validation.rs/validation_view.rs/model.rs, and tools/Test-Stage2A.ps1 are byte-for-byte unchanged from this turn's starting tree. Stage 3A/3B source/tests/contracts, design assets, historical evidence and experiments were not modified. No reset, restore, stash, cleanup or deletion was used.

## Implementation and files

| File | Stage 4B change |
| --- | --- |
| `crates/sei-windows/src/network.rs` | Typed counters/status/identity, pure eligibility, aggregation and monotonic-time tracker. |
| `crates/sei-windows/src/network/native.rs` | Owned read-only IP Helper table; isolated SDK-compatible FFI and native layout/query test. |
| `crates/sei-windows/src/network/tests.rs` | Nine counter, identity, policy, timing and failure tests. |
| `crates/sei-windows/src/lib.rs` | Export network module; original Stage 4A metrics export retained. |
| `crates/sei-desk/src/monitor.rs` | Query network in the existing sampler, publish a boxed typed reading in the bounded update, reset its baseline after sampler panic; backpressure test. |
| `crates/sei-desk/src/main.rs` | Retain latest network reading and bounded history, expire history on the existing freshness pulse, reject out-of-order sample publication; focus/state regression test. |
| `crates/sei-desk/src/traffic.rs`, `traffic/tests.rs` | History retention, formatting and normalized time-based chart geometry; six tests. |
| `crates/sei-desk/src/traffic_chart.rs` | Small iced advanced quad-renderer widget for paired receive/transmit bars. |
| `crates/sei-desk/src/view.rs` | Replace unavailable network card with separate byte rates/status; add responsive traffic history, legend, shared scale and empty state. Other cards, navigation and Applications remain. |
| `README.md` | Update only the introductory feature description/report links. |
| This report | Actual results and limits. |

There are no manifest, lockfile or dependency-feature changes. The locked windows-sys 0.61.2 source contains GetIfTable2/FreeMibTable and MIB_IF_ROW2, but IP Helper/Ndis features are not enabled. This follows Stage 4A's established narrow FFI convention instead of changing dependencies: exact repr(C) SDK fields, scoped borrows, a FreeMibTable RAII owner and safety comments. The native test validates 1352-byte row stride and the table/operational/counter offsets (8/1156/1208 bytes), then successfully queries this Windows host. Returned rows are copied to owned Rust data before native memory is freed. At most 256 rows are accepted; larger tables fail explicitly, with memory freed. The API's own allocation occurs before that count check.

GetIfTable2 returns 64-bit InOctets/OutOctets. Each is used once; subcategory counters are not added again. Return codes are checked directly. Sources: [GetIfTable2](https://learn.microsoft.com/en-us/windows/win32/api/netioapi/nf-netioapi-getiftable2), [MIB_IF_ROW2](https://learn.microsoft.com/en-us/windows/win32/api/netioapi/ns-netioapi-mib_if_row2).

## Policy, rates and history

- Include only Ethernet (type 6) and Wi-Fi (71) marked hardware, operationally up, administratively up and media-connected. Exclude filter, unauthenticated, disconnected, paused, low-power and endpoint flags; software, loopback, tunnels, virtual adapters and other interface types are excluded. This deliberately omits WWAN, VPN logical counters and virtual-switch host interfaces rather than combining stacked views.
- Sum eligible physical-link octets, independently for receive/transmit. The identity is InterfaceGuid + InterfaceLuid; interface index is never used as a persistent key. Duplicate GUID **or** LUID, zero identities and over-limit tables fail closed instead of counting an adapter twice. Ordering of rows does not affect the aggregate.
- This is aggregate physical-link traffic, not unique application payload or internet usage. Driver hardware/status reporting is trusted. Bridged/forwarded traffic crossing two distinct physical links counts on each link; no packet-level deduplication or connectivity guarantee is claimed.
- The first observation establishes a baseline without a rate. Any eligible adapter addition/removal, GUID/LUID change or reconnect rebaselines the whole aggregate, avoiding a partially measured total. No-adapter state is unavailable. Query failures drop the baseline, retain an explicit native error and require a new baseline on recovery.
- Compute checked unsigned deltas / actual monotonic elapsed seconds. A decrease in either direction, including potential u64 wrap, invalidates that aggregate interval; no modulo-based wrap guess or negative rate is shown. The new counters become the next baseline. Summation uses u128 before conversion to finite byte rates.
- Intervals below 100 ms or above 10 s produce a baseline state, not a rate. Non-increasing timestamps do not rewind the tracker; UI sample timestamps also reject old/replayed updates. Freshness uses the Stage 4A five-second threshold: stale rates are suppressed with a paused indication, never substituted with zero.
- The existing worker queries counters after the ordinary metrics, approximately every two seconds plus query time, with no overlap. Its timestamp is captured after the table query. No timer, thread, executor or global state was added. The independent one-second clock/freshness worker remains. Drivers/table reads are not an atomic snapshot across adapters; timestamps approximate the common observation endpoint.
- History holds only delivered finite, nonnegative Live receive/transmit pairs: at most **32 points**, each no older than **60 seconds**. Cohort changes clear old history so different adapter sets are not plotted as one series. Failed/baseline observations add nothing; retained same-cohort points age out normally. Duplicate/replayed points cannot enter history. Bounded-publication drops appear as missing endpoints, not fabricated samples.
- Bars represent interval averages at their actual monotonic endpoint positions within a real 60-second axis. There is no interpolation between observations. Irregular spacing and missing intervals remain visible. Receive is blue, transmit muted lavender; both use one dynamic 0-to-peak scale (minimum full scale 1 B/s). A true zero has a baseline mark. Empty and single-point histories are supported. Values use decimal B/s, KB/s, MB/s and GB/s, consistent with the dashboard's decimal capacity units.

The pre-existing channel remains capacity three plus reservations for two fixed senders (at most five accepted undrained messages in the test). try_send does not wait on a full channel. The network payload is boxed to keep small pulse/navigation messages small. All native work stays on the existing sampler. Both worker handles remain owned, signaled and joined after the iced event loop exits. An in-progress native call must finish; forced cancellation and hard shutdown deadlines are not promised. Sampling still runs while minimized, as in Stage 4A.

## Executed validation

Process-local CARGO_INCREMENTAL=0 avoids this drive's unsupported incremental hard links. No warnings/tests were suppressed or weakened.

| Command/check | Actual result |
| --- | --- |
| `cargo fmt --all --check` | Final pass, exit 0. |
| `cargo check --workspace --all-targets --locked --offline` | Final pass, exit 0. |
| `cargo test --workspace --locked --offline` | Final pass: **80 passed, 5 intentionally ignored**. 45 ordinary service + 19 app + 16 Windows-library tests. |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Final pass, exit 0. Initial failure identified enlarged enum variants and a collapsible condition; payload boxing/let-chain fixed both without lint allowances. |
| `cargo build --workspace --release --locked --offline` | Final pass, exit 0. |
| `cargo build -p sei-apps --release --examples --locked --offline` | Pass, exit 0. |
| `cargo build -p sei-windows --release --example stage2a_probe --locked --offline` | Pass, exit 0. |
| `cargo test -p sei-apps --locked --offline -- --include-ignored` with SEI_APPS_FIXTURE set to this checkout's release launch_fixture.exe, outside filesystem sandbox | **50 passed**: 47 library + 1 consumer + 2 launch integration tests; zero failures/ignored. These overlap ordinary service tests. |
| Actual dashboard `target/release/sei-desk.exe --diagnostics` | Windows rendering, changing real rates, genuine growing/expiring history, navigation, minimum-size scrolling and normal close verified. Initial full session and final rebuilt executable both ran. |
| Actual harness `target/release/sei-desk.exe --validation --diagnostics --error-fixture` | Seeded error rendered; Escape dismissed it; native close exited 0. Diagnostic graceful exit recorded restoration=Denied, not a focus-restoration pass. All nine retained harness unit tests also passed. |
| `git diff --check`, protected-file hashes and final diff review | Pass. No service/dependency/approved-design edits. |

The **18 added tests** cover first baseline, actual fractional elapsed rates, genuine zero traffic, receive/transmit resets and u64 wrap rejection, multi-link sums, ordering, addition/removal/GUID-LUID changes, duplicate/invalid IDs, special/down interfaces, failed/missing data and reconnect, stale and non-increasing times, short/long intervals, limits, capacity/time eviction, empty/single/constant/spike/irregular chart data, invalid formatting values, bounded publication, stale UI result rejection/focus preservation and real native ABI/query behavior. Existing tests remain intact. No failed unit/integration test remains. The full legacy PowerShell UI Automation observer was not rerun; the retained harness was built, unit-tested and exercised through the same Computer Use path as the dashboard.

## Windows observations and remaining limits

On this host one physical adapter was eligible. The running application repeatedly reported approximately 1.0–1.3 MB/s receive with changing smaller transmit rates, sometimes over 100 KB/s. These came from ambient real traffic, not generated fixtures or a load test. History grew from its empty/baseline state to about 30 measured intervals and expired old samples on the 60-second axis. CPU, memory, system-drive, battery and independent clock continued updating.

Observed 1100-DIP and 760-DIP client widths and a 760 × 520 client window. PgDown exposed the complete chart, endpoints, wrapped legend/explanation and Power card; Tab/Enter opened the unchanged Applications placeholder. Normal native close exited 0 and logged dashboard_workers_joined. Runtime logs remain under ignored target/stage4b-runtime.log, stage4b-final-runtime.log and stage4b-validation-runtime.log. Selected application by its exact workspace path. Chat visualization captures include stage4b-overview/history/compact/minimum-history/applications.jpg.

The independent visual finish review returned **ship for bounded Stage 4B** with no material findings in the five captures. Its read-only documenter comparison confirmed preserved design tokens; transmit adds muted lavender #725477 alongside existing receive blue #44638A. No approved design files/sidecars were changed.

Not physically induced: adapter disconnect/replacement, multiple simultaneous hardware links, counter resets/wraps, native permission/error/hang conditions or queued UI stalls; those boundaries have pure/injected tests rather than hardware acceptance. Other Windows versions/architectures/drivers, full DPI/high-contrast/locale matrix, long-duration leak/performance qualification and atomic cross-adapter counters remain unverified. Hardware flags cannot prove every unusual driver or bridge topology is distinct. Throughput alone is not network health or internet connectivity.

The existing UIA/assistive-technology gate remains unresolved; visible local keyboard focus and headless tests do not certify it. No accessibility framework, provider fork, global hotkey/input hook, Windows-key interception, AppBar/taskbar/Start/tray replacement, Explorer modification, privileged operation or optimization action was introduced. Applications remains a placeholder. Disk activity remains explicitly unmeasured.

## Final Git status

Both HEAD hashes and branch are recorded at the top. The final status includes all original Stage 4A changes plus this extension; no commit or push:

```text
 M README.md
 M crates/sei-desk/src/main.rs
 M crates/sei-desk/src/view.rs
 M crates/sei-windows/src/lib.rs
 M tools/Test-Stage2A.ps1
?? crates/sei-desk/src/monitor.rs
?? crates/sei-desk/src/traffic.rs
?? crates/sei-desk/src/traffic/
?? crates/sei-desk/src/traffic_chart.rs
?? crates/sei-desk/src/validation.rs
?? crates/sei-desk/src/validation_view.rs
?? crates/sei-windows/src/metrics.rs
?? crates/sei-windows/src/network.rs
?? crates/sei-windows/src/network/
?? docs/implementation/STAGE-4A-DASHBOARD-ACCEPTANCE.md
?? docs/implementation/STAGE-4B-NETWORK-MONITORING-ACCEPTANCE.md
```
