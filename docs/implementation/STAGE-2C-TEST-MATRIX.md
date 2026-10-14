# Stage 2C — Windows test matrix

14 October 2026. **Core UIA slice passed on this host; overall accessibility gate not passed.** Narrator and physical keyboard operation remain unavailable/unrun. See [report](STAGE-2C-ACCESSKIT-REPORT.md), [experiment commands](../../experiments/accesskit-iced-spike/README.md), [evidence](evidence-stage2c/README.md).

Test environment: Windows NT 10.0.26300.0 x64/MSVC, Rust/Cargo 1.99.0; released iced 0.14.0, accesskit 0.25.1, accesskit_windows 0.35.1, accesskit_consumer 0.39.1. Final binary SHA256 `4F4265B39FBB47A16B280D622A62899E94C2B3EF9D99261DF8E793ECD9B15CE5`. Ordinary runtime rights, no UAC/RunAs. Results do not qualify other Windows release builds.

| ID | Check | Status | Actual evidence / remaining work |
| --- | --- | --- | --- |
| A01 | Released API, license and handle compatibility | Passed for prototype | Exact packages inspected, compiled; own HWND/thread checked; trace confirms hidden pre-show attachment |
| A02 | Raw and control view discovery | Passed | Correct PID/HWND; stable `spike-*` IDs distinguish client controls from titlebar buttons; initial/error/final trees saved |
| A03 | Input name, role, value and editability | Passed for ValuePattern | Sample text Edit; empty→sample→empty through actual cross-process SetValue; own-client image shows sample |
| A04 | Input label mapping | Partial | Name and separate Text label correct, LabelledBy authored in AccessKit; UIA LabeledBy property absent in released Windows adapter |
| A05 | Validate Invoke | Passed | Nonempty value has no error; empty value creates actual error text/Dismiss |
| A06 | CheckBox Toggle/state synchronization | Passed | Prefer reduced motion CheckBox, Off→On→Off; state trace, UIA and captured real checkbox agree |
| A07 | Error text and Dismiss Invoke | Passed | Text named Enter some text before validating.; Dismiss Button/Invoke appears, then both disappear from raw/control trees |
| A08 | Focus / error preservation | Passed for tested UIA sequence | Explicit Focus-to-Validate reaches local focus; error retains Validate focus. Trace separates Focus from Click. This is not an external foreground restoration test |
| A09 | Stale removed control | Passed for retained Invoke pattern | Final run asserts ElementNotAvailableException and HRESULT -2147220991; no stale model action |
| A10 | Geometry / move / resize | Passed at current DPI | Initial positive client bounds, actual input width grows with native Transform resize, restores, moves by 20 px and restores; captures/trees retained |
| A11 | Normal Close and stale value pattern | Passed | Actual client Close Invoke, exit 0, provider detached, no child; retained ValuePattern returns UIA_E_ELEMENTNOTAVAILABLE |
| A12 | Incremental updates / conditional instance IDs | Passed for tested transitions | Actual insertion/removal trees plus pure delta/epoch regression tests; no claim of exhaustive update stress |
| A13 | TextPattern, caret, selection, IME semantics | Unsupported/unverified | No TextPattern or text selection action advertised; whole-string Value replacement is not accessible editing proof |
| A14 | Live error notification / speech | Not implemented / Not run | Plain error text is exposed; no Live/announcement claim or event-listener success |
| K01 | Physical Tab/Shift+Tab, Enter/Space/Escape | Blocked / Not run | Native physical-interaction tooling unavailable; 7 reused state tests are not real keyboard validation |
| K02 | Physical typing, selection, edit-in-middle, IME | Blocked / Not run | Current spike does not attempt synthetic keys; previous Stage 2B injection failures are not relabeled as Stage 2C tests |
| N01 | Narrator identifies and operates essential controls | Blocked / Not run | Narrator installed in the environment, but native interactive/audio testing unavailable; essential adoption gate remains open |
| F01 | Switch to another app and return | Not run | No focus theft API is called, but source inspection is not external focus-switch validation |
| D01 | Non-default/mixed DPI and monitor changes | Not run | Scale query/update path exists; only current host configuration observed |
| D02 | High contrast and OS reduced-motion changes | Not run | No animations or preference writes; checkbox is local only; high-contrast adaptation not implemented |
| L01 | Forced process termination / provider reactivation / concurrent action stress | Not run | Normal detach does not validate forced cleanup or all races; no shell resources held |
| P01 | Release idle CPU/memory | Measured | Separate process counters, 3 s settle + 30 s interval; variable samples, no acceptance guarantee |
| Q01 | fmt/check/tests/strict Clippy/release | Passed | 10 spike tests (3 new + 7 shared), 11 original-workspace regression tests; final gate log |
| Q02 | Preservation / diff / local links / test-client grammar | Passed | 147 pre-existing files unchanged, no process remains; diff/links/whitespace/parser checks clean |

Final UIA script run completed **29 checks**, all passed. Its `UIAActionsPassed=true` covers those checks only; its `AccessibilityGate` explicitly remains not passed. It does not award a pass to Narrator, selection or announcements.

## Reproducible UIA procedure

Use [experiment README commands](../../experiments/accesskit-iced-spike/README.md) from repository root. Run the release executable and Windows PowerShell 5.1 client with a new output path. The client:

1. Starts one owned process, checks root PID/title/type, captures real raw/control trees and confirms the HWND against attachment diagnostics.
2. Checks client AutomationIds, names, roles, enabled/focusable state and bounds inside the real client.
3. Reads/sets/clears ValuePattern, invokes Validate with nonempty and empty text, records actual error and initiating focus.
4. Captures the error tree/pixels, invokes Dismiss and checks removal; retries the **retained pattern** and requires UIA_E_ELEMENTNOTAVAILABLE, not just any exception.
5. Toggles the real checkbox twice and compares ToggleState; moves/resizes only its owned window and checks corresponding bounds changes before restoring geometry.
6. Measures app process CPU/memory after settling; invokes actual client Close; retries the retained Value pattern and asserts the exact unavailable HRESULT. It closes/kills only its owned process on failure and preserves the failure.

Do not change output directories to reuse old evidence or interpret an exception message alone. UIA state waits are bounded test-client polling, not production accessibility synchronization. Each OS/provider call itself can hang; count/time guards are not COM timeouts. A separate human/runner timeout is appropriate on unhealthy desktops. API/model tests are not screen-reader conformance.

## Pending interactive acceptance

These are planned tests, not executed results. Start the isolated window normally at standard user rights. Keep the native Windows shell available.

1. Physically Tab/Shift+Tab through Input, Validate, Motion, conditional Dismiss and Close. Verify visible focus and matching UIA HasKeyboardFocus. Enter/Space must act once; ordinary typing, arrows and modifier shortcuts must remain usable.
2. Type and edit at the middle of text, select/replace, try Unicode/IME. Validate via keyboard, inspect real error, dismiss it with keyboard/Escape, and confirm disappearing focus returns to Input. ValuePattern success does not establish caret, TextPattern or Narrator editing support.
3. With Narrator, identify **and operate** input, Validate, Motion, Dismiss and Close. Record speech/roles/value/checked state separately from raw/control tree observations. Current error has no live announcement; test whether that prevents essential use and record the result. Do not award an announcement pass from visible text.
4. Switch to another application, invoke only intended accessibility actions, return, and test normal close. Record OS foreground versus local widget/UIA focus separately. The experiment intentionally does not force foreground activation.
5. Use a prepared non-default DPI/high-contrast environment and, if available, mixed monitors. Move/resize/insert/remove error and compare real control hit tests/bounds. Do not change persistent system settings from the harness.
6. Record exact OS/build, tool and binary versions, actual speech/input observations and failures. Close only the tools/window you started. If essential Narrator/input operation fails, stop bridge expansion and document the smallest missing capability before changing strategy.

Only after these essential manual tests and current limits receive an evidence-based disposition should production bridge adoption be considered. No shell implementation is authorized by these results.
