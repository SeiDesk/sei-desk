# Stage 3A — Headless application catalog

15 October 2026. **Complete for the documented local-executable subset.** This is a headless service, independent of iced and AccessKit. The [Stage 2D limited deferral](STAGE-2D-ACCESSIBILITY-CLOSURE.md) remains in force: no production launcher UI is approved or implemented. Existing designs, accessibility reports, experiments and evidence are preserved.

## Source decision and research

Use the current user's **FOLDERID_Programs** and shared **FOLDERID_CommonPrograms**, obtained through [SHGetKnownFolderPath](https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath), rather than hardcoded environment paths. Read `.lnk` files with released Windows COM `IShellLinkW` and `IPersistFile::Load(STGM_READ)`. [Shell Links documentation](https://learn.microsoft.com/en-us/windows/win32/shell/links) explains their target, argument, working-directory and icon metadata. Start Menu presence is an intentional discovery boundary, not proof of application safety, installation completeness or launchability.

| Candidate | Decision |
| --- | --- |
| User/shared Start Menu Programs | Implemented: two known folders, read-only bounded traversal, local executable targets only. No administrator privilege requested. |
| Registry App Paths / Applications | Research only. [Application Registration](https://learn.microsoft.com/en-us/windows/win32/shell/app-registration) documents executable lookup and per-application environment/verb behavior. Not a complete installed-app inventory; reproducing those semantics adds scope. No registry scan or write implemented. |
| Packaged application identities / AppsFolder | Deferred. [IApplicationActivationManager](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-iapplicationactivationmanager) is a separate activation route; requires qualified discovery, identity and activation tests. Empty/non-filesystem shortcut targets are not treated as executables. |
| PATH, disk-wide executables, Documents/Desktop/Recent, uninstall registrations | Not enumerated. Avoid broad scans, unrelated personal data and treating uninstall commands as application launches. |

API research is separate from executed Windows tests in the [matrix](STAGE-3A-TEST-MATRIX.md). The native dependency is Microsoft's released **windows 0.62.2**, MIT OR Apache-2.0, from the maintained [windows-rs project](https://github.com/microsoft/windows-rs); it was already available in the experiment's dependency cache. This adds one direct Windows-only dependency and ten Windows binding packages to the root lockfile, with no upgrades to existing locked packages. Typed COM ownership avoids handwritten vtables. The binding package's collections/future/numerics support is transitive; the service itself uses no async runtime or WinRT collection framework. Existing windows-sys use in sei-windows remains unchanged.

## Workspace and operation boundary

```text
crates/sei-apps/
  src/lib.rs             cancellation, structured errors, explicit worker APIs
  src/catalog.rs         bounded traversal, normalization, deterministic catalog
  src/launch.rs          validated launch requests / process acceptance
  src/native.rs          thread-local COM ownership and shortcut metadata
  src/tests.rs           pure/injected catalog and policy tests
  tests/windows_launch.rs  opt-in test-built child process tests
  examples/catalog_probe.rs  count-only explicit refresh/resource probe
  examples/launch_fixture.rs test-owned argument/cwd recorder
tools/Test-Stage3A.ps1    workspace checks, fixture tests and resource measurement
```

Root workspace conventions (edition 2024, declared Rust 1.88, existing lints) are retained. Only root workspace membership and lockfile change among pre-existing files. The GUI does not depend on sei-apps yet. Native code is a private module with a narrow `LinkReader` boundary; no native handles or COM objects escape into catalog entries.

`refresh_async(Cancellation, Option<SyncSender<Progress>>)` creates one named finite worker and returns a fallible thread handle containing `Result<Catalog, Error>`. The worker owns its STA initialization, shell interfaces and cleanup. Successful COM initialization, including S_FALSE, is balanced on the same thread; the owner cannot move between threads. No Resolve, Save, tracking repair, shell launch, hooks, persistent settings, watcher or timer runs during discovery. Public blocking `discover` allows fixtures/custom explicit roots; call it only on a worker. It is not a general disk scanner.

The caller must serialize refreshes, retain and join workers outside the UI thread, and discard superseded results using its own request generation. There is no daemon or hidden global cache. Progress reports visited/accepted counts through nonblocking `try_send`; it may be dropped under backpressure. The final result is authoritative. A future UI should retain its last catalog until a refresh succeeds; partial catalogs with issues remain usable, while cancellation returns an error and discards that refresh's partial result. No UI integration is included here.

## Bounds and filtering

- At most two roots; at most **8,192 filesystem entries visited**, directory depth **8**, and **1 MiB per shortcut**. Callers may lower limits, not increase them. COM string buffers have 32,768 UTF-16 units and reject missing termination or potential truncation. Launch validation adds stricter argument/path limits.
- Each directory's entries are sorted before processing. If its entry list would overflow the remaining global node budget, reject that entire directory rather than admitting a filesystem-order-dependent prefix. `truncated=true` denotes this global node-budget stop. Depth/size exclusions are separately reported as Limit issues; they do not stop siblings.
- Skip filesystem reparse entries, including junctions/symlinks, and reparse roots. Require absolute ordinary drive paths on Windows; reject UNC/device paths and parent traversal. Ancestor junctions and mapped drives are **not comprehensively resolved or classified**, so this is not a guarantee that every accepted drive path is physically local. Redirection/network outages can still block an OS call; count limits are not a wall-clock deadline or filesystem sandbox.
- Require a regular existing `.exe` target and an existing non-reparse working directory if specified. Reject non-executable documents, `.cmd`/`.bat` scripts, URI/namespace targets, unresolved/relative paths, NULs, invalid encoding and malformed metadata. No environment expansion or target repair is attempted. Shortcuts that depend on those behaviors are excluded rather than guessed.
- Reject special shortcut flags for alternate-user/run-as, Windows Installer advertisement, shim-layer and separate-VM behavior. Reject these shell/script hosts by filename: cmd.exe, powershell.exe, pwsh.exe, wscript.exe, cscript.exe, mshta.exe, explorer.exe. This conservative scope filter is **not a security allowlist**; a renamed interpreter or any other executable can perform arbitrary actions after explicit launch.
- Missing/inaccessible/stale entries become per-source issues with operation, kind and native code. One bad shortcut does not abort the catalog. An unavailable root is an issue; the other root can still succeed. Metadata existence does not validate PE format, provenance, signatures, launch success, elevation requirements or an application's behavior.

Filesystem/COM calls are synchronous inside the worker and cannot be interrupted in progress. Cancellation is checked between entries and before returning. A blocked call requires waiting for Windows or a separately isolated process design if future evidence justifies hard deadlines. No unsafe thread termination is introduced.

## Entry model, identity and deterministic deduplication

Each entry contains stable-within-policy identity, normalized display name, launch request, representative shortcut path/source, and optional icon path/index metadata. Icons are not loaded or extracted. Their strings are untrusted metadata and require separate validation before future rendering.

Display names come from shortcut filenames, with surrounding/repeated whitespace collapsed. Identity `win-exe-v1` length-prefixes the executable path, **exact opaque argument suffix**, and effective working directory (explicit directory or executable parent). Path keys normalize separators/components and ASCII case; argument case and quoting are preserved. Equivalent launches from different names/sources deduplicate. Different arguments/working directories remain distinct. Renaming a shortcut preserves identity; moving an executable or changing arguments changes it. There is no randomized hash or misleading promise of a universal application ID.

User source precedes shared source; within each root, sorted traversal chooses the representative. Final entries sort by case-insensitive display name, exact name, then identity. This is deterministic for the same filesystem snapshot. Concurrent installer changes can alter a refresh. Unicode filesystem collation, case-sensitive directories, 8.3 aliases/hardlinks and upgrade identity migration are not fully normalized; future packaged applications should use their native identities rather than pretending these path keys are AUMIDs. No pin persistence/ranking/search UI is implemented.

Catalog names/arguments/paths exist in memory because they are necessary launch metadata. No telemetry, usage history or persistent cache is created. The resource probe stores only counts and diagnostic-operation aggregates, never discovered application names, targets or arguments. Production logging must continue to avoid these values by default.

## Executed baseline and remaining work

On Windows NT **10.0.26300.0**, x64/MSVC, Rust 1.99.0, final release probe found **78 entries**, visited **252** filesystem nodes, and reported **83** filtered issues without global truncation. Two explicit refreshes returned identical entries; timings were **289.798 ms** and **204.779 ms**. This covers a subset, not all installed applications. Issues: 46 absolute-path checks, 24 executable-extension checks, 6 shell-host exclusions, 2 special-flag exclusions, 5 working-directory checks. Counts do not establish the underlying reason for every excluded application's metadata.

Over a **2.7434-second** process sample including both refreshes and a 2-second idle hold, sampled maxima were **19,099,648 bytes working set**, **3,706,880 bytes private commit**, and last sampled CPU time **0.484375 seconds**. Sampling interval was 50 ms, performed by developer tooling only. This is whole-probe memory/CPU, not per-entry allocation, a long idle benchmark, or a service guarantee. The earlier retained full run had different cold-cache timings. See [verified evidence](evidence-stage3a/verified-run/resource-baseline.json) and [count-only output](evidence-stage3a/verified-run/catalog-stdout.txt).

Stage 3A is ready as a headless foundation with explicit source limitations. Packaged/URI/registry/environment-dependent shortcuts, localized names, richer identity and icon extraction need separately authorized headless milestones and tests. Production UI and its programmatic text-range accessibility gate remain closed.
