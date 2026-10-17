# Isolated iced TextInput / Shell IME observer (Stage 2G)

Two stock TextInputs, wrapped only to observe public child Tree state, layout/viewport and the shared Shell before/after update. No replacement input, AccessKit provider, TextPattern, native hooks or reconstructed shaping. See [report](../../docs/implementation/STAGE-2G-IME-OBSERVATION-REPORT.md).

There is intentionally no new Cargo manifest/lock: Build.ps1 links the exact cached iced artifact from the unchanged accepted spike's locked build. This keeps dependency pins unchanged. If that artifact is missing, first build the accepted spike with `cargo build --manifest-path experiments/accesskit-iced-spike/Cargo.toml --locked --offline`; verify its graph/features and artifact identity before adjusting the hard-coded rlib filename for a different toolchain. Do not silently select an arbitrary cached variant. This is a host-specific experiment, not a production build architecture.

From repository root, choose fresh evidence paths:

```powershell
./experiments/iced-ime-observer/Build.ps1 -EvidenceDirectory docs/implementation/evidence-stage2g/new-build
./experiments/iced-ime-observer/Run.ps1 -EvidenceDirectory docs/implementation/evidence-stage2g/new-run
```

Build runs rustfmt, rustc type-check/compile through test build, four deterministic tests, strict clippy-driver on that target, and an optimized harness binary. Dependencies are cached debug artifacts; do not describe this as a fully release-optimized Cargo dependency graph. Outputs are isolated in ignored target/. Run owns one exact process, captures logs and enforces a 600-second outer lifetime. Normal close is F10 or titlebar close; timeout/error cleanup uses the existing owned-probe helper. No process persists after runner completion.

Controls:

- F1/F2 focus A/B through public widget operations.
- F3/F4 select A (1,4)/(4,1); F5 move A to grapheme 1.
- F6 writes a known Unicode model fixture; this is not typing or OS IME.
- F7 changes real window width 740/390 logical units.
- F8 hides, queries actual mode, waits 400 ms on a finite executor task, then shows/queries mode. No continuous polling.
- F9 changes layout padding; F10 closes.
- F11 references the actual previous sample's harness epoch after a new command epoch. This is a deliberate stale-delivery probe, not native event reordering.
- F12 writes a bounded long fixture and moves to end for viewport/scroll observation. No scroll offset is guessed.

Type and navigate normally for actual keyboard cases. Synthetic keys, public operations/model fixtures, physical input and real OS composition must be labeled separately. Never inject a fabricated InputMethod event as OS IME evidence. Logs contain only this window's test values; use nonpersonal fixtures.

`model_epoch` is the harness App::update counter, **not** a stock iced layout/selection generation. `accepted_model_epoch` checks only that counter. Typed cursor getter is clamped against the supplied model Value; non-redraw samples mark `value_lag_possible=true`. The after_child_redraw phase is after a RedrawRequested update callback, not after painting. Changed-payload deduplication does not retain every frame. Redraw observations do not establish an atomic layout/composition snapshot. `layout_generation` remains UNAVAILABLE. Focus is State::is_focused (local widget state), not a foreground ownership guarantee.

Rectangles are public iced window-layout logical units `[x,y,width,height]`. Passed viewport and text-child bounds are not an effective glyph clip or UIA screen rectangle. The shared Shell may already contain a sibling request. CHILD_ENABLED means Disabled→Enabled inside this exact stock child's update; AMBIGUOUS_INHERITED_SHARED_SHELL deliberately makes no new-child attribution. No local Shell reset/merge workaround is introduced.

Capture is capped at 2048 observation rows. Final version deduplicates unchanged per-child/epoch payloads and logs elapsed microseconds separately; run 01 predates that logging improvement and reached the cap. App command/lifecycle records remain separate. Log writes are synchronous instrumentation and can perturb timing; no performance claim follows. Logger/create-new output failures terminate the harness, not a production service.

