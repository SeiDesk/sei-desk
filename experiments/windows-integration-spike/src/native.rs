//! All Win32 calls stay here. This process owns only its test HWND, timer,
//! optional WinEvent hooks, raw-mouse registration, hotkey, tray icon and AppBar.
//! Event/window callbacks never unwind or allocate; they record counts or signal exit.
use std::{
    io,
    mem::{size_of, zeroed},
    ptr::{null, null_mut},
    sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering::Relaxed},
    time::Instant,
};
use windows_sys::{
    Win32::{
        Foundation::*,
        Graphics::{Dwm::*, Gdi::*},
        Security::*,
        System::{Console::*, LibraryLoader::GetModuleHandleW, ProcessStatus::*, Threading::*},
        UI::{
            Accessibility::*, HiDpi::*, Input::KeyboardAndMouse::*, Input::*, Shell::*,
            WindowsAndMessaging::*,
        },
    },
    core::BOOL,
};

const APPBAR_CALLBACK: u32 = WM_APP + 1;
const TRAY_CALLBACK: u32 = WM_APP + 2;
static EVENTS: AtomicU64 = AtomicU64::new(0);
static RAW_MESSAGES: AtomicU64 = AtomicU64::new(0);
static HOTKEY_MESSAGES: AtomicU64 = AtomicU64::new(0);
static TRAY_MESSAGES: AtomicU64 = AtomicU64::new(0);
static TOPOLOGY_MESSAGES: AtomicU64 = AtomicU64::new(0);
static CANCEL_APPBAR: AtomicBool = AtomicBool::new(false);
static APPBAR_READY: AtomicBool = AtomicBool::new(false);
static MESSAGE_THREAD: AtomicU32 = AtomicU32::new(0);

unsafe extern "system" fn console_handler(event: u32) -> BOOL {
    if event != CTRL_C_EVENT && event != CTRL_BREAK_EVENT {
        return 0;
    }
    let thread = MESSAGE_THREAD.load(Relaxed);
    if thread == 0 {
        return 0;
    }
    // SAFETY: Same-process thread id; its message queue exists before registration.
    unsafe { PostThreadMessageW(thread, WM_QUIT, 0, 0) }
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(Some(0)).collect()
}
fn last_error(label: &str) -> String {
    format!("{label}: {}", io::Error::last_os_error())
}
fn rect(value: RECT) -> (i32, i32, i32, i32) {
    (value.left, value.top, value.right, value.bottom)
}
fn same_rect(a: RECT, b: RECT) -> bool {
    rect(a) == rect(b)
}

/// Physical coordinates; half-open x interval and exactly one pixel at top.
fn on_edge(point: POINT, monitor: RECT, width: i32) -> bool {
    let width = width.clamp(0, monitor.right - monitor.left);
    let left = monitor.left + (monitor.right - monitor.left - width) / 2;
    point.y == monitor.top && point.x >= left && point.x < left + width
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => {
            // SAFETY: HWND belongs to this callback's thread; cleanup is owned by Session.
            unsafe { PostQuitMessage(0) };
            0
        }
        WM_HOTKEY => {
            HOTKEY_MESSAGES.fetch_add(1, Relaxed);
            0
        }
        WM_INPUT => {
            RAW_MESSAGES.fetch_add(1, Relaxed);
            // SAFETY: Windows supplies a live HRAWINPUT for this callback. We read only
            // its fixed-size header, with an aligned stack buffer, before default cleanup.
            // No button values, device IDs or pointer movement payloads are retained.
            unsafe {
                let mut header: RAWINPUTHEADER = zeroed();
                let mut size = size_of::<RAWINPUTHEADER>() as u32;
                GetRawInputData(
                    l as HRAWINPUT,
                    RID_HEADER,
                    &mut header as *mut _ as _,
                    &mut size,
                    size_of::<RAWINPUTHEADER>() as u32,
                );
            }
            // SAFETY: Forward the unchanged message so Windows performs raw-input cleanup.
            unsafe { DefWindowProcW(hwnd, msg, w, l) }
        }
        WM_DISPLAYCHANGE | WM_DPICHANGED | WM_SETTINGCHANGE => {
            TOPOLOGY_MESSAGES.fetch_add(1, Relaxed);
            CANCEL_APPBAR.store(true, Relaxed);
            // SAFETY: Forward unchanged Windows message, no pointers dereferenced here.
            unsafe { DefWindowProcW(hwnd, msg, w, l) }
        }
        APPBAR_CALLBACK => {
            // This minimal probe cancels on external shell geometry/fullscreen changes.
            // It deliberately does not attempt production relayout or re-registration.
            if APPBAR_READY.load(Relaxed)
                && (w == ABN_POSCHANGED as usize || w == ABN_FULLSCREENAPP as usize)
            {
                CANCEL_APPBAR.store(true, Relaxed);
            }
            0
        }
        TRAY_CALLBACK => {
            TRAY_MESSAGES.fetch_add(1, Relaxed);
            0
        }
        _ => {
            // SAFETY: Default handling with original parameters from Windows.
            unsafe { DefWindowProcW(hwnd, msg, w, l) }
        }
    }
}

unsafe extern "system" fn event_proc(
    _: HWINEVENTHOOK,
    _: u32,
    _: HWND,
    _: i32,
    _: i32,
    _: u32,
    _: u32,
) {
    EVENTS.fetch_add(1, Relaxed);
}

unsafe extern "system" fn monitor_proc(
    monitor: HMONITOR,
    _: HDC,
    _: *mut RECT,
    data: LPARAM,
) -> BOOL {
    // SAFETY: EnumDisplayMonitors is synchronous; data is the live caller-owned Vec.
    unsafe {
        let output = &mut *(data as *mut Vec<MONITORINFO>);
        if output.len() == output.capacity() {
            return 0;
        }
        let mut info: MONITORINFO = zeroed();
        info.cbSize = size_of::<MONITORINFO>() as u32;
        if GetMonitorInfoW(monitor, &mut info) != 0 {
            output.push(info);
        }
    }
    1
}

fn monitors() -> Result<Vec<MONITORINFO>, String> {
    // Preallocate so the FFI callback never allocates; more than 64 monitors fails closed.
    let mut output = Vec::with_capacity(64);
    // SAFETY: Callback is synchronous; pointer remains valid until enumeration returns.
    if unsafe {
        EnumDisplayMonitors(
            null_mut(),
            null(),
            Some(monitor_proc),
            &mut output as *mut _ as isize,
        )
    } == 0
    {
        return Err(last_error("EnumDisplayMonitors"));
    }
    if output.is_empty() {
        return Err("No monitor accessible".into());
    }
    Ok(output)
}

unsafe extern "system" fn enum_window(hwnd: HWND, data: LPARAM) -> BOOL {
    // SAFETY: Synchronous enumeration; data points to the caller's counters.
    unsafe {
        let counts = &mut *(data as *mut [u64; 4]);
        counts[0] += 1;
        if IsWindowVisible(hwnd) == 0 {
            return 1;
        }
        let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
        if ex & WS_EX_TOOLWINDOW != 0 || !GetWindow(hwnd, GW_OWNER).is_null() {
            return 1;
        }
        let mut cloaked: u32 = 0;
        if DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED as u32,
            &mut cloaked as *mut _ as _,
            size_of::<u32>() as u32,
        ) < 0
        {
            counts[3] += 1; // Unknown; do not silently classify as uncloaked.
            return 1;
        }
        if cloaked != 0 {
            return 1;
        }
        counts[1] += 1;
        if IsIconic(hwnd) != 0 {
            counts[2] += 1;
        }
    }
    1
}

fn snapshot() -> Result<(), String> {
    let monitors = monitors()?;
    for (index, info) in monitors.iter().enumerate() {
        println!(
            "monitor_{index}: physical={:?} work={:?} primary={}",
            rect(info.rcMonitor),
            rect(info.rcWork),
            info.dwFlags & MONITORINFOF_PRIMARY != 0
        );
    }
    // SAFETY: Sized output buffers remain live for synchronous query calls.
    unsafe {
        let taskbar = FindWindowW(wide("Shell_TrayWnd").as_ptr(), null());
        println!(
            "spike_window_found={}",
            !FindWindowW(wide("SeiDesk.Stage1.BoundedProbe").as_ptr(), null()).is_null()
        );
        println!(
            "native_taskbar: found={} visible={}",
            !taskbar.is_null(),
            !taskbar.is_null() && IsWindowVisible(taskbar) != 0
        );
        let mut counts = [0u64; 4];
        if EnumWindows(Some(enum_window), &mut counts as *mut _ as isize) == 0 {
            return Err(last_error("EnumWindows"));
        }
        println!(
            "windows: total={} eligible_heuristic={} minimized={} unknown_cloak={}",
            counts[0], counts[1], counts[2], counts[3]
        );
        let foreground = GetForegroundWindow();
        let mut bounds: RECT = zeroed();
        let bounds_ok = !foreground.is_null() && GetWindowRect(foreground, &mut bounds) != 0;
        let covers_monitor = bounds_ok
            && monitors.iter().any(|m| {
                bounds.left <= m.rcMonitor.left
                    && bounds.top <= m.rcMonitor.top
                    && bounds.right >= m.rcMonitor.right
                    && bounds.bottom >= m.rcMonitor.bottom
            });
        println!(
            "foreground: available={} maximized={} covers_monitor_heuristic={covers_monitor}",
            !foreground.is_null(),
            !foreground.is_null() && IsZoomed(foreground) != 0
        );
        let mut state = 0;
        let hr = SHQueryUserNotificationState(&mut state);
        println!("notification_state: hr={hr} state={state} (not a fullscreen oracle)");
        let mut animate: BOOL = 0;
        let ok = SystemParametersInfoW(
            SPI_GETCLIENTAREAANIMATION,
            0,
            &mut animate as *mut _ as _,
            0,
        );
        println!(
            "client_area_animation: query_ok={} enabled={}",
            ok != 0,
            animate != 0
        );
        let mut token = null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return Err(last_error("OpenProcessToken"));
        }
        let mut elevation: TOKEN_ELEVATION = zeroed();
        let mut length = 0;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            &mut elevation as *mut _ as _,
            size_of::<TOKEN_ELEVATION>() as u32,
            &mut length,
        );
        CloseHandle(token);
        if ok == 0 {
            return Err(last_error("GetTokenInformation"));
        }
        println!("token_elevated={}", elevation.TokenIsElevated != 0);
    }
    Ok(())
}

/// One owner on the native message-loop thread. Drop is normal/unwind cleanup;
/// it does not run after abort, TerminateProcess, power loss or process kill.
struct Session {
    hwnd: HWND,
    class: Vec<u16>,
    hooks: Vec<HWINEVENTHOOK>,
    raw: bool,
    hotkey: bool,
    tray: bool,
    appbar: bool,
    console: bool,
}
impl Session {
    fn new(primary: &MONITORINFO) -> Result<Self, String> {
        let class = wide("SeiDesk.Stage1.BoundedProbe");
        let title = wide("SEI-DESK STAGE 1 TEMPORARY TEST — closes automatically");
        // SAFETY: Class/title UTF-16 buffers live through calls; instance is current module.
        let hwnd = unsafe {
            let instance = GetModuleHandleW(null());
            let wc = WNDCLASSW {
                lpfnWndProc: Some(window_proc),
                hInstance: instance,
                lpszClassName: class.as_ptr(),
                hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
                ..zeroed()
            };
            if RegisterClassW(&wc) == 0 {
                return Err(last_error("RegisterClassW"));
            }
            let hwnd = CreateWindowExW(
                WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
                class.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU,
                primary.rcMonitor.left + 12,
                primary.rcMonitor.top + 12,
                640,
                80,
                null_mut(),
                null_mut(),
                instance,
                null(),
            );
            if hwnd.is_null() {
                UnregisterClassW(class.as_ptr(), instance);
                return Err(last_error("CreateWindowExW"));
            }
            hwnd
        };
        let mut session = Self {
            hwnd,
            class,
            hooks: Vec::new(),
            raw: false,
            hotkey: false,
            tray: false,
            appbar: false,
            console: false,
        };
        // SAFETY: A created HWND establishes the thread message queue. Ctrl+C posts quit,
        // never destroys HWNDs from the console-handler thread; Drop removes the handler.
        unsafe {
            MESSAGE_THREAD.store(GetCurrentThreadId(), Relaxed);
            if SetConsoleCtrlHandler(Some(console_handler), 1) == 0 {
                return Err(last_error("SetConsoleCtrlHandler"));
            }
        }
        session.console = true;
        Ok(session)
    }
    fn appbar_data(&self) -> APPBARDATA {
        // SAFETY: Zero is a valid initial APPBARDATA; required size/HWND set here.
        APPBARDATA {
            cbSize: size_of::<APPBARDATA>() as u32,
            hWnd: self.hwnd,
            ..unsafe { zeroed() }
        }
    }
    fn tray_data(&self) -> NOTIFYICONDATAW {
        // SAFETY: Zeroed optional fields are valid; size, identity and flags set by caller.
        NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: self.hwnd,
            uID: 1,
            ..unsafe { zeroed() }
        }
    }
    fn configure(&mut self, mode: &str, primary: &MONITORINFO) -> Result<(), String> {
        // SAFETY: All registrations target the live owned HWND. Successful ownership is
        // recorded immediately, so any later error unwinds through Session::drop.
        unsafe {
            if matches!(mode, "observe" | "poll" | "raw" | "panic" | "close") {
                for event in [
                    EVENT_SYSTEM_FOREGROUND,
                    EVENT_SYSTEM_MINIMIZESTART,
                    EVENT_SYSTEM_MINIMIZEEND,
                ] {
                    let hook = SetWinEventHook(
                        event,
                        event,
                        null_mut(),
                        Some(event_proc),
                        0,
                        0,
                        WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
                    );
                    if hook.is_null() {
                        return Err(last_error("SetWinEventHook"));
                    }
                    self.hooks.push(hook);
                }
                println!("winevent_hooks_registered={}", self.hooks.len());
            }
            if mode == "raw" {
                let device = RAWINPUTDEVICE {
                    usUsagePage: 1,
                    usUsage: 2,
                    dwFlags: RIDEV_INPUTSINK,
                    hwndTarget: self.hwnd,
                };
                if RegisterRawInputDevices(&device, 1, size_of::<RAWINPUTDEVICE>() as u32) == 0 {
                    return Err(last_error("RegisterRawInputDevices"));
                }
                self.raw = true;
                println!("raw_mouse_registered=true (no keyboard, no NOLEGACY)");
            }
            if mode == "hotkey" {
                if RegisterHotKey(
                    self.hwnd,
                    1,
                    MOD_CONTROL | MOD_ALT | MOD_NOREPEAT,
                    'Q' as u32,
                ) == 0
                {
                    return Err(last_error("RegisterHotKey Ctrl+Alt+Q"));
                }
                self.hotkey = true;
                println!("hotkey_registered=Ctrl+Alt+Q (counts only; Windows key untouched)");
            }
            if mode == "tray" {
                let mut icon = self.tray_data();
                icon.uFlags = NIF_ICON | NIF_TIP | NIF_MESSAGE;
                icon.uCallbackMessage = TRAY_CALLBACK;
                icon.hIcon = LoadIconW(null_mut(), IDI_INFORMATION); // Shared icon: do not destroy.
                if icon.hIcon.is_null() {
                    return Err(last_error("LoadIconW"));
                }
                let tip = wide("Sei-Desk Stage 1 temporary tray lifetime probe");
                icon.szTip[..tip.len()].copy_from_slice(&tip);
                if Shell_NotifyIconW(NIM_ADD, &icon) == 0 {
                    return Err("NIM_ADD rejected (no trustworthy GetLastError contract)".into());
                }
                self.tray = true;
                icon.Anonymous.uVersion = NOTIFYICON_VERSION_4;
                if Shell_NotifyIconW(NIM_SETVERSION, &icon) == 0 {
                    return Err("NIM_SETVERSION rejected".into());
                }
                println!(
                    "tray_add=true tray_version4=true (registration only; visual/menu unverified)"
                );
            }
            if mode == "appbar" {
                let taskbar = FindWindowW(wide("Shell_TrayWnd").as_ptr(), null());
                let mut state = 0;
                if taskbar.is_null()
                    || IsWindowVisible(taskbar) == 0
                    || SHQueryUserNotificationState(&mut state) < 0
                    || state != QUNS_ACCEPTS_NOTIFICATIONS
                {
                    return Err("AppBar admission refused: visible native taskbar and normal notification state required".into());
                }
                let mut data = self.appbar_data();
                data.uCallbackMessage = APPBAR_CALLBACK;
                if SHAppBarMessage(ABM_NEW, &mut data) == 0 {
                    return Err("ABM_NEW rejected".into());
                }
                self.appbar = true;
                data.uEdge = ABE_BOTTOM;
                data.rc = primary.rcMonitor;
                let height = (58 * GetDpiForWindow(self.hwnd) as i32 / 96).max(32);
                data.rc.top = data.rc.bottom - height;
                SHAppBarMessage(ABM_QUERYPOS, &mut data);
                data.rc.top = data.rc.bottom - height;
                SHAppBarMessage(ABM_SETPOS, &mut data);
                println!("appbar_approved={:?}", rect(data.rc));
                if SetWindowPos(
                    self.hwnd,
                    null_mut(),
                    data.rc.left,
                    data.rc.top,
                    data.rc.right - data.rc.left,
                    height,
                    SWP_NOACTIVATE | SWP_NOZORDER | SWP_SHOWWINDOW,
                ) == 0
                {
                    return Err(last_error("AppBar SetWindowPos"));
                }
                SHAppBarMessage(ABM_WINDOWPOSCHANGED, &mut data);
                // Drain startup geometry notifications before marking external-change cancellation live.
            }
            if mode == "window" {
                ShowWindow(self.hwnd, SW_SHOWNOACTIVATE);
                ShowWindow(self.hwnd, SW_SHOWMINNOACTIVE);
                let minimized = IsIconic(self.hwnd) != 0;
                ShowWindow(self.hwnd, SW_SHOWNOACTIVATE);
                let restored = IsIconic(self.hwnd) == 0;
                println!("own_window_minimized={minimized} own_window_restored={restored}");
                if !minimized || !restored {
                    return Err("Own-window restore observation failed".into());
                }
            }
        }
        Ok(())
    }
}
impl Session {
    fn cleanup(&mut self) -> bool {
        let mut ok = true;
        // Ignore output failures so cleanup cannot panic if stderr is closed.
        use std::io::Write;
        let mut record = |label: &str, result: i32| {
            ok &= result != 0;
            let _ = writeln!(io::stderr(), "cleanup_{label}={result}");
        };
        // SAFETY: Same thread owns every registration. Remove shell resources before
        // destroying their HWND. No panic is permitted during unwind cleanup.
        unsafe {
            APPBAR_READY.store(false, Relaxed);
            if self.console {
                MESSAGE_THREAD.store(0, Relaxed);
                record(
                    "console_handler",
                    SetConsoleCtrlHandler(Some(console_handler), 0),
                );
                self.console = false;
            }
            if self.appbar {
                let mut data = self.appbar_data();
                record(
                    "appbar_remove",
                    SHAppBarMessage(ABM_REMOVE, &mut data) as i32,
                );
                self.appbar = false;
            }
            if self.tray {
                record(
                    "tray_delete",
                    Shell_NotifyIconW(NIM_DELETE, &self.tray_data()),
                );
                self.tray = false;
            }
            if self.hotkey {
                record("hotkey", UnregisterHotKey(self.hwnd, 1));
                self.hotkey = false;
            }
            if self.raw {
                let device = RAWINPUTDEVICE {
                    usUsagePage: 1,
                    usUsage: 2,
                    dwFlags: RIDEV_REMOVE,
                    hwndTarget: null_mut(),
                };
                record(
                    "raw",
                    RegisterRawInputDevices(&device, 1, size_of::<RAWINPUTDEVICE>() as u32),
                );
                self.raw = false;
            }
            for hook in self.hooks.drain(..) {
                record("winevent", UnhookWinEvent(hook));
            }
            if !self.hwnd.is_null() {
                KillTimer(self.hwnd, 1);
                record("window", DestroyWindow(self.hwnd));
                record(
                    "class",
                    UnregisterClassW(self.class.as_ptr(), GetModuleHandleW(null())),
                );
                self.hwnd = null_mut();
            }
        }
        ok
    }
}
impl Drop for Session {
    fn drop(&mut self) {
        self.cleanup();
    }
}

fn resources() -> Result<(u64, usize, usize), String> {
    // SAFETY: Pseudo handle is current process; output structures are properly sized.
    unsafe {
        let (mut create, mut exit, mut kernel, mut user): (FILETIME, FILETIME, FILETIME, FILETIME) =
            (zeroed(), zeroed(), zeroed(), zeroed());
        if GetProcessTimes(
            GetCurrentProcess(),
            &mut create,
            &mut exit,
            &mut kernel,
            &mut user,
        ) == 0
        {
            return Err(last_error("GetProcessTimes"));
        }
        let ticks = |t: FILETIME| (u64::from(t.dwHighDateTime) << 32) | u64::from(t.dwLowDateTime);
        let mut memory: PROCESS_MEMORY_COUNTERS_EX = zeroed();
        memory.cb = size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
        if GetProcessMemoryInfo(GetCurrentProcess(), &mut memory as *mut _ as _, memory.cb) == 0 {
            return Err(last_error("GetProcessMemoryInfo"));
        }
        Ok((
            ticks(kernel) + ticks(user),
            memory.WorkingSetSize,
            memory.PrivateUsage,
        ))
    }
}

pub fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let mode = args.first().map(String::as_str).unwrap_or("snapshot");
    if ![
        "snapshot", "observe", "poll", "raw", "hotkey", "tray", "appbar", "window", "panic",
        "close",
    ]
    .contains(&mode)
    {
        return Err("usage: probe [snapshot|observe|poll|raw|hotkey|tray|appbar|window|panic|close] [seconds:1..30]".into());
    }
    let seconds = args
        .get(1)
        .map(|s| s.parse::<u32>())
        .transpose()
        .map_err(|_| "Invalid duration")?
        .unwrap_or(5);
    if !(1..=30).contains(&seconds) || args.len() > 2 {
        return Err("Duration must be 1..30 seconds; no extra arguments".into());
    }
    // SAFETY: Establish process awareness before creating any HWND. No persistent setting.
    if unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) } == 0 {
        return Err(last_error("SetProcessDpiAwarenessContext PMv2"));
    }
    println!("mode={mode} duration_limit_seconds={seconds}");
    snapshot()?;
    if mode == "snapshot" {
        return Ok(());
    }
    // SAFETY: Read-only singleton admission check; do not overlap experiments.
    if !unsafe { FindWindowW(wide("SeiDesk.Stage1.BoundedProbe").as_ptr(), null()) }.is_null() {
        return Err("Another Stage 1 probe window exists; close it before testing".into());
    }
    let before = monitors()?;
    let primary = before
        .iter()
        .find(|m| m.dwFlags & MONITORINFOF_PRIMARY != 0)
        .ok_or("No primary monitor")?;
    // SAFETY: Read-only foreground query; no cross-process activation is requested.
    let foreground_before = unsafe { GetForegroundWindow() };
    let mut session = Session::new(primary)?;
    session.configure(mode, primary)?;
    // SAFETY: Timer belongs to the live window and is removed by Drop. This timer also
    // bounds loops with continuous event traffic using a separate monotonic deadline.
    if unsafe {
        SetTimer(
            session.hwnd,
            1,
            if mode == "poll" { 50 } else { seconds * 1000 },
            None,
        )
    } == 0
    {
        return Err(last_error("SetTimer"));
    }
    let start = Instant::now();
    let (cpu_start, _, _) = resources()?;
    let (mut messages, mut polls, mut entries) = (0u64, 0u64, 0u64);
    let mut was_edge = false;
    let mut raw_seen = 0;
    // SAFETY: Query DPI for the owned window positioned on the primary monitor.
    let dpi = unsafe { GetDpiForWindow(session.hwnd) } as i32;
    println!("probe_window_dpi={dpi}");
    let width = (760 * dpi / 96)
        .min(primary.rcMonitor.right - primary.rcMonitor.left - 24 * dpi / 96)
        .max(0);
    if mode == "close" {
        // SAFETY: Asynchronous normal close request to the owned HWND, not another app.
        if unsafe { PostMessageW(session.hwnd, WM_CLOSE, 0, 0) } == 0 {
            return Err(last_error("PostMessageW WM_CLOSE"));
        }
        println!("fixture_requested_wm_close=true");
    }
    if mode == "appbar" {
        // SAFETY: Drain at most 64 pending startup messages from our own thread;
        // after that, any further geometry change cancels the probe rather than guessing.
        unsafe {
            let mut pending: MSG = zeroed();
            for _ in 0..64 {
                if PeekMessageW(&mut pending, null_mut(), 0, 0, PM_REMOVE) == 0 {
                    break;
                }
                TranslateMessage(&pending);
                DispatchMessageW(&pending);
            }
        }
        APPBAR_READY.store(true, Relaxed);
    }
    while start.elapsed().as_secs_f64() < f64::from(seconds) {
        // SAFETY: Initialized MSG buffer; GetMessage provides valid parameters for dispatch.
        let mut message: MSG = unsafe { zeroed() };
        // SAFETY: Owned thread's standard blocking message loop.
        let result = unsafe { GetMessageW(&mut message, null_mut(), 0, 0) };
        if result == -1 {
            return Err(last_error("GetMessageW"));
        }
        if result == 0 {
            println!("received_wm_quit=true");
            break;
        }
        messages += 1;
        // SAFETY: Dispatch unchanged message. The callback does not unwind.
        unsafe {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
        let raw_count = RAW_MESSAGES.load(Relaxed);
        if (mode == "poll" && message.message == WM_TIMER)
            || (mode == "raw" && raw_count != raw_seen)
        {
            raw_seen = raw_count;
            let mut point = POINT { x: 0, y: 0 };
            // SAFETY: Stack-owned writable POINT; no input is changed or intercepted.
            if unsafe { GetCursorPos(&mut point) } != 0 {
                polls += 1;
                let edge = on_edge(point, primary.rcMonitor, width);
                if edge && !was_edge {
                    entries += 1;
                }
                was_edge = edge;
            }
        }
        // The first deadline timer is authoritative. Comparing elapsed time alone
        // could re-enter GetMessage just before the deadline and wait a second period.
        if mode != "poll" && message.message == WM_TIMER && message.wParam == 1 {
            break;
        }
        if CANCEL_APPBAR.load(Relaxed) && (mode == "appbar" || mode == "poll" || mode == "raw") {
            println!("cancelled_on_shell_or_display_change=true");
            break;
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    let (cpu_end, working, private) = resources()?;
    println!(
        "measurement: elapsed_s={elapsed:.3} cpu_ms={:.3} single_core_cpu_percent={:.5} working_set_bytes={working} private_bytes={private} messages={messages} cursor_queries={polls} edge_entries={entries} raw_messages={} winevent_callbacks={} hotkey_messages={} tray_callbacks={} topology_messages={}",
        (cpu_end - cpu_start) as f64 / 10_000.0,
        (cpu_end - cpu_start) as f64 / 100_000.0 / elapsed,
        RAW_MESSAGES.load(Relaxed),
        EVENTS.load(Relaxed),
        HOTKEY_MESSAGES.load(Relaxed),
        TRAY_MESSAGES.load(Relaxed),
        TOPOLOGY_MESSAGES.load(Relaxed)
    );
    // SAFETY: Read-only foreground query. Equality is evidence only if no external input occurred.
    let foreground_after = unsafe { GetForegroundWindow() };
    println!(
        "foreground_same_at_end={}",
        foreground_after == foreground_before
    );
    if mode == "panic" {
        panic!("Deliberate Rust unwind test; no AppBar/hotkey/tray owned");
    }
    let cleanup_ok = session.cleanup();
    drop(session);
    let after = monitors()?;
    let work_restored = before.len() == after.len()
        && before
            .iter()
            .zip(&after)
            .all(|(a, b)| same_rect(a.rcMonitor, b.rcMonitor) && same_rect(a.rcWork, b.rcWork));
    println!("work_areas_match_baseline={work_restored}");
    if !cleanup_ok {
        return Err(
            "One or more explicit cleanup calls failed; inspect evidence before another probe"
                .into(),
        );
    }
    if !work_restored {
        return Err(
            "Work areas changed; inspect desktop before further probes; do not use SPI_SETWORKAREA"
                .into(),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    // Pure geometry tests; these are not Windows integration successes.
    #[test]
    fn negative_origin_and_one_physical_pixel() {
        let r = RECT {
            left: -1920,
            top: -1080,
            right: 0,
            bottom: 0,
        };
        assert!(on_edge(POINT { x: -960, y: -1080 }, r, 760));
        assert!(!on_edge(POINT { x: -960, y: -1079 }, r, 760));
        assert!(!on_edge(POINT { x: -1341, y: -1080 }, r, 760));
        assert!(!on_edge(POINT { x: -580, y: -1080 }, r, 760));
    }
    #[test]
    fn narrow_monitor_clamps_sensor_width() {
        let r = RECT {
            left: 100,
            top: 200,
            right: 300,
            bottom: 500,
        };
        assert!(on_edge(POINT { x: 100, y: 200 }, r, 760));
        assert!(!on_edge(POINT { x: 300, y: 200 }, r, 760));
    }
}
