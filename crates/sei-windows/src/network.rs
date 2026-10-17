//! Owned interface counters and pure, monotonic-time aggregate rate calculation.
use crate::NativeError;
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};
#[cfg(windows)]
mod native;

pub const MAX_INTERFACES: usize = 256;
pub const MAX_INTERVAL: Duration = Duration::from_secs(10);
pub const STALE_AFTER: Duration = Duration::from_secs(5);

/// GUID and LUID together: an interface index is never a persistent identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identity {
    pub guid: u128,
    pub luid: u64,
}
#[derive(Debug, Clone)]
pub struct Interface {
    pub id: Identity,
    pub kind: u32,
    pub flags: u8,
    pub operational: bool,
    pub admin_up: bool,
    pub connected: bool,
    pub received: u64,
    pub transmitted: u64,
}
impl Interface {
    pub fn included(&self) -> bool {
        // Physical Ethernet / Wi-Fi only. Exclude filters, unauthenticated,
        // disconnected, paused, low-power and endpoint layers of a stack.
        matches!(self.kind, 6 | 71)
            && self.flags & 1 != 0
            && self.flags & 0b1111_1010 == 0
            && self.operational
            && self.admin_up
            && self.connected
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rates {
    pub receive: f64,
    pub transmit: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Live(Rates),
    Baseline(&'static str),
    Unavailable(&'static str),
    Failed(NativeError),
}
#[derive(Debug, Clone)]
pub struct Reading {
    pub at: Instant,
    pub cohort: Vec<Identity>,
    pub status: Status,
}
impl Reading {
    pub fn stale(&self, now: Instant) -> bool {
        now.saturating_duration_since(self.at) >= STALE_AFTER
    }
}
type Counters = BTreeMap<Identity, (u64, u64)>;
#[derive(Default)]
pub struct Tracker {
    previous: Option<(Instant, Counters)>,
}
impl Tracker {
    pub fn observe(&mut self, data: Result<Vec<Interface>, NativeError>, at: Instant) -> Reading {
        let status = |status| Reading {
            at,
            cohort: Vec::new(),
            status,
        };
        // Never rewind a valid baseline on duplicate or out-of-order results.
        if self
            .previous
            .as_ref()
            .is_some_and(|(before, _)| at <= *before)
        {
            return status(Status::Unavailable("Invalid or out-of-order sample time"));
        }
        let rows = match data {
            Ok(rows) if rows.len() <= MAX_INTERFACES => rows,
            Ok(_) => {
                self.previous = None;
                return status(Status::Unavailable(
                    "Interface count exceeds monitoring limit",
                ));
            }
            Err(error) => {
                self.previous = None;
                return status(Status::Failed(error));
            }
        };
        let mut current = Counters::new();
        for row in rows.into_iter().filter(Interface::included) {
            // Duplicate GUID OR LUID could be two representations of one link.
            // Reject the snapshot instead of guessing which counter to retain.
            if row.id.guid == 0
                || row.id.luid == 0
                || current
                    .keys()
                    .any(|id| id.guid == row.id.guid || id.luid == row.id.luid)
            {
                self.previous = None;
                return status(Status::Unavailable("Invalid or duplicate adapter identity"));
            }
            current.insert(row.id, (row.received, row.transmitted));
        }
        let cohort = current.keys().copied().collect();
        let previous = self.previous.replace((at, current.clone()));
        let state = if current.is_empty() {
            Status::Unavailable("No connected physical Ethernet or Wi-Fi adapters")
        } else if let Some((before, previous)) = previous {
            let elapsed = at.duration_since(before);
            if !current.keys().eq(previous.keys()) {
                Status::Baseline("Adapters changed · collecting a new baseline")
            } else if elapsed > MAX_INTERVAL {
                Status::Baseline("Sampling gap · collecting a new baseline")
            } else if elapsed < Duration::from_millis(100) {
                Status::Baseline("Sampling interval too short · waiting for the next sample")
            } else {
                aggregate(&current, &previous, elapsed)
                    .map(Status::Live)
                    .unwrap_or(Status::Baseline(
                        "Counters reset · collecting a new baseline",
                    ))
            }
        } else {
            Status::Baseline("Collecting the first counter interval")
        };
        Reading {
            at,
            cohort,
            status: state,
        }
    }
}
fn aggregate(current: &Counters, previous: &Counters, elapsed: Duration) -> Option<Rates> {
    // Very short intervals are not useful observations and magnify jitter.
    if elapsed < Duration::from_millis(100) || elapsed > MAX_INTERVAL {
        return None;
    }
    let mut rx = 0u128;
    let mut tx = 0u128;
    for (id, (received, transmitted)) in current {
        let (old_rx, old_tx) = previous.get(id)?;
        // 64-bit wrap and reset cannot be distinguished: rebaseline both.
        rx += u128::from(received.checked_sub(*old_rx)?);
        tx += u128::from(transmitted.checked_sub(*old_tx)?);
    }
    Some(Rates {
        receive: rx as f64 / elapsed.as_secs_f64(),
        transmit: tx as f64 / elapsed.as_secs_f64(),
    })
}
pub fn sample() -> Result<Vec<Interface>, NativeError> {
    #[cfg(windows)]
    {
        native::sample()
    }
    #[cfg(not(windows))]
    {
        Err(NativeError {
            operation: "Windows network counters unavailable",
            code: 50,
        })
    }
}

#[cfg(test)]
mod tests;
