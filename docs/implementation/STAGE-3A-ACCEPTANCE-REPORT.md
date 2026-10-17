# Stage 3A acceptance review

15 October 2026. Reviewed commit: `e11af2cf6baea5c20e1edc7d48c603c6ab77397c` on `main`, with a clean initial working tree.

## Verdict: PASS — documented headless subset

The catalog and launch service are ready for a small **headless consumer integration** on the qualified Windows host, under their existing source/launch constraints. No runtime defect requiring a fix was confirmed. This is not a complete Windows application inventory, a security sandbox, cross-build production qualification, or approval for production launcher UI. The programmatic text-range/caret/selection accessibility gate remains unchanged.

The exact commit's documented Windows suite was executed first, before changing source. It passed formatting, all-target compilation, ordinary workspace tests, strict Clippy, release builds, all 16 opt-in-inclusive sei-apps tests and the installed-catalog probe. Two additional real Windows acceptance tests were then added, and the complete suite was executed again successfully. All original evidence remains intact. New results are in [evidence-stage3a-acceptance](evidence-stage3a-acceptance/README.md); they are not copied passes from the original report.

## Executed checks and evidence boundaries

Environment: Windows NT **10.0.26300.0**, x64/MSVC, Rust/Cargo **1.99.0**. Current-token administrator membership was false; no RunAs/UAC elevation was requested. Filesystem sandbox exceptions used for test-owned file operations are separate from Windows privilege elevation; see [environment.json](evidence-stage3a-acceptance/environment.json).

| Behavior | Result observed during this review |
| --- | --- |
| Exact commit, documented suite | **Passed.** 25 ordinary workspace tests, two opt-in launch tests separately enabled, 16 sei-apps tests including those opt-ins. Original probe executable hash matched the previously recorded hash. |
| Final tree including acceptance tests | **Passed.** Same 25 ordinary workspace tests; four deliberately ignored opt-in tests enabled separately; **18 sei-apps tests** passed in that run (16 library + 2 integration). These totals overlap; do not add ordinary and opt-in counts as unique tests. |
| Identity and normalization | Pure/injected tests passed for whitespace normalization, path components/separators/ASCII case, effective default cwd identity and case-sensitive argument distinctions. Real COM duplicate links deduplicated; changing the representative shortcut filename preserved its launch identity. |
| Ordering and deduplication | Injected source-order reversal and repeated fixtures passed. Real shortcut discovery repeated identically while locked/stale/malformed fixtures were present. Two installed-catalog refreshes matched within each fresh probe. |
| Bounds | Fixture tests passed for node/depth/file-size limits and rejecting an entire over-budget directory before parsing its arbitrary enumeration prefix. Code review confirms fixed root count, bounded buffers/arguments and no disk-wide traversal. These are count bounds, not a hard OS-call timeout. |
| Malformed and stale shortcuts | Real invalid `.lnk` returned Native / shortcut Load / HRESULT `0x80004005`; a real shortcut whose target had been removed returned Missing / target metadata / code 2. Other entries remained usable. |
| Inaccessible shortcut | A test-owned `.lnk` opened with share mode 0 caused real COM Load failure `0x80070020` (sharing violation); catalog reported an issue without aborting. No ACL change was needed. This does not establish actual ACL-denial behavior. |
| Inaccessible/stale launch target | Exclusively locked owned fixture copy failed CreateProcess with code **32**, mapped ProcessCreation. After successful preflight and deletion, the cached request failed target validation with Missing / code **2**. Neither failed attempt created its output marker. |
| Executable argv | Real test-built child received exact empty, space-containing, quoted, backslash, Unicode and shell-like arguments. Structured `.args` and shortcut `.raw_arg` paths both passed. |
| Working directory | Real child confirmed explicit cwd, shortcut cwd, and executable-parent default when omitted. |
| Shortcut launch semantics | Ordinary local executable metadata was read through COM and dispatched directly. Run-as flag was persisted in a real `.lnk` and rejected. No Resolve/tracking repair or ShellExecute occurred. This verifies the documented metadata-to-request subset, not Explorer's complete shortcut behavior. |
| URI/packaged cases | Policy rejection tests passed. These targets remain unsupported; no protocol handler, AppsFolder or packaged application was activated. |
| Enumeration does not execute targets | Production call graph contains filesystem/COM read operations only; no spawn/Resolve/ShellExecute route. Real discover calls over fixture-launchable links left the fixture marker absent, including after a short 200-ms observation. The marker alone is not a general proof against arbitrary platform vulnerabilities; it corroborates the call-graph review. No installed target was launched. |
| Cancellation | Before-start fixture tests passed. A cancellation hook following one real COM read returned Cancelled, discarded the partial catalog and prevented a second read. Cancellation is cooperative at entry boundaries; the current entry's normalization can finish and individual OS calls are not interrupted. Pre-launch cancellation created no output. Post-creation cancellation left the accepted fixture able to finish normally. |
| Errors | Injected mappings for Windows 2/3/5/740/193/216/8 passed; actual lock, stale-file, malformed-COM and invalid-executable paths were tested separately. ACL access denial and elevation-required behavior remain injected mapping evidence, not executed UAC/ACL cases. |
| Quality gates | fmt --all --check, workspace all-target check, workspace tests, strict all-target Clippy, release workspace/examples, opt-in Windows tests and git diff --check passed on the final reviewed tree. |

New tests reside only in the existing native test module and Windows integration test file. They use owned temporary files and the freshly built fixture (or a byte-for-byte file copy), clean their files, wait for owned children, and kill/wait only an owned fixture if its ten-second test deadline expires. They add no runtime behavior or dependency. The existing AccessKit spike and all production GUI files are untouched.

## Failures encountered and disposition

The new shortcut test initially failed at its test-owned filename rename with Windows code 5 under the filesystem sandbox. Releasing the explicit file lock did not change that sandboxed result. The identical test passed outside filesystem sandbox restrictions, and the complete final suite also passed there. The first two failed captures are preserved; this was not hidden or converted into a mocked success. No service code change was made to work around it. Ordinary Windows testing outside this agent sandbox does not require RunAs.

The original suite reported no fresh behavioral failures. Its earlier historical sort-assertion and 193-only format-error failures remain in the original evidence and were not rerun as supposedly current failures.

## Resource measurements and comparison

The existing observer was rerun: release process counters sampled every **50 ms**, including two explicit catalog refreshes and a **two-second idle hold**. No background catalog polling is introduced by the service. These are whole-probe measurements, not a production idle CPU budget, exact peak allocation or leak test.

| Sample | First / second refresh | Entries / nodes / issues | Sampled max working set / private commit | Last sampled CPU / observation |
| --- | --- | --- | --- | --- |
| Prior recorded evidence (comparison only) | 289.798 / 204.779 ms | 78 / 252 / 83 | 19,099,648 / 3,706,880 bytes | 0.484375 s / 2.7434 s |
| **Fresh exact-commit run** | **285.778 / 198.150 ms** | **78 / 252 / 83** | **19,062,784 / 3,616,768 bytes** | **0.484375 s / 2.7799 s** |
| **Fresh final review run** | **234.042 / 183.581 ms** | **78 / 252 / 83** | **19,234,816 / 3,547,136 bytes** | **0.4375 s / 2.6255 s** |

Within each fresh probe, returned entries were identical across refreshes. Count and exclusion-operation aggregates match the original record (46 absolute-path, 24 extension, 6 host, 2 special-flag, 5 cwd exclusions). Historical full entry snapshots were intentionally not persisted, so cross-run equality of every historical entry cannot be asserted from matching counts alone. Cache state was not controlled: the older report's “cold-cache” wording is not an established measurement condition.

Exact-commit executable SHA256 matched prior evidence: `80B773A27C2B0104974995B9964F6985629894E779C04562E4055A43E0E5E35C`. The final test-extended rebuild is separately identified in its resource JSON; do not treat its capture as execution of an unchanged binary. No remaining owned probe/fixture process was found at final review. Normal cleanup does not prove exhaustive heap/handle leak freedom or forced-termination behavior.

## Native, security and lifecycle review

- COM owner is !Send/!Sync, initialized on an operation-owned STA thread and balanced for successful initialization including S_FALSE. Local COM interface references drop before apartment teardown. Known-folder task memory is copied then freed, including conversion failure. Buffers and pointers are scoped to synchronous calls; unsafe operations are documented in the native module. No COM object or borrowed pointer crosses into a catalog entry.
- The production path never evaluates a command string or invokes a shell intermediary. It supplies an explicit executable to Rust Command; raw arguments preserve existing shortcut suffix semantics only. `cmd.exe` and other named hosts are conservatively filtered, but that filter is not a security allowlist and cannot prevent a renamed interpreter or arbitrary executable from performing its own actions.
- No runas, automatic privilege retry, taskbar replacement, shell registration or persistent modification exists. CreateProcess runs with the caller's token and inherited environment. Accepted applications are not sandboxed and may themselves request privileges; this service does not request them.
- Validation rejects relative/parent paths, device/UNC paths, alternate streams, NULs, invalid encoding and final-component reparses, with conservative length/count caps. Reparse ancestors, mapped drives, aliases/case-sensitive directories, concurrent installer changes and file substitution are documented limitations. Preflight is not an immutable trust check; creation errors remain authoritative.
- A catalog launch request is a snapshot: launch revalidates its executable/cwd, not the original `.lnk` file. Retargeting/deleting a shortcut takes effect on a subsequent catalog refresh; an older request may remain launchable if its stored executable still exists. Consumer integration must select from the current catalog generation rather than assume a live shortcut handle.
- Worker handles and accepted Child handles have explicit caller ownership. Dropping a JoinHandle detaches work, so consumers must retain handles and arrange off-UI-thread joining. Dropping Child closes handles without terminating the application. Counts/progress are bounded/nonblocking; callers must serialize refreshes and avoid an unbounded flood of independent launch workers. No continuous polling service is present.

## Unresolved Windows-specific qualification

These do not block the small headless integration recommended below, but remain required before broader claims or scope expansion:

1. Supported Windows 10/11 build matrix, declared Rust 1.88 compiler and non-x64 targets were not qualified by this host.
2. Actual ACL-denied folders/executables, elevation-required manifests, mapped/redirected drive outages, forced termination, queue/request floods and long-duration leaks were not exercised. Share-lock tests are not substitutes for those scenarios.
3. Packaged applications/AUMIDs, URI/document associations, App Paths environment semantics, MSI repair, shell verbs, show/minimize hints, localization and icon extraction remain outside this implementation.
4. Long shortcut target paths and unusual Unicode/filesystem aliases remain unqualified. [GetPath documentation](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getpath) documents a MAX_PATH return limit; a 32K output buffer and 4096-unit request validation limit alone do not establish long `.lnk` target support. [GetArguments documentation](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getarguments) warns about truncation; current boundary-saturation checks reject potential truncation at its buffer end, but oversized/native-version edge cases were not newly tested. Do not claim arbitrary-length shortcut fidelity.
5. Filesystem/COM calls may block; cooperative cancellation cannot preempt them. Strong wall-clock deadlines or hostile-file isolation would require a separately justified boundary. No unsafe thread termination or false cancellation of launched applications is proposed.
6. Production launcher accessibility, focus, selection and announcement behavior is not tested or approved by this headless review. Previous accessibility evidence remains historical and intact.

## Required fixes and smallest next integration

**Required runtime fixes: none confirmed within the documented subset.** Two acceptance regression tests were added to close real Windows evidence gaps; original service behavior and dependencies are unchanged.

Recommend one **headless catalog-session consumer**: retain the last usable snapshot, serialize explicit refreshes, attach generation IDs to completion/error results, resolve explicit launch selections against the current snapshot, and own worker/child handles without blocking a future UI thread. Test cancellation, superseded completions, partial-source failure and acceptance/error propagation with the existing fixture. Do not wire this into production launcher widgets or add catalog sources as part of that minimal step. No next-stage integration was implemented here.

Git: changes are uncommitted on `main`. Modified only `crates/sei-apps/src/native.rs` (test module additions) and `crates/sei-apps/tests/windows_launch.rs` (new opt-in test), plus this report and a new acceptance evidence directory. Preservation hashes verify all other pre-existing files, including the original evidence and accessibility spike, remain unchanged.
