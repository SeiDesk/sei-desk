# Stage 3A — Launch service

15 October 2026. Headless implementation in [sei-apps](../../crates/sei-apps/src/lib.rs). See the [catalog report](STAGE-3A-APPLICATION-CATALOG.md), [test matrix](STAGE-3A-TEST-MATRIX.md) and [execution evidence](evidence-stage3a/README.md). No production UI or shell integration is added.

## Request, execution and ownership

`LaunchRequest::executable(PathBuf, Vec<OsString>, Option<PathBuf>)` constructs a direct request and checks syntax only. `launch_async(request, Cancellation)` owns one finite worker; validation and process creation run there. The lower-level blocking `launch` and `validate` APIs are for worker/tests, not UI-thread calls. Thread creation errors, join panics, and service errors are distinct channels; a future UI must handle all three without blocking on join in its update path.

The worker rechecks syntax, target existence/type and working directory immediately before creation. Rust `std::process::Command` provides the explicit executable and structured `.args`, using Windows process creation instead of string concatenation, file associations, PATH lookup or an injected command interpreter. Stdin/stdout/stderr are null for the ordinary service path. `CREATE_NO_WINDOW` suppresses an automatic console popup for console applications; GUI applications remain GUI applications. This is a deliberate minimal launch policy, not a complete reproduction of Explorer's console/show behavior.

`LaunchReceipt` owns `std::process::Child`. Success means **process creation was accepted**, not that initialization succeeded, a window appeared, or foreground was acquired. Caller may obtain the PID or explicitly wait outside the UI thread. Dropping the child closes owned handles but does not kill the application. The normal service never waits for application exit, restores another window, installs hooks or changes foreground policy.

Cancellation is checked at worker entry and immediately before spawn. Cancellation during validation is observed before creation; individual filesystem/CreateProcess calls are not abortable. A request cancelled before those checks returns Cancelled. If cancellation races after the final check, process creation can still succeed, and that success is returned honestly. Cancellation **never terminates a launched application**. There is no retry loop, process-kill feature or background reaper service.

## Launch semantics by target type

| Type | Implemented behavior |
| --- | --- |
| Absolute ordinary drive `.exe` path | Direct CreateProcess through Rust Command; structured arguments and explicit cwd or executable-parent default. No automatically selected executable from current directory or PATH. |
| Supported Start Menu `.lnk` | Read-only COM metadata is converted to an explicit executable request. Its argument suffix is kept opaque and passed with Windows `CommandExt::raw_arg`; target and cwd are revalidated. The `.lnk` itself is never ShellExecuted. |
| Special `.lnk` / MSI / alternate user / shim / namespace target | Unsupported; no guessed fallback or runas dialog. Shortcut icon metadata may be retained but never executed. |
| Packaged application / AUMID | Unsupported in Stage 3A. Separate discovery and IApplicationActivationManager work would be needed. Do not feed an AUMID to CreateProcess. |
| URI / `.url` / document association | Unsupported; no protocol handlers, downloads or implicit ShellExecute. An ordinary executable may receive a literal URI argument only if a caller explicitly supplies it. |
| Batch/script or named shell host | Unsupported by the current filter. This does not sandbox other executables or detect all interpreters. |

Shortcut arguments are a documented exception to token-vector handling: the [IShellLink argument contract](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getarguments) gives a Windows string, not an argv array. Guessing tokenization could change application-specific syntax. [Rust raw_arg](https://doc.rust-lang.org/std/os/windows/process/trait.CommandExt.html) preserves it without additional quoting; it does **not** invoke a shell. Never concatenate search/query text into it. `Arguments::ShortcutRaw` is a provenance convention, not a capability/security token: callers supplying such metadata are responsible for explicit user launch intent.

Structured argument escaping was tested against the test-built Rust child, including empty args, spaces, embedded quotes, backslashes, Unicode and literal `&|%PATH%$()`. Application parsers can differ from standard Windows/Rust argv behavior; a vector cannot guarantee every executable's private grammar. Raw shortcut fixtures separately verified spaces, empty arguments and literal shell-like characters, plus cwd. No shell interpretation or environment substitution was observed in those fixture arguments.

This subset does not preserve every shell-link property: show/minimize hints, shell verbs, compatibility machinery, environment-path expansion, installer repair, DDE and shell grouping/activation are not reproduced. Applications depending on those semantics are outside this qualification. A future expanded implementation must test its own behavior before changing this contract.

## Validation and structured errors

Reject relative/parent-traversing, UNC/device, alternate-stream or NUL-containing Windows paths. Paths are limited to 4,096 UTF-16 units. Require `.exe` and a regular non-reparse final target; require a valid non-reparse explicit cwd. Structured arguments must be valid Unicode without NUL, at most 256 arguments and fewer than 6,000 UTF-16 units in total; raw shortcut arguments must be NUL-free and shorter than 16,000 UTF-16 units. Conservative limits leave room for executable/argument quoting within the [CreateProcess command-line limit](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw).

| ErrorKind | Examples / handling |
| --- | --- |
| Cancelled | No launch at the observed cancellation point; caller retains catalog/query. |
| Missing | Removed target or cwd; Windows codes 2/3 preserved; refresh/retry only on explicit request. |
| AccessDenied | Access rejection or elevation-required code 740; do not retry elevated. |
| Invalid | Invalid syntax/type or bad executable format (193/216); retain native code and operation. |
| Unsupported | Target/source/shortcut semantics outside the qualified subset; do not silently invoke Explorer or a shell. |
| ProcessCreation | Other OS spawn failure; preserve its native code for diagnosis. |
| Native / Limit | COM/filesystem failure or bounded discovery exclusion; see operation/code and per-source catalog issues. |

Existence checks are advisory: files, ACLs, targets and directories can change before creation. The actual spawn result remains authoritative. There is no signature trust check or immutable file-handle launch; path deduplication and shell-host exclusions are not malware protection. A catalog entry must never trigger launch merely because enumeration found it. The intended future route is an explicit user action selecting a catalog entry or trusted request.

No elevation is requested, even when the executable's manifest requires it. The service runs with the invoking process's token and inherited environment; it is not a sandbox. The executing host was not launched with a RunAs request for these tests. System settings, native taskbar, Explorer, startup configuration and registry are untouched.

## Evidence and integration disposition

Fresh Windows tests created real COM shortcut files in owned temporary directories and launched only the freshly built `launch_fixture.exe`. They verified actual argument/cwd round-trips, run-as-flag exclusion, normal child exit, pre-launch cancellation and post-spawn non-cancellation. An existing invalid executable reached CreateProcess and produced **216** on this host; the Invalid mapping accepts documented format/machine-mismatch codes 193/216. Error mapping for access denial/elevation-required is injected unit evidence, not a real privilege or UAC test. No arbitrary discovered application was launched.

The future UI should consume asynchronous results, keep the selected item/query on failure and close only after process acceptance, as the existing design specifies. It must not restore an old foreground window over a newly launched application. None of that UI behavior is implemented or qualified by this headless stage. Production launcher text-range/caret/selection accessibility remains a separate gate.
