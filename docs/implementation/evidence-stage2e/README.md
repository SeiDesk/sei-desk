# Stage 2E capability investigation evidence

2026-10-15. [Report](../STAGE-2E-TEXT-INPUT-CAPABILITY-REPORT.md): BLOCKED for complete text-range integration. Earlier reports/evidence and existing application/experiment source remain unchanged.

## Sources, identity and results

- `preservation-before.json`, `git-status-before.txt`, `head.txt`: pre-probe input state, including all uncommitted user work.
- `inspected-source-hashes.json`, `source-review.txt`: pinned cached released source and exact relevant lines. Source feasibility is separate from UIA execution.
- `public-api-probe.rs`: evidence-local, non-window tests using actual pinned iced State/Value types. Typed Tree getter compiles; no live-widget readback implementation is claimed.
- `api-probe-build.txt`: first direct rustc link failed (missing existing native import-library search path). Retained failure, not an API failure or passing build.
- `api-probe-build-final.txt`, `api-probe-tests.txt`: corrected existing-library link; three tests passed. Real caret/selection type methods and Unicode prefix mapping, not provider/keyboard proof.
- `private-paragraph-probe.rs`, `private-paragraph-result.txt`: deliberate negative compile check; E0616/exit 1 is the expected observed rejection of State.value access, not a successful paragraph adapter.
- `probe-identities.json`: source/manifest/lock and executable hashes, OS/toolchain. API binary artifacts are in the existing ignored experiment target; failed-link intermediates generated in this new evidence folder were removed as listed in `owned-link-artifact-cleanup.json`. No pre-existing file was removed.
- `quality-gates.txt`: fresh unchanged experiment fmt/check/13 tests/strict all-target Clippy/release build, all exit 0.
- `uia-client.txt`, `uia-baseline/`: fresh existing 29-check Windows client, all passed; owned PID/HWND, raw/control trees, actions, stale-element exceptions and graceful detach. Two-second idle sample is not a new 30-second resource benchmark.
- `Inspect-TextPatterns.ps1`, `pattern-client.txt`, `pattern-observation/`: separate actual owned Windows UIA query. ValuePattern=true, TextPattern=false, LabeledBy=false; observation script exit 0 does not mean accessibility passed. Native WindowPattern close/exit 0/no child.
- `final-review.json`: preservation, documentation/whitespace/parser checks, source/probe hashes, remaining owned processes and git diff check.

## Reproduce

From repository root with cached locked dependencies:

```powershell
cargo fmt --manifest-path experiments/accesskit-iced-spike/Cargo.toml --check
cargo check --manifest-path experiments/accesskit-iced-spike/Cargo.toml --locked --offline
cargo test --manifest-path experiments/accesskit-iced-spike/Cargo.toml --locked --offline
cargo clippy --manifest-path experiments/accesskit-iced-spike/Cargo.toml --all-targets --locked --offline -- -D warnings
cargo build --manifest-path experiments/accesskit-iced-spike/Cargo.toml --release --locked --offline
powershell.exe -NoProfile -File experiments/accesskit-iced-spike/Test-UIA.ps1 -OutputDirectory docs/implementation/evidence-stage2e/another-uia-run -IdleSeconds 2
powershell.exe -NoProfile -File docs/implementation/evidence-stage2e/Inspect-TextPatterns.ps1 -OutputDirectory docs/implementation/evidence-stage2e/another-pattern-run
```

Direct API probes reuse existing compiled iced and native import libraries; no manifest/dependency changes. Resolve the iced rlib from the experiment's debug/deps (currently `libiced-5752bf559ff9ed40.rlib`) and the installed `windows_x86_64_msvc-0.52.6/lib` directory. If multiple cached iced variants exist, use the artifact matching the pinned manifest/features rather than choosing another graph.

```powershell
$probeDeps = (Resolve-Path experiments/accesskit-iced-spike/target/debug/deps).Path
$probeIced = Join-Path $probeDeps 'libiced-5752bf559ff9ed40.rlib'
$probeNative = Join-Path $env:USERPROFILE '.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/windows_x86_64_msvc-0.52.6/lib'
rustc --edition=2024 --test docs/implementation/evidence-stage2e/public-api-probe.rs --extern "iced=$probeIced" -L "dependency=$probeDeps" -L "native=$probeNative" -o experiments/accesskit-iced-spike/target/debug/text-api-probe.exe
./experiments/accesskit-iced-spike/target/debug/text-api-probe.exe
rustc --edition=2024 --crate-type lib --emit metadata docs/implementation/evidence-stage2e/private-paragraph-probe.rs --extern "iced=$probeIced" -L "dependency=$probeDeps" -o experiments/accesskit-iced-spike/target/debug/private-paragraph-probe.rmeta
```

The last command MUST fail with E0616 for the inspected private-field boundary; do not describe that exit as a successful build. API probes do not create a window, synthesize text semantics or claim UIA ranges. Fresh physical keyboard, Narrator, actual text-range operations, independent live-region events and complex-input/display qualification were not performed. Previous user attestation is historical and not reused as new input/speech evidence. No commit or push.
