//! Queries and placement for owned companion windows only. No shell resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Placement {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub dpi: u32,
    work: Rect,
}
impl Placement {
    pub fn logical_size(self) -> (f32, f32) {
        let scale = self.dpi as f32 / 96.0;
        (self.width as f32 / scale, self.height as f32 / scale)
    }
    /// Validate a physical work rectangle before it can be used for placement.
    pub fn new(work: Rect, dpi: u32) -> Result<Self, String> {
        if !(48..=768).contains(&dpi) {
            return Err("Display scale is unavailable.".into());
        }
        let px = |dip: i64| (dip * i64::from(dpi) + 48) / 96;
        let width = i64::from(work.right) - i64::from(work.left);
        let height = i64::from(work.bottom) - i64::from(work.top);
        let margin = px(12);
        let w = px(580).min(width - 2 * margin);
        let h = px(440).min(height - 2 * margin);
        if w < px(360) || h < px(300) || width > 100_000 || height > 100_000 {
            return Err("The display work area cannot safely fit the launcher.".into());
        }
        let x = i64::from(work.left) + (width - w) / 2;
        let y = i64::from(work.top) + px(80).min(height - h - margin).max(margin);
        Ok(Self {
            x: i32::try_from(x).map_err(|_| "Invalid display position.")?,
            y: i32::try_from(y).map_err(|_| "Invalid display position.")?,
            width: w as i32,
            height: h as i32,
            dpi,
            work,
        })
    }
}

#[cfg(windows)]
mod native {
    use super::*;
    use std::ffi::c_void;
    use windows_sys::Win32::{
        Foundation::HWND, System::Threading::GetCurrentProcessId, UI::WindowsAndMessaging::*,
    };
    #[repr(C)]
    struct MonitorInfo {
        size: u32,
        monitor: Rect,
        work: Rect,
        flags: u32,
    }
    // Existing locked bindings do not enable GDI/HiDpi. These narrow SDK ABI
    // declarations follow the metrics module convention without new features.
    #[link(name = "user32")]
    unsafe extern "system" {
        fn MonitorFromWindow(window: HWND, flags: u32) -> *mut c_void;
        fn MonitorFromRect(rect: *const Rect, flags: u32) -> *mut c_void;
        fn GetMonitorInfoW(monitor: *mut c_void, info: *mut MonitorInfo) -> i32;
        fn GetDpiForWindow(window: HWND) -> u32;
    }
    fn owned(value: usize) -> Result<HWND, String> {
        let hwnd = value as HWND;
        let mut pid = 0;
        // SAFETY: Query an opaque HWND; validate liveness and current-process
        // ownership before any write. The caller obtains it from a live borrow.
        unsafe {
            if IsWindow(hwnd) == 0
                || GetWindowThreadProcessId(hwnd, &mut pid) == 0
                || pid != GetCurrentProcessId()
            {
                return Err("The owned window is no longer available.".into());
            }
        }
        Ok(hwnd)
    }
    fn work(monitor: *mut c_void) -> Result<Rect, String> {
        let zero = Rect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        let mut info = MonitorInfo {
            size: std::mem::size_of::<MonitorInfo>() as u32,
            monitor: zero,
            work: zero,
            flags: 0,
        };
        // SAFETY: Initialized SDK-layout output; handle comes from a fresh
        // monitor query. Failure is propagated rather than using old geometry.
        if monitor.is_null() || unsafe { GetMonitorInfoW(monitor, &mut info) } == 0 {
            return Err("Monitor geometry is unavailable.".into());
        }
        Ok(info.work)
    }
    pub fn inspect(value: usize, require_foreground: bool) -> Result<Placement, String> {
        let hwnd = owned(value)?;
        // SAFETY: Live owned HWND, synchronous read-only queries on its UI thread.
        unsafe {
            if IsIconic(hwnd) != 0 || (require_foreground && GetForegroundWindow() != hwnd) {
                return Err("Open the launcher from the active management window.".into());
            }
            Placement::new(work(MonitorFromWindow(hwnd, 0))?, GetDpiForWindow(hwnd))
        }
    }
    pub fn place(value: usize, placement: Placement) -> Result<(), String> {
        let hwnd = owned(value)?;
        // SAFETY: Fresh monitor lookup validates the saved work rectangle before
        // moving only the currently borrowed owned window. Ordinary Z-order,
        // no activation or work-area/system-setting mutation.
        unsafe {
            if work(MonitorFromRect(&placement.work, 0))? != placement.work {
                return Err("The display changed before launcher placement.".into());
            }
            let mut current = std::mem::zeroed();
            if GetWindowRect(hwnd, &mut current) == 0 {
                return Err("Cannot read launcher bounds.".into());
            }
            if current.left == placement.x
                && current.top == placement.y
                && current.right - current.left == placement.width
                && current.bottom - current.top == placement.height
            {
                return Ok(());
            }
            if SetWindowPos(
                hwnd,
                std::ptr::null_mut(),
                placement.x,
                placement.y,
                placement.width,
                placement.height,
                SWP_NOACTIVATE | SWP_NOZORDER,
            ) == 0
            {
                return Err("Windows could not position the launcher.".into());
            }
        }
        Ok(())
    }
    pub fn foreground(value: usize) -> bool {
        owned(value).is_ok_and(|hwnd| {
            // SAFETY: Query only; handle was just validated.
            unsafe { GetForegroundWindow() == hwnd && IsIconic(hwnd) == 0 }
        })
    }
}
#[cfg(windows)]
pub use native::{foreground, inspect, place};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn negative_origins_and_scaled_work_areas_stay_inside_margins() {
        for dpi in [96, 120, 144, 192] {
            let work = Rect {
                left: -1920,
                top: -1080,
                right: 0,
                bottom: 0,
            };
            let p = Placement::new(work, dpi).unwrap();
            assert!(p.x >= work.left && p.y >= work.top);
            assert!(p.x + p.width <= work.right && p.y + p.height <= work.bottom);
            let (w, h) = p.logical_size();
            assert!(w >= 360.0 && h >= 300.0);
        }
    }
    #[test]
    fn invalid_or_too_small_geometry_never_produces_a_placement() {
        for (work, dpi) in [
            (
                Rect {
                    left: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                },
                96,
            ),
            (
                Rect {
                    left: 0,
                    top: 0,
                    right: 300,
                    bottom: 200,
                },
                96,
            ),
            (
                Rect {
                    left: i32::MIN,
                    top: 0,
                    right: i32::MAX,
                    bottom: 1080,
                },
                96,
            ),
            (
                Rect {
                    left: 0,
                    top: 0,
                    right: 1920,
                    bottom: 1080,
                },
                0,
            ),
        ] {
            assert!(Placement::new(work, dpi).is_err());
        }
    }
}
