# Isolated AccessKit + iced Windows spike

One ordinary validation window, not the Sei-Desk shell. Its own `[workspace]` and Cargo.lock keep dependencies out of the root application. Requires Windows x64/MSVC, Rust 1.88+ (tested 1.99.0), Windows SDK/linker and an ordinary user desktop. No UAC, Explorer/taskbar changes, registry writes, hooks, tray host or startup service. Keep the native taskbar available.

Pinned released components: iced 0.14.0, accesskit 0.25.1, accesskit_windows 0.35.1, windows-sys 0.61.2. See the [technical report](../../docs/implementation/STAGE-2C-ACCESSKIT-REPORT.md) for licenses, resolved versions and limitations. No fork/patch or `accesskit_winit` adapter is used.

Run from repository root:

```powershell
cargo run --manifest-path experiments/accesskit-iced-spike/Cargo.toml --locked
cargo fmt --manifest-path experiments/accesskit-iced-spike/Cargo.toml --check
cargo check --manifest-path experiments/accesskit-iced-spike/Cargo.toml --locked
cargo test --manifest-path experiments/accesskit-iced-spike/Cargo.toml --locked
cargo clippy --manifest-path experiments/accesskit-iced-spike/Cargo.toml --all-targets --locked -- -D warnings
cargo build --manifest-path experiments/accesskit-iced-spike/Cargo.toml --release --locked
powershell.exe -NoProfile -File experiments/accesskit-iced-spike/Test-UIA.ps1 -IdleSeconds 30 -OutputDirectory docs/implementation/evidence-stage2c/manual-01
```

Use a new/empty output directory every time. `--offline` can be added when dependencies are cached. The PowerShell client inspects only its owned process/window, captures only the foreground client pixels, operates actual Value/Invoke/Toggle patterns, temporarily moves/resizes its own window, measures that process, and invokes Close. Its waits are bounded (3 s state/discovery, 10 s shutdown, 1–120 s idle); a hung OS UIA call itself has no timeout guarantee. Do not leave the test unattended on an unhealthy desktop. A failed run still closes its exact child in `finally`; if that fails, only that child is killed and the failure is recorded. Manual runs have no automatic expiry: use Close, Escape after dismissing an error, or Alt+F4. Diagnostics identify the own HWND for recovery; no persistent resources are created.

Expected client: Sample text, Validate, Prefer reduced motion, inline error/Dismiss when empty validation fails, and Close. Tab/Shift+Tab traverse real controls; Enter/Space activate actions; text input owns typing. Escape dismisses error before closing. The checkbox is an in-memory demonstration; no Windows motion setting changes and nothing animates. See the [test matrix](../../docs/implementation/STAGE-2C-TEST-MATRIX.md) for pending physical keyboard/Narrator procedures. Physical keyboard behavior is not certified by a UIA Invoke test.

Architecture:

- `main.rs`: iced lifecycle, event stream and actual controls. Window starts hidden; provider attaches on its owning thread; first layout snapshot is published before show.
- `wrapper.rs`: thin forwarding widget wrapper exposing its actual layout rectangle to one iced operation. It does not duplicate drawing or input.
- `semantics.rs`: stable IDs, model-to-node mapping, conditional error identities and changed-node updates. Reuses the existing pure Stage 2A model by a source reference without editing it.
- `native.rs`: thread-local non-Send AccessKit owner, short initial-tree snapshot lock, bounded nonblocking action queue, two documented unsafe query boundaries, detach before exit. All mutable application state stays on iced's thread.
- `Test-UIA.ps1`: separate .NET Windows UIA test client, not an application frontend.

The spike exposes ValuePattern for whole-string updates, Invoke and Toggle, plus local semantic focus. **No TextPattern/caret/selection or live announcement is advertised.** The AccessKit model records LabelledBy, but the released Windows provider does not expose UIA's LabeledBy property. An explicit accessible input name is available. Queue overload drops requests with a diagnostic; no backpressure or unbounded retry service is introduced. No claim is made for rapid update ordering, every stale-action race, forced termination, mixed DPI, screen-reader editing or inaccessible applications elsewhere.

Normal close detaches the subclass on the owning thread before iced destroys the HWND. Forced termination cannot run Rust destructors: Windows destroys this process's HWND/address space, but that path was not executed for this spike. There are no external shell registrations to repair. Never treat graceful cleanup as proof of a tested forced-exit path.

Core Windows UIA operation was demonstrated; the overall accessibility gate remains open until actual Narrator and physical keyboard validation. Do not integrate this experiment into production or expand into desktop shell features on the basis of tree discovery alone.
