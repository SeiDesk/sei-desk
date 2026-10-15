# Stage 3A — Test matrix

15 October 2026. **PASS for the implemented headless subset** on Windows NT 10.0.26300.0, x64/MSVC, Rust/Cargo 1.99.0. Declared Rust 1.88 and other Windows 10/11 builds were not separately qualified. No production launcher UI gate is changed.

| Test | Status | Evidence type / result |
| --- | --- | --- |
| Normalize display names/path keys, stable launch identity | Passed | Injected shortcut fixtures + real filesystem metadata; equivalent effective cwd identities match, argument case stays distinct. |
| Deterministic deduplication and ordering/source precedence | Passed | Repeated fixture discovery and reversed source input; user source wins. |
| Malformed, stale, missing and denied entries | Passed | Missing/stale filesystem targets are real; malformed/denied LinkReader results are injected. Other valid entries survive. |
| Missing source and access-denied mapping | Passed | Missing source real; denial injected as OS error 5. No ACL changes or actual restricted directory qualification. |
| Node/depth/shortcut-size bounds | Passed | Fixture tests; overflowing directory discarded before parsing its arbitrary prefix. |
| Cancellation before/during refresh, before launch | Passed | Injected reader triggers real cancellation token; cancelled launch creates no output. |
| Progress channel backpressure | Passed | Zero-capacity sender cannot block refresh; final catalog remains authoritative. |
| Unsupported URI/packaged-style/network/script targets | Passed | Policy unit tests; no real protocol or packaged app activation attempted. |
| Real `.lnk` read, arguments and malformed COM Load | Passed | Actual Windows COM files, never executing installed targets. |
| Actual shortcut-derived request and raw argument/cwd round-trip | Passed | Opt-in native fixture test; local test-built child only. |
| Alternate-user/run-as shortcut flag | Passed | Real COM flag persisted/read and rejected; no alternate-user launch attempted. |
| Actual structured argument quoting and Unicode | Passed | Opt-in CreateProcess fixture test, child records received argv/cwd. |
| Cancellation after accepted creation | Passed | Child exits normally and writes expected output after token cancellation. |
| Actual invalid PE spawn error | Passed | CreateProcess returned 216; mapped Invalid. Initial test's assumption of 193 only was corrected, not hidden. |
| Other launch-error mappings | Passed | Injected Windows codes 2,3,5,740,193,216,8; operation/code preserved. |
| Read-only installed Start Menu discovery | Passed | 78 entries / 252 nodes / 83 filtered issues; two refreshes identical, no discovered target launched. |
| Resource baseline | Passed — measurement completed | Release probe, 50-ms process sampling, two explicit refreshes plus two-second idle hold. Not a performance guarantee. |
| Workspace fmt/check/test/strict Clippy/release | Passed | Final verified run: 25 ordinary workspace tests, two opt-in tests separately enabled; sei-apps opt-in-inclusive total 16. |
| Documentation links/whitespace, final scope/preservation | Passed | Preservation evidence and final review log. |
| Actual inaccessible ACL source, UAC/elevation denial, mapped/redirected drive outage | Not run | No ACL or persistent settings changed; do not substitute injected errors for real Windows behavior. |
| Packaged/URI/App Paths discovery and launch, richer shortcut semantics | Not run / outside implemented subset | Research only; explicit exclusions documented. |
| Long-duration leaks, hostile files/TOCTOU, Unicode case-sensitive filesystem aliases, hard OS-call timeout | Not run | Current finite/count-bounded design does not prove these properties. |
| Narrator, UIA text ranges, production UI focus/navigation | Not run in Stage 3A | Headless scope; previous evidence preserved. Production accessibility gate remains. |

## Reproduce

From repository root on Windows, use a **new** evidence directory:

```powershell
powershell.exe -NoProfile -File tools/Test-Stage3A.ps1 -OutputDirectory docs/implementation/evidence-stage3a/new-run
```

The runner checks formatting, all workspace targets, workspace tests, strict all-target Clippy, release workspace/examples, and diff whitespace. It then points a process-local fixture variable at this workspace's built fixture and enables only sei-apps' ignored tests. Tests start only that bounded fixture, wait for normal exit, kill/wait their own child if its ten-second deadline expires, and clean owned temporary files. No installed application launch is authorized by the runner.

The count-only catalog probe performs two explicit refreshes, compares returned entries, reports timing/counts and then holds for two seconds without polling the catalog. The external PowerShell observer samples process counters every 50 ms. A 30-second observer timeout terminates only its owned probe and fails the run. The service itself does not acquire this hard timeout capability. CPU and memory counters include startup, COM work, catalog data and the idle hold, not just steady-state service overhead.

For individual commands:

```powershell
cargo fmt --all --check
cargo check --workspace --all-targets --locked --offline
cargo test --workspace --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo build --workspace --release --locked --offline
cargo build -p sei-apps --release --examples --locked --offline
$env:SEI_APPS_FIXTURE = (Resolve-Path target/release/examples/launch_fixture.exe).Path
cargo test -p sei-apps --locked --offline -- --include-ignored
git diff --check
```

Offline commands assume the locked dependencies are cached. Set the fixture environment variable only in the test process; do not configure startup or persistent environment settings. Ordinary cargo test deliberately does not launch even the fixture unless explicitly enabled. The runner temporarily disables incremental compilation in its process because this workspace filesystem reported unsupported hard links; this changes no production setting.

See [evidence index](evidence-stage3a/README.md), [catalog source policy](STAGE-3A-APPLICATION-CATALOG.md), and [launch semantics](STAGE-3A-LAUNCH-SERVICE.md).
