# Stage 8B — Launcher Usability and Visual Refinement

Date: 2026-10-18 (Asia/Calcutta).

Result: one narrowly scoped keyboard-guidance correction, validated by regression tests, controlled renders, and live automated interaction. Production accessibility remains **BLOCKED**. This is not a release-readiness determination.

## Baseline

Repository: `X:\Project\SeiDesk\sei-desk`.
Starting and final branch: `main`.
Starting and final HEAD: `af7db2f0a9b1fc2540071a3d2f41c9f4ca7c139c`.

Complete starting `git status --short`:

```text
 M crates/sei-desk/src/desktop.rs
 M crates/sei-desk/src/desktop/tests.rs
?? docs/implementation/STAGE-8A-LAUNCHER-UX-PRODUCTIZATION.md
?? docs/implementation/STAGE-8A1-HUMAN-CONFIRMATION.md
```

The pre-existing desktop changes include the Stage 8A empty-result Enter behavior and the owner's requested All applications default. Both are preserved. No staging, commit, push, reset, clean, restore, or stash was performed.

## Inspection and selected issue

Read the UI/UX specification, especially §8; inspected approved `docs/design/mockups/03-start-launcher-states.png` and its corresponding SVG; read Stage 8A and Stage 8A.1 reports; inspected launcher rendering, invocation, selection, Enter handling, tests, and existing evidence.

The specification requires no action on Enter with no results and useful no-results guidance. Plate 03 supplies recovery/query-edit and Escape guidance in that state. The implementation instead displayed `Up/Down selects · Enter launches · Tab changes focus` even when there were no results. This advertised unavailable actions, particularly after Stage 8A made empty-result Enter a deliberate no-op. The earlier Stage 8A `after-no-match-enter.jpg` also shows that footer; it is historical evidence, not a new before capture.

Selected correction: make the existing footer truthful for an empty result set. This changes one text decision without introducing state, redesigning the launcher, or changing input/launch behavior.

- `crates/sei-desk/src/launcher.rs`: private `keyboard_hint` helper chooses `Edit search · Tab changes focus · Esc closes` when results are empty. The view uses that helper. Populated results retain the existing hint; a valid Retry launch state retains its existing hint and precedence.
- `crates/sei-desk/src/launcher/tests.rs`: new `keyboard_hint_does_not_offer_launch_when_results_are_empty` regression covers populated results, no match, clearing, empty Favorites, loading/default state, and Retry precedence.
- This report is the only new nonignored file.

No further visual correction was justified within this bounded pass. Palette, typography, geometry, selected-item styling, and ownership/lifecycle remain unchanged. Existing differences between Plate 03 and current composition are deferred; the user's All applications default is preserved rather than changed to the specification's pinned-first proposal. No approved design assets or historical reports were rewritten.

## Validation

Commands used existing locked/offline dependencies, with `CARGO_INCREMENTAL=0` for Rust validation.

| Command | Actual outcome |
| --- | --- |
| `cargo test -p sei-desk --locked --offline keyboard_hint_does_not_offer` | First failed against the old footer behavior (0 passed, 1 failed); after correction, 1 passed. |
| `cargo fmt --all` | Passed. |
| `cargo test -p sei-desk --locked --offline desktop::tests` | 11 passed, including existing empty-result Enter, All applications default, keyboard, and lifecycle coverage. |
| `cargo test -p sei-desk --locked --offline launcher::tests -- --skip launcher_controlled_state_render_evidence` | 2 passed, including existing long-title bounds coverage. |
| `cargo test -p sei-desk --locked --offline launcher_controlled_state_render_evidence -- --ignored` | 1 passed, with `SEI_LAUNCHER_RENDER_DIR` pointing to the new evidence directory's `renders` subdirectory. |
| `cargo fmt --all --check` | Passed. |
| `cargo check --workspace --all-targets --locked --offline` | Passed. |
| `cargo test --workspace --locked --offline --quiet` | 139 passed, 0 failed, 6 ignored: 45 service, 76 desktop, 18 Windows tests passed. |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Passed. |
| `cargo build --workspace --release --locked --offline` | Passed. |
| `git diff --check` | Passed. |

Focused and render tests overlap workspace coverage and are not additional unique test totals. The usual six ignored tests include the controlled renderer (run separately above) and five native opt-in tests; those five were not run. Workspace checks used approved ordinary filesystem access for existing atomic preference fixtures; no administrator elevation, installation, or dependency change occurred.

## UI evidence actually obtained

New uniquely named ignored evidence root:
`target/stage8b-ab830b8b3e92431f92fb9efbb2c49e45/`.

The rebuilt release executable was run with `--diagnostics`, with process-local `LOCALAPPDATA` set to that directory's new `profile` subdirectory. Live checks used automated mouse/keyboard interaction with the actual Windows app, **not human physical input**:

| Scenario | Observed result | Evidence under the root |
| --- | --- | --- |
| Local Ctrl+L opens launcher | All applications, 78 catalog results, search focus, existing populated hint | `live-results.jpg` |
| Search `zzzz-no-app`, then Enter | Query and no-results guidance remain; recovery footer visible | `live-no-match-enter.jpg` |
| Ctrl+A, Backspace | Results and selection return; search remains usable; populated hint restored | `live-cleared.jpg` |
| Select empty Favorites | Empty guidance and recovery footer visible | `live-empty-favorites.jpg` |
| Close manager normally with companion open | Process exit 0; log ends with `dashboard_workers_joined` | `runtime.log` |

No catalog application was launched in this pass. The automated interaction did not test native taskbar coexistence, global shortcuts, Explorer, or fullscreen behavior.

The existing controlled renderer produced 14 actual iced renders: results, long-title, no-match, launch-failure, loading, refresh-failure, and empty-favorites, each at 580×440 and 360×300. Raw RGBA and lossless PNG copies are in `renders/`; PNG conversion used the existing Python/Pillow runtime. These are controlled model/render fixtures, not reproduced Windows service or launch failures. Visual inspection found the footer within bounds at both sizes. Four live captures and all 14 controlled PNGs were reviewed; a fresh finish review returned SHIP with no material fixes. A separate read-only documentation review confirmed the limited state-guidance change and unchanged visual language. No detector or additional native qualification was inferred from either review.

## Preservation and final state

The baseline inventory and hashes are recorded in the evidence root as `status.txt` and `hashes.json`. Of 745 original tracked/nonignored files, 743 remain byte-identical; only the two intended launcher source/test files changed. In particular, both pre-existing desktop modifications and both untracked Stage 8A reports remain byte-identical. Manifests, lockfiles, other source, existing evidence, and design assets remain unchanged. Generated evidence is confined to the new ignored directory.

Exact Stage 8B changed paths:

```text
crates/sei-desk/src/launcher.rs
crates/sei-desk/src/launcher/tests.rs
docs/implementation/STAGE-8B-LAUNCHER-USABILITY.md
```

Complete final `git status --short`:

```text
 M crates/sei-desk/src/desktop.rs
 M crates/sei-desk/src/desktop/tests.rs
 M crates/sei-desk/src/launcher.rs
 M crates/sei-desk/src/launcher/tests.rs
?? docs/implementation/STAGE-8A-LAUNCHER-UX-PRODUCTIZATION.md
?? docs/implementation/STAGE-8A1-HUMAN-CONFIRMATION.md
?? docs/implementation/STAGE-8B-LAUNCHER-USABILITY.md
```

## Limitations and stop

No new physical-keyboard, Narrator, UI Automation, real IME, DPI, high-contrast, display, or fullscreen qualification occurred. Stage 8A.1 human observations remain historical evidence. Production accessibility remains **BLOCKED**. No dependencies, framework, shell behavior, accessibility architecture, or manager/companion lifecycle changed. The bounded correction and report are complete; await owner review without starting another stage or committing/pushing.
