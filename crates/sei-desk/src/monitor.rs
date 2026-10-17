//! Two fixed workers: a two-second sampler and a one-second freshness pulse.
//! Bounded delivery drops readings under backpressure; no catch-up queue or
//! thread per tick. The owner outlives iced and joins only after its event loop.
use iced::futures::{Stream, channel::mpsc, stream};
use sei_windows::metrics::{self, CpuTimes, Snapshot};
use sei_windows::network::{self, Reading, Tracker};
use std::{
    hash::{Hash, Hasher},
    sync::{Arc, Mutex, mpsc as sync},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub enum Update {
    Sample {
        snapshot: Snapshot,
        cpu: Option<f32>,
        network: Box<Reading>,
        at: Instant,
    },
    Pulse(String),
    Failed(String),
}
#[derive(Clone)]
pub struct Feed(Arc<Mutex<Option<mpsc::Receiver<Update>>>>);
impl PartialEq for Feed {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for Feed {}
impl Hash for Feed {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.0).hash(state);
    }
}

pub fn stream(feed: &Feed) -> impl Stream<Item = Update> + use<> {
    let receiver = feed.0.lock().expect("feed mutex poisoned").take();
    stream::unfold(receiver, |receiver| async move {
        use iced::futures::StreamExt;
        let mut receiver = receiver?;
        receiver.next().await.map(|update| (update, Some(receiver)))
    })
}
struct Worker {
    stop: sync::Sender<()>,
    handle: Option<JoinHandle<()>>,
}
impl Worker {
    fn spawn(
        name: &str,
        interval: Duration,
        mut task: impl FnMut() + Send + 'static,
    ) -> std::io::Result<Self> {
        let (stop, receiver) = sync::channel();
        let handle = thread::Builder::new().name(name.into()).spawn(move || {
            loop {
                task();
                if receiver.recv_timeout(interval) != Err(sync::RecvTimeoutError::Timeout) {
                    break;
                }
            }
        })?;
        Ok(Self {
            stop,
            handle: Some(handle),
        })
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
pub struct Monitor {
    feed: Feed,
    workers: Vec<Worker>,
}
impl Monitor {
    pub fn start() -> std::io::Result<Self> {
        let (mut sender, receiver) = mpsc::channel(3);
        let mut pulse = sender.clone();
        let feed = Feed(Arc::new(Mutex::new(Some(receiver))));
        let mut previous: Option<CpuTimes> = None;
        let mut network_tracker = Tracker::default();
        let sampler = Worker::spawn("sei-metrics", Duration::from_secs(2), move || {
            // Contain a sampler panic and publish a visible failure; the pulse
            // continues so retained readings are still marked paused.
            match std::panic::catch_unwind(|| {
                let snapshot = metrics::sample();
                let interfaces = network::sample();
                (snapshot, interfaces, Instant::now())
            }) {
                Ok((snapshot, interfaces, at)) => {
                    let network = network_tracker.observe(interfaces, at);
                    let current = snapshot.cpu.as_ref().ok().copied();
                    let cpu = current
                        .zip(previous)
                        .and_then(|(now, before)| now.utilization_since(before));
                    previous = current; // Errors break the sampling interval.
                    let _ = sender.try_send(Update::Sample {
                        snapshot,
                        cpu,
                        network: Box::new(network),
                        at,
                    });
                }
                Err(_) => {
                    previous = None;
                    network_tracker = Tracker::default();
                    let _ = sender.try_send(Update::Failed(
                        "Monitoring failed. Close and reopen Sei-Desk to retry.".into(),
                    ));
                }
            }
        })?;
        let ticker = Worker::spawn("sei-freshness", Duration::from_secs(1), move || {
            let _ = pulse.try_send(Update::Pulse(metrics::clock()));
        })?;
        Ok(Self {
            feed,
            workers: vec![sampler, ticker],
        })
    }
    pub fn feed(&self) -> Feed {
        self.feed.clone()
    }
}
impl Drop for Monitor {
    fn drop(&mut self) {
        // Signal all workers before joining either one.
        for worker in &self.workers {
            let _ = worker.stop.send(());
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn publication_stays_bounded_with_two_undrained_senders() {
        let (mut sender, _receiver) = mpsc::channel(3);
        let mut other = sender.clone();
        let mut accepted = 0;
        for _ in 0..1000 {
            for sender in [&mut sender, &mut other] {
                match sender.try_send(Update::Pulse(String::new())) {
                    Ok(()) => accepted += 1,
                    Err(error) => assert!(error.is_full()),
                }
            }
        }
        assert!(accepted <= 5);
    }
    #[test]
    fn shutdown_wakes_sleepers_and_joins_with_undrained_channel() {
        let started = Instant::now();
        for _ in 0..3 {
            let monitor = Monitor::start().unwrap();
            drop(monitor);
        }
        assert!(started.elapsed() < Duration::from_secs(5));
    }
}
