# Stage 5A evidence

Collected 2026-10-17. Read the [acceptance report](../STAGE-5A-STABILIZATION-ACCEPTANCE.md) for interpretation and limitations. Observer success is not accessibility conformance. UI screenshots use synthetic input; there was no human Narrator/physical-keyboard/real-IME session.

## Machine observations

- `stage5a-validation-baseline`, `stage5a-validation-keyboard`, `stage5a-validation-error`: pre-edit existing validation harness runs against baseline release hash `3B0A06F91FD8736EB10AA664F44AE29AD0745ECF0BC1FFF3D2B062430CF556EA`.
- `stage5a-dashboard-uia`: same baseline binary observed by the extended existing harness, exact owned dashboard PID/type/title.
- `stage5a-final-dashboard-uia`, `stage5a-final-validation`, `stage5a-final-keyboard`, `stage5a-final-error`: final release hash `E00CE35758CB0A61516FC8AF3475503B6F6F3A88BF0AF3697EE86C74BAFBA168`; modes ran sequentially. Each contains `run.json`, bounded `uia-raw.json`, `uia-control.json` and diagnostics `app-stderr.txt`.
- `stage5a-applications-accessibility.txt`: separate Computer Use accessibility extraction on the actual baseline Applications page, containing native window chrome but no semantic client controls. It is not the same API/tree traversal as the observer.
- `stage5a-*-quality.log`, `stage5a-*-services*.log`: exact Cargo output, including the incorrectly configured final fixture run and corrected passing run. Use the corrected log for final acceptance; do not discard the failed invocation.
- `stage5a-recovery-before.log`: regression failure on original source; `stage5a-recovery-after.log` and `stage5a-repeated-recovery-after.log`: targeted green runs. The first after-log's `recovery` filter runs the distinct-copy test; the repeated-startup test was run separately. Both appear in the full final suite.
- `stage5a-final-runtime.log`: final release restricted-filesystem session; pin replacement denied, existing configuration retained. `stage5a-final-runtime-normal.log` and `stage5a-restart-runtime.log`: normal user access save/restart/launch/unpin sessions. These processes exited 0; diagnostics record `dashboard_workers_joined`.
- `tracked-changes.json`: tracked files whose SHA256 differs from the pre-edit inventory. `files-sha256.json`: hashes for evidence files other than this hash index itself.

Observer commands (run from repository root, built release/probe present):

```powershell
powershell.exe -NoProfile -File tools/Test-Stage2A.ps1 -Dashboard -IdleSeconds 5 -OutputDirectory target/stage5a-final-dashboard-uia
powershell.exe -NoProfile -File tools/Test-Stage2A.ps1 -IdleSeconds 3 -OutputDirectory target/stage5a-final-validation
powershell.exe -NoProfile -File tools/Test-Stage2A.ps1 -Keyboard -IdleSeconds 3 -OutputDirectory target/stage5a-final-keyboard
powershell.exe -NoProfile -File tools/Test-Stage2A.ps1 -ErrorFixture -IdleSeconds 3 -OutputDirectory target/stage5a-final-error
```

Baseline validation runs used the same modes with baseline directories and three-second intervals; baseline dashboard used five seconds. Build probe: `cargo build -p sei-windows --release --example stage2a_probe --locked --offline`. Synthetic keyboard delivery is verified by the app state trace, not by mere dispatch success. Baseline error screenshot capture was refused after foreground focus changed; final own-window captures succeeded. No refused/occluded image is included here.

## Inspected own-window captures

| File (all start `stage5a-`) | Observed state |
|---|---|
| `search-clear-before.jpg` | Baseline: cleared query, retained CMake row offscreen. |
| `hidden-selection-before.jpg` | Baseline: CMake details visible while its selected row is outside results viewport. |
| `search-clear-after.jpg` | Final: cleared query, retained mint CMake row visible. |
| `grid-after.jpg` | Final: same identity visible in text grid. |
| `refresh-after.jpg` | Final: generation 2, CMake still visible. |
| `write-error.jpg` | Actual restricted-filesystem denial, inline retained-pins error. |
| `pin-restart.jpg` | Fresh process/current catalog reconciles saved CMake pin. |
| `traffic.jpg` | Actual measured rates/history, 12 measured intervals at capture. |
| `maximized.jpg` | Actual 1920×1080 reflow; normal app captures are 1102×772. |
| `cmake-launched.jpg` | Newly opened CMake 4.3.4 window after explicit selected launch; closed without project edits. |

The fresh Impeccable finish reviewer inspected the before/after, grid, refresh, error, pin, traffic and maximized captures and source/specification/plate 06. Disposition: `ship` for the bounded correction, no material fixes. This is not a production accessibility/release approval. Approved assets and historical records remain intact.

A separate read-only documenter checked source, specification, plate 06, Stage 4D and list/grid/refresh/error/maximized captures. It confirmed preserved palette, native type hierarchy, navigation, internal scrolling, focus/selection distinction and explicit launch behavior. No approved design files were changed or absent engine records invented.
