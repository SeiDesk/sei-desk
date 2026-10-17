//! Narrow desktop queries and a guarded restoration request. No shell registrations,
//! hooks, services, HWND creation, system-setting writes or background threads.
use std::fmt;

pub mod metrics;
pub mod network;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionPreference {
    Reduce,
    Allow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeError {
    pub operation: &'static str,
    pub code: u32,
}
impl fmt::Display for NativeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} failed (Windows error {})", self.operation, self.code)
    }
}
impl std::error::Error for NativeError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoreOutcome {
    ObservedForeground,
    Requested,
    Denied,
    Skipped,
}

/// A short-lived launch bookmark, not a durable foreign-control focus handle.
/// Identity is rechecked at close; HWND reuse within the same process/thread is
/// still a limitation. Windows foreground rules can deny the request.
#[derive(Default, Clone, Copy)]
pub struct ForegroundBookmark {
    #[cfg(windows)]
    window: usize,
    #[cfg(windows)]
    process: u32,
    #[cfg(windows)]
    thread: u32,
}

impl ForegroundBookmark {
    pub fn capture() -> Self {
        #[cfg(windows)]
        {
            use windows_sys::Win32::UI::WindowsAndMessaging::*;
            let mut process = 0;
            // SAFETY: Query only; output PID lives through the synchronous call.
            unsafe {
                let window = GetForegroundWindow();
                let thread = GetWindowThreadProcessId(window, &mut process);
                Self {
                    window: window as usize,
                    process,
                    thread,
                }
            }
        }
        #[cfg(not(windows))]
        Self::default()
    }

    /// Call only for explicit close/dismissal, never focus loss or validation errors.
    pub fn restore_if_owned(&self) -> RestoreOutcome {
        #[cfg(windows)]
        {
            use windows_sys::Win32::{
                Foundation::HWND, System::Threading::GetCurrentProcessId,
                UI::WindowsAndMessaging::*,
            };
            // SAFETY: HWND is opaque and never dereferenced. Revalidate its current
            // process/thread and ensure our process still owns foreground before asking.
            unsafe {
                let mut current_process = 0;
                GetWindowThreadProcessId(GetForegroundWindow(), &mut current_process);
                let mut target_process = 0;
                let target = self.window as HWND;
                let target_thread = GetWindowThreadProcessId(target, &mut target_process);
                if !can_restore(
                    current_process == GetCurrentProcessId(),
                    IsWindow(target) != 0,
                    self.process != 0
                        && self.process == target_process
                        && self.thread == target_thread
                        && self.process != GetCurrentProcessId(),
                ) {
                    return RestoreOutcome::Skipped;
                }
                if SetForegroundWindow(target) == 0 {
                    return RestoreOutcome::Denied;
                }
                if GetForegroundWindow() == target {
                    RestoreOutcome::ObservedForeground
                } else {
                    RestoreOutcome::Requested
                }
            }
        }
        #[cfg(not(windows))]
        RestoreOutcome::Skipped
    }
}

fn can_restore(owns_foreground: bool, valid_target: bool, same_identity: bool) -> bool {
    owns_foreground && valid_target && same_identity
}

pub fn motion_preference() -> Result<MotionPreference, NativeError> {
    #[cfg(windows)]
    {
        use windows_sys::Win32::{Foundation::GetLastError, UI::WindowsAndMessaging::*};
        let mut animate: i32 = 0;
        // SAFETY: SPI_GETCLIENTAREAANIMATION writes a BOOL to our sized stack value;
        // flags=0, query-only. No registry or system preference is modified.
        unsafe {
            if SystemParametersInfoW(
                SPI_GETCLIENTAREAANIMATION,
                0,
                &mut animate as *mut _ as _,
                0,
            ) == 0
            {
                return Err(NativeError {
                    operation: "SPI_GETCLIENTAREAANIMATION",
                    code: GetLastError(),
                });
            }
        }
        Ok(if animate == 0 {
            MotionPreference::Reduce
        } else {
            MotionPreference::Allow
        })
    }
    #[cfg(not(windows))]
    Err(NativeError {
        operation: "Windows motion preference unavailable on this platform",
        code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn restoration_requires_ownership_and_live_matching_identity() {
        for owns in [false, true] {
            for valid in [false, true] {
                for identity in [false, true] {
                    assert_eq!(
                        can_restore(owns, valid, identity),
                        owns && valid && identity
                    );
                }
            }
        }
    }
    #[test]
    fn native_error_keeps_operation_and_code() {
        let error = NativeError {
            operation: "preference query",
            code: 5,
        };
        assert_eq!(
            error.to_string(),
            "preference query failed (Windows error 5)"
        );
    }
}
