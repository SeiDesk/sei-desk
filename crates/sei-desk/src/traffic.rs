//! Bounded genuine interval averages, positioned by monotonic sample end time.
use sei_windows::network::{Identity, Rates, Reading, Status};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
pub const WINDOW: Duration = Duration::from_secs(60);
pub const CAPACITY: usize = 32;
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub at: Instant,
    pub rates: Rates,
}
#[derive(Default)]
pub struct History {
    points: VecDeque<Point>,
    cohort: Vec<Identity>,
    last: Option<Instant>,
}
impl History {
    pub fn observe(&mut self, reading: &Reading) {
        if self.last.is_some_and(|last| reading.at <= last) {
            return;
        }
        self.last = Some(reading.at);
        self.prune(reading.at);
        if !matches!(reading.status, Status::Failed(_)) && self.cohort != reading.cohort {
            self.points.clear();
            self.cohort.clone_from(&reading.cohort);
        }
        if let Status::Live(rates) = reading.status
            && rates.receive.is_finite()
            && rates.transmit.is_finite()
            && rates.receive >= 0.0
            && rates.transmit >= 0.0
        {
            if self.points.len() == CAPACITY {
                self.points.pop_front();
            }
            self.points.push_back(Point {
                at: reading.at,
                rates,
            });
        }
    }
    pub fn prune(&mut self, now: Instant) {
        while self
            .points
            .front()
            .is_some_and(|point| now.saturating_duration_since(point.at) > WINDOW)
        {
            self.points.pop_front();
        }
    }
    pub fn len(&self) -> usize {
        self.points.len()
    }
    pub fn points(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }
    pub fn peak(&self) -> f64 {
        self.points.iter().fold(1.0f64, |peak, p| {
            peak.max(p.rates.receive).max(p.rates.transmit)
        })
    }
}
pub fn format_rate(rate: f64) -> String {
    if !rate.is_finite() || rate < 0.0 {
        return "Unavailable".into();
    }
    let (value, unit) = if rate >= 1e9 {
        (rate / 1e9, "GB/s")
    } else if rate >= 1e6 {
        (rate / 1e6, "MB/s")
    } else if rate >= 1e3 {
        (rate / 1e3, "KB/s")
    } else {
        (rate, "B/s")
    };
    format!("{value:.1} {unit}")
}

/// Normalized pair-bar coordinates. No connecting line crosses a missing sample.
pub fn positions(history: &History, now: Instant) -> Vec<(f32, f32, f32)> {
    let peak = history.peak();
    history
        .points()
        .filter_map(|p| {
            let age = now.checked_duration_since(p.at)?;
            if age > WINDOW {
                return None;
            }
            Some((
                (1.0 - age.as_secs_f64() / WINDOW.as_secs_f64()) as f32,
                (p.rates.receive / peak) as f32,
                (p.rates.transmit / peak) as f32,
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests;
