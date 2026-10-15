# Stage 2D — Test matrix

Date: 2026-10-15. Overall **PASS within the isolated-spike/headless-next-stage boundary**, with explicitly approved text-range deferral. See the [closure report](STAGE-2D-ACCESSIBILITY-CLOSURE.md) for the evidence limits and [evidence index](evidence-stage2d/README.md).

| Check | Result | Evidence / limit |
| --- | --- | --- |
| Fresh Windows UIA discovery, correct process/window, names, roles, enabled/focusable states, real bounds | Passed | Both current runs; complete 29 named assertions in each run.json. |
| Input ValuePattern read/set, model synchronization | Passed | Separate native UIA client, not a mock. |
| Validate Invoke, error exposed, initiating focus retained | Passed | Native UIA assertions. |
| Dismiss Invoke, error/action removed, retained old action rejected | Passed | Native UIA assertions including element-not-available HRESULT. |
| Checkbox Toggle, changed/restored semantic state | Passed | Native UIA assertions. |
| Resize/move bounds, close Invoke, provider detach, stale closed input rejected, no child remaining | Passed | Both fresh runs, normal exit code 0. |
| Physical Tab/Shift+Tab, editing in middle, selection replacement, keyboard Validate/Toggle/Dismiss | Passed — user-attested | User reported PASS and EVERYTHING WORKS against the manual checklist; not synthetic-input evidence. |
| Keyboard focus restoration to input after error dismissal | Passed — user-attested | Supplied manual checklist; does not qualify cross-application foreground restoration. |
| Narrator names/roles, editing/navigation/activation, checkbox states | Passed — user-attested | Human confirmation; no exact spoken transcript/audio supplied. |
| Narrator repeated error feedback, stable focus, missing/duplicate announcements | Passed — user-attested | User confirmed checklist globally; no exact speech or duplicate count available. |
| Polite live-region API and Windows adapter mechanism | Research only + compiled | Exact released source inspected; semantic regression test passed. |
| Native live-event emission/count capture | Blocked | Available managed UIA lookup did not expose event/property; no independent event count. Human feedback is separate evidence. |
| Stale async layout/focus and late-after-close messages | Passed — unit tests | Two new App update tests; not exhaustive native concurrency stress. |
| Unchanged error delta, focus stability, removed live node | Passed — unit test | New semantic test; not proof of speech count. |
| Formatting/check/tests/strict Clippy/release build | Passed | Final quality-gates.txt sequence; 13 tests. |
| Idle CPU/memory | Passed — measurement completed | Two 30-second release samples; no performance guarantee. |
| UIA TextPattern, caret/selection query and text ranges | Not implemented — approved limited deferral | User approval only for headless Stage 3; mandatory disposition before launcher UI. |
| High contrast, non-default/mixed DPI, IME/graphemes, other screen readers | Not run in Stage 2D | Not implied by general manual confirmation; production qualification remains. |
| External-application focus restore, queue saturation, forced exit, long-duration leaks | Not run in Stage 2D | Preserve earlier evidence; no exhaustive reliability claim. |

## Reproduce automated checks

From repository root, on Windows with the locked dependencies available:

```powershell
cargo fmt --manifest-path experiments/accesskit-iced-spike/Cargo.toml --check
cargo check --manifest-path experiments/accesskit-iced-spike/Cargo.toml --locked --offline
cargo test --manifest-path experiments/accesskit-iced-spike/Cargo.toml --locked --offline
cargo clippy --manifest-path experiments/accesskit-iced-spike/Cargo.toml --all-targets --locked --offline -- -D warnings
cargo build --manifest-path experiments/accesskit-iced-spike/Cargo.toml --release --locked --offline
powershell.exe -NoProfile -File experiments/accesskit-iced-spike/Test-UIA.ps1 -OutputDirectory docs/implementation/evidence-stage2d/uia-new-run -IdleSeconds 30
git diff --check
```

Use a fresh output directory. The harness starts its own release child, exercises all 29 assertions, samples process counters, closes it, and reports child/provider cleanup. Read `run.json` and stdout/stderr; process exit alone is insufficient. `Narrator` and `PhysicalKeyboard` marked Not run in that JSON refer only to the automated harness. Their separate human results are in manual-attestation.json. Do not reuse older captures as a current pass.

## Reproduce human acceptance

Launch `experiments/accesskit-iced-spike/target/release/accesskit-iced-spike.exe` on an interactive desktop, without synthetic input. Record binary hash, OS, display scale, and observer. Start Narrator yourself when ready.

1. Physically type `abcd`, press Left, then `X`: expect `abcXd`. Select with Shift+Left and replace. Check visual caret/selection, then Ctrl+A/Backspace to clear.
2. Tab to Validate, Shift+Tab back, then Tab/Enter. Error should appear with focus still on Validate.
3. Tab to Prefer reduced motion and Space to toggle. Tab to Dismiss error and Enter: error disappears and focus returns to Sample text.
4. With Narrator, identify and operate input, Validate and checkbox. Record exact names/roles, value speech, checked/unchecked announcements, and editing/navigation limitations.
5. Validate empty input. Record exact error speech or silence, duplicates, and focus. Dismiss and repeat for a new error occurrence. Do not infer announcements from a UIA tree alone.
6. Switch to another application and back; record focus behavior separately. Close normally and check no process remains. Stop Narrator if started only for testing.

The user confirmed steps supplied in this session, which covered steps 1–5 and normal close. Cross-application switching was not in that supplied checklist and is not credited as executed. For future runs, record individual outcomes and speech rather than only an aggregate confirmation. Unperformed steps must stay Not run or Blocked.
