//! A single headless Consumer owner. No enumeration, completion join or native
//! launch runs on iced's executor/UI thread. Idle waits are event-driven.
use super::{Catalog, Frame, Key, busy};
use iced::futures::{self, FutureExt, Stream, StreamExt, channel::mpsc, task::AtomicWaker};
use sei_apps::{
    Cancellation, LaunchRequest,
    consumer::{Accepted, Consumer, ConsumerError, NativeService, Operation, Service, Status},
};
use std::{
    hash::{Hash, Hasher},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    task::Poll,
    thread::JoinHandle,
};

#[derive(Debug)]
pub enum Command {
    Refresh,
    Cancel,
    Select(Option<Key>),
    Launch(Option<Key>),
}
struct Envelope {
    seq: u64,
    command: Command,
}
#[derive(Default)]
struct Latest {
    value: Mutex<Option<Arc<Frame>>>,
    waker: AtomicWaker,
    closed: AtomicBool,
}
impl Latest {
    fn publish(&self, frame: Frame) {
        *self.value.lock().expect("latest state poisoned") = Some(Arc::new(frame));
        self.waker.wake();
    }
}
#[derive(Clone)]
pub struct Feed(Arc<Latest>);
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
pub fn stream(feed: &Feed) -> impl Stream<Item = Arc<Frame>> + use<> {
    let latest = feed.0.clone();
    futures::stream::poll_fn(move |cx| {
        latest.waker.register(cx.waker());
        if let Some(value) = latest.value.lock().expect("latest state poisoned").take() {
            Poll::Ready(Some(value))
        } else if latest.closed.load(Ordering::Acquire) {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    })
}
#[derive(Default)]
struct Stop {
    closed: AtomicBool,
    waker: AtomicWaker,
}
#[derive(Clone)]
pub struct Handle {
    sender: Arc<Mutex<mpsc::Sender<Envelope>>>,
    sequence: Arc<Mutex<u64>>,
    stop: Arc<Stop>,
    feed: Feed,
}
impl Handle {
    pub fn feed(&self) -> Feed {
        self.feed.clone()
    }
    pub fn send(&self, command: Command) -> Result<u64, String> {
        if self.stop.closed.load(Ordering::Acquire) {
            return Err("Applications is shutting down".into());
        }
        let mut seq = self.sequence.lock().expect("command sequence poisoned");
        let next = seq
            .checked_add(1)
            .ok_or("Application command IDs exhausted")?;
        self.sender
            .lock()
            .expect("command sender poisoned")
            .try_send(Envelope { seq: next, command })
            .map_err(|_| "Applications is busy. Try again.".to_string())?;
        *seq = next;
        Ok(next)
    }
    pub fn stop(&self) {
        self.stop.closed.store(true, Ordering::Release);
        self.stop.waker.wake();
    }
}
// Entry + return/unwind notifications per Service call, at most two occupied
// Consumer slots. Only return receipts consume completions. Capacity four
// (+ one sender reservation) is separate from UI command backpressure.
#[derive(Clone, Copy)]
struct Notice {
    operation: Operation,
    id: u64,
    finished: bool,
}
struct Signals {
    wake: Mutex<mpsc::Sender<Notice>>,
    refresh: AtomicU64,
    launch: AtomicU64,
    entered_refresh: AtomicU64,
}
impl Signals {
    fn enter(self: &Arc<Self>, operation: Operation) -> Wake {
        let id = match operation {
            Operation::Refresh => self.refresh.load(Ordering::Acquire),
            Operation::Launch => self.launch.load(Ordering::Acquire),
        };
        if operation == Operation::Refresh {
            self.entered_refresh.store(id, Ordering::Release);
        }
        let notice = Notice {
            operation,
            id,
            finished: false,
        };
        let _ = self
            .wake
            .lock()
            .expect("wake sender poisoned")
            .try_send(notice);
        Wake {
            signals: self.clone(),
            notice,
        }
    }
}
struct Notifying {
    service: Arc<dyn Service>,
    signals: Arc<Signals>,
}
struct Wake {
    signals: Arc<Signals>,
    notice: Notice,
}
impl Drop for Wake {
    fn drop(&mut self) {
        let _ = self
            .signals
            .wake
            .lock()
            .expect("wake sender poisoned")
            .try_send(Notice {
                finished: true,
                ..self.notice
            });
    }
}
impl Service for Notifying {
    fn refresh(&self, cancel: &Cancellation) -> Result<sei_apps::Catalog, sei_apps::Error> {
        let _wake = self.signals.enter(Operation::Refresh);
        self.service.refresh(cancel)
    }
    fn launch(
        &self,
        request: LaunchRequest,
        cancel: &Cancellation,
    ) -> Result<Accepted, sei_apps::Error> {
        let _wake = self.signals.enter(Operation::Launch);
        self.service.launch(request, cancel)
    }
}
pub struct Owner {
    handle: Handle,
    worker: Option<JoinHandle<()>>,
}
impl Owner {
    pub fn start() -> std::io::Result<Self> {
        Self::with_service(Arc::new(NativeService))
    }
    pub(super) fn with_service(service: Arc<dyn Service>) -> std::io::Result<Self> {
        let (sender, mut commands) = mpsc::channel::<Envelope>(4);
        let (wake, mut wakes) = mpsc::channel(4);
        let feed = Feed(Arc::new(Latest::default()));
        let stop = Arc::new(Stop::default());
        let handle = Handle {
            sender: Arc::new(Mutex::new(sender)),
            sequence: Arc::new(Mutex::new(0)),
            stop: stop.clone(),
            feed: feed.clone(),
        };
        let worker = std::thread::Builder::new().name("sei-applications".into()).spawn(move || {
            let signals = Arc::new(Signals { wake: Mutex::new(wake), refresh: AtomicU64::new(0), launch: AtomicU64::new(0), entered_refresh: AtomicU64::new(0) });
            let mut consumer = Consumer::with_service(Arc::new(Notifying { service, signals: signals.clone() }));
            let mut catalog: Option<Arc<Catalog>> = None;
            let mut revision = 0u64; let mut ack = 0u64;
            let mut launch_name = None;
            let mut cancel_pending: Option<sei_apps::consumer::OperationId> = None;
            let mut error = None;
            let mut stale = false;
            loop {
                enum Event { Command(Option<Envelope>), Wake(Option<Notice>), Stop }
                let event = futures::executor::block_on(async {
                    let command = commands.next().fuse(); let wake = wakes.next().fuse();
                    let closing = futures::future::poll_fn(|cx| {
                        stop.waker.register(cx.waker());
                        if stop.closed.load(Ordering::Acquire) { Poll::Ready(()) } else { Poll::Pending }
                    }).fuse();
                    futures::pin_mut!(command, wake, closing);
                    futures::select_biased! { _ = closing => Event::Stop, notice = wake => Event::Wake(notice), cmd = command => Event::Command(cmd) }
                });
                if stop.closed.load(Ordering::Acquire) { break; }
                match event {
                    Event::Stop | Event::Command(None) | Event::Wake(None) => break,
                    Event::Wake(Some(notice)) if !notice.finished => {
                        if notice.operation == Operation::Refresh && let Some(id) = cancel_pending && id.get() == notice.id
                            && let Err(e) = consumer.cancel(Operation::Refresh, id) { error = Some(e); }
                    }
                    Event::Wake(Some(_)) => {
                        // Wake precedes Consumer's terminal publication. wait()
                        // handles that race and joins off-UI; no ready-queue poll.
                        match consumer.wait() {
                            Ok(Some(notice)) => stale |= notice.stale_discarded,
                            Ok(None) => (), Err(e) => error = Some(e),
                        }
                    }
                    Event::Command(Some(envelope)) => {
                        if envelope.seq <= ack { continue; }
                        error = None; stale = false;
                        ack = envelope.seq;
                        let result = match envelope.command {
                            Command::Refresh => consumer.request_refresh().map(|_| ()),
                            Command::Cancel => {
                                if let Status::Running { id, .. } = consumer.refresh_status() {
                                    let id = *id; cancel_pending = Some(id);
                                    // Cancelling before Service entry can bypass the
                                    // callback entirely in Consumer. Defer until its
                                    // entry acknowledgement, then use Consumer.cancel.
                                    if signals.entered_refresh.load(Ordering::Acquire) == id.get() { consumer.cancel(Operation::Refresh, id).map(|_| ()) } else { Ok(()) }
                                } else { Ok(()) }
                            },
                            Command::Select(key) => select(&mut consumer, key.as_ref()),
                            Command::Launch(key) => {
                                if busy(consumer.launch_status()) { Err(ConsumerError::Busy(Operation::Launch)) }
                                else { select(&mut consumer, key.as_ref()).and_then(|()| {
                                    let name = consumer.snapshot().and_then(|s| s.catalog.entries.iter().find(|e| Some(&e.id) == key.as_ref().map(|k| &k.id))).map(|e| e.display_name.clone());
                                    consumer.request_launch().map(|_| { launch_name = name; })
                                }) }
                            }
                        };
                        if let Err(e) = result { stale = e == ConsumerError::StaleSelection; error = Some(e); }
                        if let Status::Pending(id) = consumer.refresh_status() { signals.refresh.store(id.get(), Ordering::Release); signals.entered_refresh.store(0, Ordering::Release); }
                        if let Status::Pending(id) = consumer.launch_status() { signals.launch.store(id.get(), Ordering::Release); }
                        if let Err(e) = consumer.start_pending() { error = Some(e); }
                    }
                }
                if !busy(consumer.refresh_status()) { cancel_pending = None; }
                if let Some(snapshot) = consumer.snapshot()
                    && catalog.as_ref().is_none_or(|c| c.generation != snapshot.generation.get()) {
                    catalog = Some(Arc::new(Catalog::project(snapshot)));
                }
                let Some(next) = revision.checked_add(1) else { break; }; revision = next;
                feed.0.publish(Frame { revision, ack, catalog: catalog.clone(), refresh: consumer.refresh_status().clone(),
                    launch: consumer.launch_status().clone(), launch_name: launch_name.clone(), error: error.clone(), stale, cancel_pending: cancel_pending.is_some() });
            }
            consumer.begin_shutdown(); consumer.shutdown();
            feed.0.closed.store(true, Ordering::Release); feed.0.waker.wake();
        })?;
        Ok(Self {
            handle,
            worker: Some(worker),
        })
    }
    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }
}
fn select(consumer: &mut Consumer, key: Option<&Key>) -> Result<(), ConsumerError> {
    match key {
        None => consumer.clear_selection(),
        Some(key)
            if consumer
                .snapshot()
                .is_some_and(|s| s.generation.get() == key.generation) =>
        {
            consumer.select(&key.id)
        }
        Some(_) => {
            consumer.clear_selection()?;
            Err(ConsumerError::StaleSelection)
        }
    }
}
impl Drop for Owner {
    fn drop(&mut self) {
        self.handle.stop();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
#[cfg(test)]
mod tests;
