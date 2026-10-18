# Stage 7A — Desktop overlay feasibility and product scope

2026-10-17. **Recommendation for owner review: an explicitly invoked launcher companion, retaining the native Windows taskbar.** This is an internal implementation milestone proposal, not implementation authorization or release approval. Rust + iced remains the UI architecture. Production accessibility remains **BLOCKED** under Stage 5E; ordinary-window feasibility does not resolve it.

## 1. Decision summary

| Intended feature | Verdict | Decision implication |
|---|---|---|
| Ordinary borderless floating iced surface | **feasible** | Locked source exposes the required ordinary-window primitives. Product-specific lifecycle, input and deployment still need qualification. |
| Full top quick-control dock, including passive edge reveal | **unproven** | Its drawing and individual controls are plausible; non-activating global reveal, input pass-through, fullscreen and topology handling are not demonstrated in production. Defer this combined milestone. |
| Explicit keyboard-first launcher companion | **feasible with constraints** | Existing catalog/launch services and iced windows provide a bounded path. Explicit opening, window-scoped state, truthful focus failure and accessibility gates are essential. Recommend this first. |
| Custom bottom taskbar as a native-taskbar replacement | **not recommended** | AppBar is not a taskbar implementation. Tray compatibility, window identity/grouping, focus restrictions and independent recovery remain unresolved. Do not suppress the native taskbar. |
| Optional additional AppBar beside the native taskbar | **unproven** | Documented APIs exist, but production coexistence/recovery is not validated. Separate future decision and isolated tests would be required. |

“Feasible” here means supported APIs/source establish a plausible engineering path, not a verified runtime feature. Recommendations and acceptance criteria below are proposals. No owner approval, budget, staffing, deadline, new technical candidate or accessibility exception is inferred.

## 2. Baseline and evidence hierarchy

Repository: `X:\Project\SeiDesk\sei-desk`. Starting/final branch: `main`. Starting/final HEAD: `1c985d6f466a7b4f5c8f940744b62a8e7440bff2`.

Read the design specification, approved mockups and Stage 4A–4D, 5A–5E and 6A–6D reports. In particular, inspected plates 02/03/04/09/10 for taskbar, launcher, controls, auto-hide and expanded interaction; plates 05/06 and their implemented adaptations were also reviewed in the preceding refinement. Mockup timing, sensors, icons and controls remain requirements/illustrations, not runtime evidence. The original design assets and records are preserved.

Evidence has three distinct levels:

1. **Repository fact:** current source/lockfile/API inspection performed in Stage 7A.
2. **Historical observation:** attributable earlier reports, not rerun or promoted to wider acceptance.
3. **Proposal/unknown:** implementation design and future tests requiring approval; API availability alone does not establish compatibility or readiness.

| Repository evidence | Established boundary |
|---|---|
| `crates/sei-desk/src/main.rs:590–631` | One ordinary decorated management window, `iced::application`, Segoe UI, 1100×740 client/minimum 760×520, explicit close handling and owners dropped after event-loop exit. No production dock, launcher popup, AppBar or global shortcut. |
| `main.rs:79–176`, `493–555` | Shared reducer/subscriptions and local keyboard handling; `local_event` currently ignores the window ID argument. A second window therefore requires explicit routing/state ownership rather than copying the view and assuming isolation. |
| `applications.rs`, `applications/browser.rs`, `applications/runtime.rs`, `crates/sei-apps/src/consumer.rs` | Bounded catalog/query/projection, stable identity plus generation, explicit selection and captured launch request, cancellation, latest-state publication and owned service lanes. Reusable functionality, not a ready-made desktop launcher. |
| `applications/preferences.rs` and worker | Confirmed persisted favorite identities; view/sort/group/filter choices remain session-only. Launcher use must not fork or silently reinterpret this store. |
| `crates/sei-windows/src/lib.rs:38–110` | Foreground bookmark revalidates HWND/PID/thread and requests restoration only while our process owns foreground. Distinguishes skipped/denied/requested/observed. Same-process/thread HWND reuse remains a stated limit; not a general window switcher. Also exposes read-only motion preference, metrics and network. |
| Stage 4A–4D and 6A–6D | Actual management-window monitoring, catalog, favorites, keyboard and lifecycle work; Stage 6D reports 120 ordinary tests and 50 overlapping opt-in fixture tests plus scoped GUI observations. No new tests or runtime sessions were run in Stage 7A. These results do not validate overlay focus/fullscreen/DPI behavior. |
| Stage 5A–5E | Production semantic controls were absent in the recorded UIA inspection; C0 failed source qualification; owner retained iced/upstream-first and rejected C0 for integration. No provider experiment or alternate GUI was approved. Proposed review dates remain proposals. |
| `experiments/windows-integration-spike/README.md`, `src/native.rs`; Stage 1 report | Standalone Win32 harness, excluded from production workspace and without iced. Historical native queries, owned observer cleanup and limited probes do not prove a production overlay. AppBar registration was not admitted in the reported environment; Explorer restart, AppBar crash recovery, full-screen classification and mixed-DPI transitions were not qualified. No spike was executed here. |

### Exact locked iced source examined

The lock contains iced/iced_core/iced_runtime **0.14.0**, iced_widget **0.14.2**, iced_winit **0.14.1** and winit **0.30.13**. Root features are existing tiny-skia/thread-pool/crisp/advanced. Read locally cached published source under `C:/Users/swani/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`; no dependency resolution or cache changes.

| Source, symbol and approximate line range | What source proves / does not prove |
|---|---|
| `iced_core-0.14.0/src/window/settings.rs:31–103`, `Settings` | Size, position, visibility, decorations, transparency, level and close policy exist. Its `blur` documentation explicitly limits support to macOS/Linux: Windows Acrylic/Mica is not established. |
| `iced_core-0.14.0/src/window/settings/windows.rs:5–48`, `PlatformSpecific` | Skip-taskbar, undecorated shadow and corner preference exist. These concern the app's own window, not hiding the Windows taskbar. No exposed no-activate field here. |
| `iced_runtime-0.14.0/src/window.rs:196`, `Window`; `272–284`, open/close; `376–436`, scale/move/focus/level; `462–499`, run/passthrough | Window-ID tasks, native handle-bearing callback and mouse pass-through APIs exist. `raw_id` is an opaque window ID, **not proof of an HWND**; use a validated `HasWindowHandle` path if native integration is later approved. Requests are not observed OS success. |
| `iced-0.14.0/src/daemon.rs:16–35`, `daemon` | Multi-window/background lifecycle option exists, opens no window by default and needs explicit exit when all windows close. Do not introduce a resident process accidentally; a daemon conversion is not automatically necessary or approved. |
| `iced_winit-0.14.1/src/conversion.rs:38–61, 98–103, 347–368`; `window/state.rs:154`; `lib.rs:1605–1637` | Settings are mapped to winit; move/rescale events and focus/level/run actions are implemented. A focus action calls `focus_window`; `Run` looks up the live window before callback. This is a supported inspection boundary, not permission to persist an HWND or subclass unsafely. |
| `iced_core-0.14.0/src/window/event.rs:8–46` | Opened/moved/resized/rescaled/focused/unfocused events exist. Current production reducer does not implement overlay-specific responses to all of them. |

## 3. Separate feature assessments

### A. Top quick-control dock

**Drawing/window — feasible.** Borderless positioning and opaque slate rendering fit existing iced. Normal versus topmost Z-order is a deliberate policy; `SetWindowPos` distinguishes no-activate positioning from topmost placement. Transparency is not a hit-test or blur guarantee. Windows no-activate styles have important keyboard/assistive-technology implications and cannot simply be applied to interactive controls. ([SetWindowPos](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos), [Extended Window Styles](https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles))

**Auto-hide/global edge reveal — unproven.** The specification requires a hidden one-physical-pixel sensor, no click interception, 180 ms reveal/450 ms dismissal, pointer bridges, drag/keyboard/confirmation holds and reduced-motion fallback. Local hover events only cover a receiving window: `TrackMouseEvent` supplies hover/leave notifications, not a global hidden-edge sensor. `GetCursorPos` supplies screen coordinates subject to desktop/access constraints. Bounded sensing, invalidation and drag suppression would need separate proof; no permanent transparent input strip or unbounded polling is justified. ([TrackMouseEvent](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackmouseevent), [GetCursorPos](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorpos))

**Pointer/focus — feasible with constraints for explicit use; passive reveal unproven.** Passive show must preserve the external editor's foreground; deliberate control entry may acquire focus. An active keyboard session or captured slider must suspend auto-hide. Unresolved power confirmation must not time out or replay after restart. Focus-loss alone cannot distinguish an external click, owned popover and IME candidate UI. Source/API presence does not resolve this boundary. `SetForegroundWindow` can be denied even when documented eligibility appears satisfied; no repeated focus-forcing workaround is proposed. ([SetForegroundWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow))

**Placement/display/DPI — feasible with constraints, untested in production.** Use monitor-local work/monitor rectangles and per-window scaling; distinguish physical edge coordinates from iced DIP. `MonitorFromWindow`, `GetMonitorInfoW` and `WM_DPICHANGED` provide relevant primitives. Negative origins, stacked screens, rotation, primary swaps and removal must invalidate geometry before reuse. Winit rescaling does not itself re-anchor a dock/sensor. Primary-monitor-only first dock is a proposed product bound, not permission to ignore disconnection. ([MonitorFromWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromwindow), [GetMonitorInfoW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmonitorinfow), [WM_DPICHANGED](https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged))

**Hardware/session controls — separate capabilities, not one readiness claim:**

| Control | API evidence and practical limit | Verdict / missing production evidence |
|---|---|---|
| Volume/mute | `IAudioEndpointVolume` controls endpoint master volume/mute; this affects other audio apps, and software controls differ in exclusive mode. Device replacement and callbacks require owned lifetime, coalescing and truthful confirmed state. | **feasible with constraints**; no production endpoint selection, COM/notification lifecycle or hardware acceptance. [Official interface](https://learn.microsoft.com/en-us/windows/win32/api/endpointvolume/nn-endpointvolume-iaudioendpointvolume). |
| Built-in brightness | WMI `WmiSetBrightness` takes percent and timeout and reports success/error. A compatible provider/device and capability discovery are prerequisites; no universal display coverage inferred. | **feasible with constraints**; no production provider/device evidence. [Official method](https://learn.microsoft.com/en-us/windows/win32/wmicoreprov/wmisetbrightness-method-in-class-wmimonitorbrightnessmethods). |
| External brightness | `SetMonitorBrightness` operates on physical monitor handles. Microsoft warns that arbitrary monitors may not implement MCCS correctly and calls for physical validation. | **unproven** for intended hardware; unsupported devices must remain explicitly unavailable. No generic enabled slider. [Official API and warning](https://learn.microsoft.com/en-us/windows/win32/api/highlevelmonitorconfigurationapi/nf-highlevelmonitorconfigurationapi-setmonitorbrightness). |
| Lock | `LockWorkStation` is an asynchronous request, restricted to an interactive desktop; success does not establish completed lock. | **feasible with constraints**, explicit action only; no call performed. [Official API](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-lockworkstation). |
| Sleep/hibernate | `SetSuspendState` needs shutdown privilege and supported system state; errors must be surfaced. | **feasible with constraints**, separate capability/privilege and confirmation policy; no call performed. [Official API](https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-setsuspendstate). |
| Restart/shutdown/sign out | `ExitWindowsEx` distinguishes logoff and machine shutdown; shutdown/restart require an enabled shutdown privilege. Acceptance is not completed shutdown; force flags risk data loss. | **feasible with constraints**, explicit per-action confirmation, no automatic escalation/force/retry/replay. No call performed. [Official API](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-exitwindowsex). |
| Status | Existing battery/network/clock can be projected without inventing readings. New audio/device status needs a real source. Network throughput is not internet connectivity. | **feasible** for already collected status; no extra metric is approved. |

**Maximized/fullscreen — unproven as a combined policy.** Maximized work-area use must differ from borderless/exclusive fullscreen. DWM attributes and monitor bounds can inform a classifier; `SHQueryUserNotificationState` returns notification suitability, including busy/presentation/exclusive-D3D states, not an authoritative per-monitor overlay license. Unknown or transitioning state should suppress automatic reveal, never queue a surprise opening. Ordinary overlays cannot promise visibility over exclusive fullscreen or secure desktops. ([DwmGetWindowAttribute](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmgetwindowattribute), [SHQueryUserNotificationState](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shqueryusernotificationstate), [notification-state enum](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ne-shellapi-query_user_notification_state))

**Overall dock verdict: unproven.** Keep its approved design; defer automatic sensing and system writes pending independent acceptance. An explicit-only companion would meet less of the dock brief but reduce risk; it is not the first recommended milestone below.

### B. Keyboard-first launcher

**Explicit popup/search/launch — feasible with constraints.** Existing text editing, query bounds, generation/identity checks, failure feedback, pins and Consumer can be reused. A single ordinary focusable borderless iced window can present them. The approved dark launcher plate is visual authority for that surface; management windows stay unchanged. No packaged-app, icon, file/web search or universal installed-app coverage is inferred from the existing supported-shortcut catalog.

**Trigger/native Start — constrained.** Begin with an explicit management-window command. A future optional non-Windows hotkey could use `RegisterHotKey`, with registration conflict/error handling, `MOD_NOREPEAT` and matched unregistration. Windows-logo shortcuts are OS-reserved; registration is not evidence that bare Windows-key replacement works. No low-level keyboard interception, synthetic Start toggling, Start dismissal or policy change is recommended. Native Start remains available. ([RegisterHotKey](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey))

**Focus/restoration — constrained.** Request focus only following deliberate invocation; observe the outcome. If denied, retain the existing manager route rather than repeatedly raising a topmost window. The current bookmark is a narrow starting point for dismissal, not a guarantee of returning to another app's text caret. Never restore on external focus loss or after a successful launch, which could pull focus away from the launched app. Foreign HWND validity/PID/thread must be rechecked; reuse remains a limit. The first milestone invokes from the already-foreground manager and returns only to that live manager, avoiding a promise of arbitrary foreign restoration. ([SetForegroundWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow))

**Missing boundary:** window-ID-scoped events, focus, size, query/selection projection, widget IDs and close semantics. Current global `Message::Close`, width and ignored event window ID cannot be assumed safe for two windows. A shared bounded launch authority must resolve the popup's exact ID/generation at submission; no selection-index handoff or duplicated unbounded coordinator. IME loss-of-focus/candidate interaction and semantic accessibility are still unqualified. **Overall: feasible with constraints**, suitable for a separately approved internal milestone; production accessibility/release remains blocked.

### C. Custom bottom taskbar

**AppBar/work area — API path feasible; production unproven.** Registration/negotiation uses `ABM_NEW`, `ABM_QUERYPOS`, `ABM_SETPOS`, callback processing and `ABM_REMOVE`. Approved rectangles coexist with taskbar/other appbars; reservation is not permission to replace them. Display and position/fullscreen notifications need lifecycle handling. No assumption of automatic crash cleanup follows. ([Application Desktop Toolbars](https://learn.microsoft.com/en-us/windows/win32/shell/application-desktop-toolbars))

**Running windows/grouping/activation — constrained and incomplete.** `EnumWindows` enumerates top-level desktop-app windows, not a complete semantic model of apps/tabs/virtual desktops. Eligibility, ownership, tool/cloaked windows, lifetime races and minimized state need policy and tests. AppUserModelID is relevant to grouping; executable path alone is not shell-equivalent identity. Foreground restrictions still apply to activation. Existing catalog IDs identify launch targets, not running-window groups. WinEvent callbacks need a message loop, careful lifetime/reentrancy and bounded coalescing. ([EnumWindows](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumwindows), [Taskbar Extensions](https://learn.microsoft.com/en-us/windows/win32/shell/taskbar-extensions), [SetWinEventHook](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwineventhook), [SetForegroundWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow))

**Tray compatibility — replacement not established.** `Shell_NotifyIconW` and notification-area documentation describe an app managing its own icon in the shell's notification area. They do not establish a supported third-party tray-rehosting implementation for Sei-Desk. A Sei-Desk status strip/owned tray icon cannot claim compatibility with other applications' tray menus, notifications or accessibility. Keep Explorer's tray available. ([Shell_NotifyIconW](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shell_notifyiconw), [Notification Area](https://learn.microsoft.com/en-us/windows/win32/shell/notification-area))

**Explorer/crash recovery — unproven.** `TaskbarCreated` is a broadcast to top-level windows; owned icons may need rebuilding. It is not evidence that a message-only receiver or replayed notification reproduces an actual Explorer restart. Reconcile current resources with bounded retry/backoff, not stale queued commands. Process termination can bypass application cleanup, so Drop/normal exit is insufficient proof of crash recovery. Earlier observer-child termination did not include AppBar resources. ([The Taskbar — creation notification](https://learn.microsoft.com/en-us/windows/win32/shell/taskbar), [Terminating a Process](https://learn.microsoft.com/en-us/windows/win32/procthread/terminating-a-process))

**Overall: native-taskbar replacement not recommended; additive bar unproven.** No Explorer manipulation, native-taskbar hiding, `ABM_SETSTATE`, shell replacement or manual work-area repair is proposed. Even a future additive bar needs a separate recovery qualification before any reservation ships.

## 4. Strategy comparison

| Approach | Reliability / complexity | Accessibility | Multi-monitor / recovery | Requirements left unmet |
|---|---|---|---|---|
| 1. Ordinary borderless iced windows | Lowest shell coupling; moderate window/focus routing work. Avoids claiming automatic desktop behavior from visual placement. | Existing blocker remains; borderless controls need named close/dismiss and semantic focus paths. | Work-area clamping/rescale still required. Closing/crashing removes only owned windows; no native-taskbar state to restore. | Work-area reservation, running-window shell behavior, global edge reveal, third-party tray; hardware controls need separate work. |
| 2. Limited companion dock + explicit launcher, native taskbar intact | Best incremental product strategy. Launcher can lead; dock/control work staged independently. Bounded resource ownership and manager fallback reduce failure surface. | Same blocker, with separate platform/operator gates before production acceptance. No provider change implied. | Initially one launcher on invocation monitor; later dock primary-only if approved. Failure leaves Start/taskbar/tray available; own optional registrations would require cleanup. | Full custom taskbar, bare Windows-key replacement, tray rehosting and universal fullscreen/brightness guarantees. |
| 3. Custom AppBar/taskbar with recovery path | Highest complexity and qualification burden: callbacks, reservation, window inventory/grouping, shell restart and crash handling. Native taskbar must still remain available. | Adds essential running-window/tray routes to an already blocked semantic surface. | Per-monitor reservation/topology tests and independent cleanup procedure essential. Normal-exit success insufficient. | Supported third-party tray rehosting and shell parity not established; native-taskbar suppression remains excluded. |

**Choose strategy 2 as the direction, but implement only its launcher slice first if approved.** This preserves the original product goal while exposing the smallest new Windows lifecycle boundary. It is a recommendation, not a retroactive approval of earlier Stage 5C choices.

## 5. One bounded next milestone — proposal only

**Name:** Explicit launcher companion, native taskbar retained. **Audience:** internal engineering acceptance while accessibility remains blocked. No deadline or new stage is scheduled.

### Included

- One optional focusable, opaque, borderless iced launcher opened from a named keyboard-reachable command in the existing foreground manager. At most one popup plus the manager; repeated invocation reuses it, never spawns a window/worker per key.
- Reuse supported catalog, bounded query, visible selection, shared confirmed favorites and exact generation/identity launch contract. Keep management behavior and session-only presentation policy unchanged. Do not fork the services or infer a new target from display names.
- Compact application-only search, existing reading-order Up/Down, Enter policy, no-match/loading/error feedback and a visible named dismiss action. Popup Escape dismisses; manager Escape continues its existing query-clear behavior. Failure retains popup query/selection; process acceptance may dismiss without claiming a visible target window or restoring manager focus over it.
- Window-ID-scoped events, layout/focus/widget operations, bounded service delivery and explicit lifetime ownership. Manager closes as today: exit the process and join owned workers; popup close only closes the popup. No hidden resident process.
- Place on the manager's invocation monitor inside its current work area; handle rescale/removal before showing. Keep ordinary Z-order, no global sensor or always-on-top demand. If safe placement/focus cannot be established, keep the manager usable and report failure rather than leave an unreachable window.

### Excluded

Dock/auto-hide/sensors, hardware writes, power/session actions, global hotkey, Windows-key interception, Start manipulation, tray registration/rehosting, AppBar/reservation, running-window taskbar, autostart/resident mode, transparency/blur promises, new dependencies and provider experiments. No replacement mockups or management redesign. No release/accessibility qualification by milestone completion alone.

### Required acceptance evidence

| Gate / matrix | Future acceptance requirement (not run in 7A) |
|---|---|
| Window routing and reuse | Open/reinvoke/dismiss repeatedly; events and delayed tasks for a closed popup cannot change manager focus/width/query or a replacement popup. Maximum two windows and existing bounded workers/queues; no detached work. |
| Focus | Invoke from foreground manager, minimized/closing manager, denied focus request and rapid Alt+Tab. Observe actual foreground; never force retries. Dismiss returns only to the still-live manager if appropriate; external focus loss does not steal focus back. Test owned menus and real IME candidates separately. |
| Input and identity | Plain/modified/repeated Enter, empty/no-match, changed/removed entry, refresh and save receipts while popup open. Exactly current valid selection launches once; no unintended launch from stale widget or manager selection. Retain Stage 6B real-widget regression. |
| Display changes | Owner-approved OS/build matrix; 100/125/150/175/200% scaling, mixed DPI, negative origins, stacked monitors, rotation, primary swap and disconnect/reconnect. No offscreen controls; invalid geometry suppresses show until recomputed. |
| Maximized/fullscreen | Maximized editor/browser, browser F11, borderless/exclusive applications and presentation on same/other monitor. No automatic opening or topmost interference. If another app takes foreground/fullscreen, hide/dismiss safely; require fresh deliberate invocation after return. Notification/geometry heuristics are not an acceptance substitute. |
| Secure/session boundary | Lock/unlock, UAC/secure desktop and session changes: no overlay above secure desktop, stale reopen, queued launch or forced focus on return. No setting change or secure-desktop automation to bypass restrictions. |
| Explorer | Milestone has no shell-owned registration to repair. In a separately approved controlled session, actual Explorer restart must leave manager/popup dismissible, native Start/taskbar available after Explorer recovers and no commands replayed. Synthetic `TaskbarCreated` alone is not acceptance. |
| Failure/shutdown | Window-create failure, missing monitor, service/save failure, popup closed during pending work, manager exit and forced termination in an isolated test. Preserve pins/last good catalog, bound pending tasks, join accepted work on normal exit and never require Sei-Desk to restore native taskbar state. |
| Accessibility/platform | Existing semantic/UIA/editor/label/container-scroll gates, human Narrator, physical keyboard, real IME and relevant contrast/DPI testing remain separate production gates. No C0 integration or acceptance waiver. Internal reliability acceptance cannot close them. |

These are future test categories, not a request to run Explorer/fullscreen/security experiments now. A scope-limited result must record unsupported configurations explicitly.

### Prerequisites and owner decisions required

1. Approve or reject the **launcher-only internal milestone**, its explicit manager trigger, exclusions and manager-exit behavior. Approval is not recorded here; no global trigger or resident dock is bundled in.
2. Confirm the supported Windows versions/builds and initial monitor scope. Prior NT build observations do not establish a marketing edition or a qualified OS allowlist.
3. Approve the concrete window-routing/lifecycle and acceptance plan before implementation. Audit the exact native-handle, work-area and focus path against the locked graph; `raw_id` must not be cast to HWND. If necessary APIs/features are unavailable, stop for separate dependency/feature approval rather than adding bindings implicitly.
4. Confirm presentation details within approved plate 03, including discoverable dismissal and text-first icon fallback. Preserve approved artwork; no design replacement is implied.
5. Keep accessibility BLOCKED and upstream-first/C0 rejection intact. Any accessibility candidate or provider work still needs its own passing preflight and explicit scope authorization; no native-controls fallback is initiated.
6. Decide later, separately, whether dock controls/auto-hide or an additive AppBar merit investment. No budget, person, staffing, support commitment, date or reminder is assigned.

### Rollback and safe shutdown

Proposed rollback is **remove/disable the optional popup invocation and close/terminate only Sei-Desk**; existing manager behavior and the unchanged native Windows Start/taskbar remain the access path. No native taskbar setting, Explorer state, work-area reservation, global registration or pending system write is changed by this milestone, so restoring them cannot depend on a functioning Sei-Desk UI. Closing the popup never undoes user launches. Restart begins with popup closed and no replayed action or remembered foreign focus target.

For a future additive AppBar, normal same-thread unregister-before-destroy and independently verified abnormal-termination/work-area recovery would be prerequisites; merely promising a recovery button in that bar is unacceptable. A broader taskbar replacement has no approved rollback design and remains not recommended.

## 6. Exact official documentation consulted

Read-only web inspection on 2026-10-17. These are API contracts, not Sei-Desk runtime results. Some Microsoft pages include older-shell examples; their existence is not a Windows 11 parity claim. Only official Microsoft documentation was used for Windows claims.

| Source | Inspected contract |
|---|---|
| [Application Desktop Toolbars](https://learn.microsoft.com/en-us/windows/win32/shell/application-desktop-toolbars) | AppBar registration, position negotiation, notifications, removal. |
| [SetForegroundWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow) | Foreground eligibility, denial, permission handoff. |
| [WM_DPICHANGED](https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged) | Per-window DPI transition and suggested rectangle. |
| [Notifications and the Notification Area](https://learn.microsoft.com/en-us/windows/win32/shell/notification-area) | Application-owned notification UI. |
| [RegisterHotKey](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey) | Delivery, conflicts, no-repeat, OS-reserved Windows shortcuts and cleanup. |
| [EnumWindows](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumwindows) | Top-level desktop-app window enumeration. |
| [SetWindowPos](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos) | Position/Z-order/no-activation requests. |
| [GetMonitorInfoW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmonitorinfow) | Monitor information query. |
| [SHQueryUserNotificationState](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shqueryusernotificationstate) | Notification suitability query. |
| [Taskbar Extensions](https://learn.microsoft.com/en-us/windows/win32/shell/taskbar-extensions) | Group identity and shell taskbar functionality beyond a bar window. |
| [IAudioEndpointVolume](https://learn.microsoft.com/en-us/windows/win32/api/endpointvolume/nn-endpointvolume-iaudioendpointvolume) | Endpoint volume/mute and exclusive/shared-mode limits. |
| [WmiSetBrightness](https://learn.microsoft.com/en-us/windows/win32/wmicoreprov/wmisetbrightness-method-in-class-wmimonitorbrightnessmethods) | Brightness method parameters/result. |
| [SetMonitorBrightness](https://learn.microsoft.com/en-us/windows/win32/api/highlevelmonitorconfigurationapi/nf-highlevelmonitorconfigurationapi-setmonitorbrightness) | Physical-monitor brightness and compatibility warning. |
| [ExitWindowsEx](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-exitwindowsex) | Session/power request, privilege and force risks. |
| [LockWorkStation](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-lockworkstation) | Interactive asynchronous lock request. |
| [SetSuspendState](https://learn.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-setsuspendstate) | Sleep/hibernate request and privilege. |
| [Extended Window Styles](https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles) | No-activate/tool-window/topmost behavior. |
| [TrackMouseEvent](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackmouseevent) | Window hover/leave tracking. |
| [GetCursorPos](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorpos) | Desktop cursor coordinate/access contract. |
| [SetWinEventHook](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwineventhook) | Event delivery thread/message-loop/reentrancy. |
| [Shell_NotifyIconW](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shell_notifyiconw) | Manage an app's own notification icon. |
| [The Taskbar](https://learn.microsoft.com/en-us/windows/win32/shell/taskbar) | Own taskbar participation and TaskbarCreated broadcast. |
| [QUERY_USER_NOTIFICATION_STATE](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ne-shellapi-query_user_notification_state) | Busy/presentation/exclusive-D3D notification states. |
| [DwmGetWindowAttribute](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmgetwindowattribute) | DWM attribute query boundary. |
| [MonitorFromWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromwindow) | Window-to-monitor association. |
| [Terminating a Process](https://learn.microsoft.com/en-us/windows/win32/procthread/terminating-a-process) | Termination differs from normal cleanup. |

## 7. Integrity, validation and Git status

Only this document is added. Before/after SHA256 inventory covers **731 pre-existing tracked/nonignored untracked files**; all remain unchanged. Entry/final short status below preserves all existing work; final status adds only the Stage 7A report. Inventory and status are ignored inspection artifacts under `target/stage7a-baseline/`.

`git diff --check` and a separate new-report trailing-whitespace check passed. Source inspection initially guessed an absent cached `program.rs`; directory search located the actual `iced_winit/src/lib.rs` implementation. No build/test, dependency resolution, archive download, installation, runtime/UIA/IME session, prototype, shell operation or automation was run. Official web pages were read only. No source, manifest, lockfile, test, script, historical evidence, acceptance gate or approved asset changed. No stage/commit/push/reset/clean/restore/stash.

Starting status is the following list **without** the Stage 7A line; final `git status --short` includes it:

```text
 M README.md
 M crates/sei-desk/src/applications.rs
 M crates/sei-desk/src/applications/preferences.rs
 M crates/sei-desk/src/applications/preferences/tests.rs
 M crates/sei-desk/src/main.rs
 M crates/sei-desk/src/traffic_chart.rs
 M crates/sei-desk/src/view.rs
 M tools/Test-Stage2A.ps1
?? crates/sei-desk/src/applications/search.rs
?? docs/implementation/STAGE-5A-STABILIZATION-ACCEPTANCE.md
?? docs/implementation/STAGE-5B-ACCESSIBILITY-ARCHITECTURE-DECISION.md
?? docs/implementation/STAGE-5C-OWNER-DECISION-PACKET.md
?? docs/implementation/STAGE-5D-ACCESSIBILITY-CANDIDATE-PREFLIGHT.md
?? docs/implementation/STAGE-5E-UPSTREAM-FIRST-OWNER-DECISION.md
?? docs/implementation/STAGE-6A-DASHBOARD-RELIABILITY-REPORT.md
?? docs/implementation/STAGE-6B-LAUNCHER-ACCEPTANCE.md
?? docs/implementation/STAGE-6C-VISUAL-REFINEMENT-REPORT.md
?? docs/implementation/STAGE-6D-END-TO-END-ACCEPTANCE.md
?? docs/implementation/STAGE-7A-DESKTOP-FEASIBILITY.md
?? docs/implementation/evidence-stage5a/
?? docs/implementation/evidence-stage5b/
```

**Documentation-only assessment complete. Await owner review; no implementation or further experiment is authorized by this report.**
