//! Read-only snapshots. Call from a worker, never the window event loop.
//! The small Kernel32 ABI declarations avoid changing the accepted dependency
//! features. Each buffer is owned, initialized, and used synchronously.
use crate::NativeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuTimes {
    pub idle: u64,
    pub kernel: u64,
    pub user: u64,
}

impl CpuTimes {
    /// Kernel includes idle. Reject resets, zero intervals and invalid counters.
    pub fn utilization_since(self, previous: Self) -> Option<f32> {
        let idle = self.idle.checked_sub(previous.idle)?;
        let total = self
            .kernel
            .checked_sub(previous.kernel)?
            .checked_add(self.user.checked_sub(previous.user)?)?;
        if total == 0 || idle > total {
            return None;
        }
        Some((100.0 * (total - idle) as f64 / total as f64) as f32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capacity {
    pub total: u64,
    pub available: u64,
}
impl Capacity {
    pub fn used(self) -> Option<u64> {
        (self.total > 0).then_some(())?;
        self.total.checked_sub(self.available)
    }
    pub fn percent(self) -> Option<f32> {
        Some((self.used()? as f64 / self.total as f64 * 100.0) as f32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Battery {
    Absent,
    Present {
        percent: Option<u8>,
        charging: bool,
        ac: Option<bool>,
    },
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub cpu: Result<CpuTimes, NativeError>,
    pub memory: Result<Capacity, NativeError>,
    pub disk: Result<Capacity, NativeError>,
    pub disk_root: String,
    pub battery: Result<Battery, NativeError>,
    pub clock: String,
}

pub fn sample() -> Snapshot {
    #[cfg(windows)]
    {
        native::sample()
    }
    #[cfg(not(windows))]
    {
        let error = NativeError {
            operation: "Windows monitoring unavailable",
            code: 50,
        };
        Snapshot {
            cpu: Err(error),
            memory: Err(error),
            disk: Err(error),
            disk_root: "System drive".into(),
            battery: Err(error),
            clock: "Time unavailable".into(),
        }
    }
}

/// Windows local wall clock, independent of the potentially slower sampler.
pub fn clock() -> String {
    #[cfg(windows)]
    {
        native::clock()
    }
    #[cfg(not(windows))]
    {
        "Local time unavailable".into()
    }
}

#[cfg(windows)]
mod native {
    use super::*;
    use windows_sys::Win32::{
        Foundation::{FILETIME, GetLastError},
        System::Threading::GetSystemTimes,
    };

    // Exact SDK layouts: MEMORYSTATUSEX, SYSTEM_POWER_STATUS, SYSTEMTIME.
    #[repr(C)]
    #[derive(Default)]
    struct Memory {
        length: u32,
        load: u32,
        total: u64,
        available: u64,
        page_total: u64,
        page_available: u64,
        virtual_total: u64,
        virtual_available: u64,
        extended_available: u64,
    }
    #[repr(C)]
    #[derive(Default)]
    struct Power {
        ac: u8,
        flags: u8,
        percent: u8,
        saver: u8,
        remaining: u32,
        full: u32,
    }
    #[repr(C)]
    #[derive(Default)]
    struct LocalTime {
        year: u16,
        month: u16,
        weekday: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
        millis: u16,
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GlobalMemoryStatusEx(buffer: *mut Memory) -> i32;
        fn GetDiskFreeSpaceExW(
            path: *const u16,
            available: *mut u64,
            total: *mut u64,
            free: *mut u64,
        ) -> i32;
        fn GetSystemPowerStatus(buffer: *mut Power) -> i32;
        fn GetLocalTime(buffer: *mut LocalTime);
    }
    fn result<T>(ok: i32, operation: &'static str, value: T) -> Result<T, NativeError> {
        if ok != 0 {
            Ok(value)
        } else {
            // SAFETY: GetLastError takes no pointers; immediately follows the call.
            Err(NativeError {
                operation,
                // SAFETY: No pointers; captures this thread's last native error.
                code: unsafe { GetLastError() },
            })
        }
    }
    fn ticks(time: FILETIME) -> u64 {
        (u64::from(time.dwHighDateTime) << 32) | u64::from(time.dwLowDateTime)
    }

    pub fn sample() -> Snapshot {
        let mut idle = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();
        // SAFETY: Three distinct initialized FILETIME outputs live through the call.
        let ok = unsafe { GetSystemTimes(&mut idle, &mut kernel, &mut user) };
        let cpu = result(
            ok,
            "CPU counters",
            CpuTimes {
                idle: ticks(idle),
                kernel: ticks(kernel),
                user: ticks(user),
            },
        );
        let mut memory = Memory {
            length: size_of::<Memory>() as u32,
            ..Default::default()
        };
        // SAFETY: SDK-compatible, initialized buffer with required byte length.
        let ok = unsafe { GlobalMemoryStatusEx(&mut memory) };
        let memory = result(
            ok,
            "Physical memory",
            Capacity {
                total: memory.total,
                available: memory.available,
            },
        );
        // SystemRoot is local Windows installation storage; reject remote or
        // unusual paths rather than risk a network filesystem query on shutdown.
        let root = std::env::var("SystemRoot").unwrap_or_default();
        let drive = root.as_bytes();
        let disk_root =
            if drive.len() >= 3 && drive[0].is_ascii_alphabetic() && &drive[1..3] == b":\\" {
                root[..3].to_owned()
            } else {
                String::new()
            };
        let disk = if disk_root.is_empty() {
            Err(NativeError {
                operation: "Local system drive unavailable",
                code: 123,
            })
        } else {
            let path: Vec<u16> = disk_root.encode_utf16().chain(Some(0)).collect();
            let (mut available, mut total, mut free) = (0, 0, 0);
            // SAFETY: NUL-terminated UTF-16 local root and distinct u64 outputs.
            let ok = unsafe {
                GetDiskFreeSpaceExW(path.as_ptr(), &mut available, &mut total, &mut free)
            };
            // Capacity/free use the caller-visible quota pair, consistently.
            result(ok, "System drive capacity", Capacity { total, available })
        };
        let mut power = Power::default();
        // SAFETY: Exact initialized SYSTEM_POWER_STATUS layout, synchronous output.
        let ok = unsafe { GetSystemPowerStatus(&mut power) };
        let battery = result(
            ok,
            "Battery status",
            decode_power(power.flags, power.percent, power.ac),
        );
        Snapshot {
            cpu,
            memory,
            disk,
            disk_root,
            battery,
            clock: clock(),
        }
    }

    pub fn clock() -> String {
        let mut time = LocalTime::default();
        // SAFETY: Exact initialized SYSTEMTIME layout; GetLocalTime cannot fail.
        unsafe { GetLocalTime(&mut time) };
        format!(
            "{:02}:{:02}:{:02}  ·  {:04}-{:02}-{:02}",
            time.hour, time.minute, time.second, time.year, time.month, time.day
        )
    }

    fn decode_power(flags: u8, percent: u8, ac: u8) -> Battery {
        if flags == 255 {
            Battery::Unknown
        } else if flags & 128 != 0 {
            Battery::Absent
        } else {
            Battery::Present {
                percent: (percent <= 100).then_some(percent),
                charging: flags & 8 != 0,
                ac: match ac {
                    0 => Some(false),
                    1 => Some(true),
                    _ => None,
                },
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn sdk_layouts_and_unknown_battery() {
            assert_eq!(size_of::<Memory>(), 64);
            assert_eq!(size_of::<Power>(), 12);
            assert_eq!(size_of::<LocalTime>(), 16);
            assert_eq!(decode_power(255, 255, 255), Battery::Unknown);
            assert_eq!(decode_power(128, 255, 1), Battery::Absent);
            assert_eq!(
                decode_power(8, 255, 1),
                Battery::Present {
                    percent: None,
                    charging: true,
                    ac: Some(true)
                }
            );
        }
        #[test]
        fn real_snapshot_has_clock_and_valid_capacities() {
            let snapshot = sample();
            assert!(snapshot.clock.contains(':'));
            assert!(
                snapshot
                    .memory
                    .expect("Windows physical memory")
                    .used()
                    .is_some()
            );
            assert!(snapshot.disk.expect("Local system drive").used().is_some());
            assert!(snapshot.cpu.is_ok());
            assert!(snapshot.battery.is_ok());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn counter_deltas_include_kernel_idle_and_reject_gaps() {
        let start = CpuTimes {
            idle: 10,
            kernel: 20,
            user: 10,
        };
        let end = CpuTimes {
            idle: 30,
            kernel: 50,
            user: 20,
        };
        assert_eq!(end.utilization_since(start), Some(50.0));
        assert_eq!(start.utilization_since(end), None);
        assert_eq!(start.utilization_since(start), None);
        assert_eq!(CpuTimes { idle: 500, ..end }.utilization_since(start), None);
    }
    #[test]
    fn invalid_capacity_never_becomes_zero_usage() {
        assert_eq!(
            Capacity {
                total: 0,
                available: 0
            }
            .percent(),
            None
        );
        assert_eq!(
            Capacity {
                total: 10,
                available: 11
            }
            .used(),
            None
        );
        assert_eq!(
            Capacity {
                total: 10,
                available: 4
            }
            .percent(),
            Some(60.0)
        );
    }
}
