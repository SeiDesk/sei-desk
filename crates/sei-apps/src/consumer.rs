//! Single-owner headless coordination. One refresh and one launch may overlap;
//! no permanent worker, shared model lock, timer, or automatic launch exists.
use crate::{Cancellation, Catalog, Error, ErrorKind, LaunchRequest};
use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{
        Arc,
        mpsc::{self, Receiver, SyncSender},
    },
    thread::JoinHandle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationId(u64);
impl OperationId {
    pub fn get(self) -> u64 {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Refresh,
    Launch,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsumerError {
    Closed,
    Busy(Operation),
    IdExhausted,
    NoSelection,
    UnknownEntry,
    StaleSelection,
    ThreadStart { code: Option<i32> },
    WorkerPanicked,
    Service(Error),
}
impl std::fmt::Display for ConsumerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ConsumerError {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status<T> {
    Idle,
    Pending(OperationId),
    Running {
        id: OperationId,
        cancellation_requested: bool,
    },
    Succeeded {
        id: OperationId,
        value: T,
    },
    Failed {
        id: OperationId,
        error: ConsumerError,
    },
    Cancelled(OperationId),
    Disposed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefreshSummary {
    pub entries: usize,
    pub issues: usize,
    pub truncated: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Accepted {
    pub pid: u32,
}
#[derive(Debug)]
pub struct Snapshot {
    pub generation: OperationId,
    pub catalog: Catalog,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub generation: OperationId,
    pub entry_id: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletionNotice {
    pub operation: Operation,
    pub id: OperationId,
    pub stale_discarded: bool,
}

/// Blocking service boundary, invoked exclusively by finite consumer workers.
/// Fake implementations are for deterministic tests, not Windows validation.
pub trait Service: Send + Sync + 'static {
    fn refresh(&self, cancel: &Cancellation) -> Result<Catalog, Error>;
    fn launch(&self, request: LaunchRequest, cancel: &Cancellation) -> Result<Accepted, Error>;
}
#[derive(Default)]
pub struct NativeService;
impl Service for NativeService {
    fn refresh(&self, cancel: &Cancellation) -> Result<Catalog, Error> {
        crate::refresh(cancel, None)
    }
    fn launch(&self, request: LaunchRequest, cancel: &Cancellation) -> Result<Accepted, Error> {
        let receipt = crate::launch(request, cancel)?;
        // Closing our Child handle never terminates the accepted application.
        // Application lifetime/window tracking is outside this consumer's scope.
        Ok(Accepted {
            pid: receipt.child.id(),
        })
    }
}
struct Intent {
    selection: Selection,
    request: LaunchRequest,
}
struct Work {
    id: OperationId,
    cancel: Cancellation,
    handle: Option<JoinHandle<()>>,
    intent: Option<Intent>,
}
enum Completion {
    Refresh(OperationId, Result<Catalog, ConsumerError>),
    Launch(OperationId, Result<Accepted, ConsumerError>),
}

/// Installed before any service/payload code. Return and unwind both publish
/// exactly once. The fallback is retained until a real result is available.
/// This does not cover process abort, forced termination or nonterminating Drop.
struct TerminalPublisher {
    sender: SyncSender<Completion>,
    completion: Option<Completion>,
}
impl TerminalPublisher {
    fn new(sender: SyncSender<Completion>, operation: Operation, id: OperationId) -> Self {
        let completion = match operation {
            Operation::Refresh => Completion::Refresh(id, Err(ConsumerError::WorkerPanicked)),
            Operation::Launch => Completion::Launch(id, Err(ConsumerError::WorkerPanicked)),
        };
        Self {
            sender,
            completion: Some(completion),
        }
    }
    fn publish(mut self, completion: Completion) {
        self.completion = Some(completion);
        // Drop publishes before later captured-resource cleanup can unwind.
    }
}
impl Drop for TerminalPublisher {
    fn drop(&mut self) {
        if let Some(completion) = self.completion.take() {
            // One event per occupied slot, capacity two, receiver owned until
            // after all joins: this cannot wait for an owner to drain the queue.
            let _ = self.sender.send(completion);
        }
    }
}

fn join_worker(handle: JoinHandle<()>) {
    if let Err(mut payload) = handle.join() {
        // A panic payload is arbitrary user code on Drop. Dispose it here,
        // outside unwinding, without letting a finite nested cleanup panic
        // unwind the owner or leak the replacement payload. This catches only
        // payload disposal, never model/state transitions. A malicious infinite
        // destructor/panic chain can still block, like a nonreturning Service.
        while let Err(next) = catch_unwind(AssertUnwindSafe(|| drop(payload))) {
            payload = next;
        }
    }
}

/// Own on one headless thread. shutdown/Drop joins workers and may wait for an
/// uninterruptible OS call: never dispose this owner on a future GUI thread.
pub struct Consumer {
    service: Arc<dyn Service>,
    next_id: u64,
    closed: bool,
    snapshot: Option<Snapshot>,
    selection: Option<Selection>,
    refresh_status: Status<RefreshSummary>,
    launch_status: Status<Accepted>,
    refresh_work: Option<Work>,
    launch_work: Option<Work>,
    sender: SyncSender<Completion>,
    receiver: Receiver<Completion>,
    last_discarded: Option<CompletionNotice>,
}
impl Default for Consumer {
    fn default() -> Self {
        Self::new()
    }
}
impl Consumer {
    pub fn new() -> Self {
        Self::with_service(Arc::new(NativeService))
    }
    pub fn with_service(service: Arc<dyn Service>) -> Self {
        // At most two outstanding workers, each sending exactly one completion.
        // Completed workers still occupy their slot until received and joined.
        let (sender, receiver) = mpsc::sync_channel(2);
        Self {
            service,
            next_id: 0,
            closed: false,
            snapshot: None,
            selection: None,
            refresh_status: Status::Idle,
            launch_status: Status::Idle,
            refresh_work: None,
            launch_work: None,
            sender,
            receiver,
            last_discarded: None,
        }
    }
    pub fn snapshot(&self) -> Option<&Snapshot> {
        self.snapshot.as_ref()
    }
    pub fn selection(&self) -> Option<&Selection> {
        self.selection.as_ref()
    }
    pub fn refresh_status(&self) -> &Status<RefreshSummary> {
        &self.refresh_status
    }
    pub fn launch_status(&self) -> &Status<Accepted> {
        &self.launch_status
    }
    pub fn last_discarded(&self) -> Option<CompletionNotice> {
        self.last_discarded
    }
    pub fn is_closed(&self) -> bool {
        self.closed
    }
    fn open(&self) -> Result<(), ConsumerError> {
        if self.closed {
            Err(ConsumerError::Closed)
        } else {
            Ok(())
        }
    }
    fn allocate(&mut self) -> Result<OperationId, ConsumerError> {
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or(ConsumerError::IdExhausted)?;
        Ok(OperationId(self.next_id))
    }
    pub fn request_refresh(&mut self) -> Result<OperationId, ConsumerError> {
        self.open()?;
        if self.refresh_work.is_some() {
            return Err(ConsumerError::Busy(Operation::Refresh));
        }
        let id = self.allocate()?;
        self.refresh_work = Some(Work {
            id,
            cancel: Cancellation::default(),
            handle: None,
            intent: None,
        });
        self.refresh_status = Status::Pending(id);
        Ok(id)
    }
    pub fn select(&mut self, entry_id: &str) -> Result<(), ConsumerError> {
        self.open()?;
        let generation = self
            .snapshot
            .as_ref()
            .filter(|s| s.catalog.entries.iter().any(|e| e.id == entry_id))
            .map(|s| s.generation);
        // A failed selection must not leave a different app selected silently.
        self.selection = None;
        let generation = generation.ok_or(ConsumerError::UnknownEntry)?;
        self.selection = Some(Selection {
            generation,
            entry_id: entry_id.into(),
        });
        Ok(())
    }
    pub fn clear_selection(&mut self) -> Result<(), ConsumerError> {
        self.open()?;
        self.selection = None;
        Ok(())
    }
    pub fn request_launch(&mut self) -> Result<OperationId, ConsumerError> {
        self.open()?;
        if self.launch_work.is_some() {
            return Err(ConsumerError::Busy(Operation::Launch));
        }
        let selection = self.selection.clone().ok_or(ConsumerError::NoSelection)?;
        let request = self
            .snapshot
            .as_ref()
            .filter(|s| s.generation == selection.generation)
            .and_then(|s| {
                s.catalog
                    .entries
                    .iter()
                    .find(|e| e.id == selection.entry_id)
            })
            .ok_or(ConsumerError::StaleSelection)?
            .request
            .clone();
        let id = self.allocate()?;
        self.launch_work = Some(Work {
            id,
            cancel: Cancellation::default(),
            handle: None,
            intent: Some(Intent { selection, request }),
        });
        self.launch_status = Status::Pending(id);
        Ok(id)
    }
    /// Dispatch only previously requested work. Does not enumerate/create a
    /// process on this thread; all effects run through Service in owned workers.
    pub fn start_pending(&mut self) -> Result<(), ConsumerError> {
        self.open()?;
        for operation in [Operation::Refresh, Operation::Launch] {
            let slot = match operation {
                Operation::Refresh => &mut self.refresh_work,
                Operation::Launch => &mut self.launch_work,
            };
            let Some(work) = slot.as_mut().filter(|w| w.handle.is_none()) else {
                continue;
            };
            if let Some(intent) = &work.intent
                && !self.snapshot.as_ref().is_some_and(|s| {
                    s.generation == intent.selection.generation
                        && s.catalog
                            .entries
                            .iter()
                            .any(|e| e.id == intent.selection.entry_id)
                })
            {
                self.launch_status = Status::Failed {
                    id: work.id,
                    error: ConsumerError::StaleSelection,
                };
                *slot = None;
                continue;
            }
            let id = work.id;
            let cancel = work.cancel.clone();
            let intent = work.intent.take();
            let service = self.service.clone();
            let sender = self.sender.clone();
            let handle = std::thread::Builder::new()
                .name(
                    match operation {
                        Operation::Refresh => "sei-consumer-refresh",
                        Operation::Launch => "sei-consumer-launch",
                    }
                    .into(),
                )
                .spawn(move || {
                    let publisher = TerminalPublisher::new(sender, operation, id);
                    let completion = catch_unwind(AssertUnwindSafe(|| {
                        // A cancellation before dispatch reaches even injected
                        // services without allowing a process-creation side effect.
                        if cancel.is_cancelled() {
                            let error = ConsumerError::Service(Error::new(
                                ErrorKind::Cancelled,
                                "consumer dispatch",
                            ));
                            return match operation {
                                Operation::Refresh => Completion::Refresh(id, Err(error)),
                                Operation::Launch => Completion::Launch(id, Err(error)),
                            };
                        }
                        match intent {
                            None => Completion::Refresh(
                                id,
                                service.refresh(&cancel).map_err(ConsumerError::Service),
                            ),
                            Some(intent) => Completion::Launch(
                                id,
                                service
                                    .launch(intent.request, &cancel)
                                    .map_err(ConsumerError::Service),
                            ),
                        }
                    }))
                    .unwrap_or(match operation {
                        Operation::Refresh => {
                            Completion::Refresh(id, Err(ConsumerError::WorkerPanicked))
                        }
                        Operation::Launch => {
                            Completion::Launch(id, Err(ConsumerError::WorkerPanicked))
                        }
                    });
                    publisher.publish(completion);
                });
            match handle {
                Ok(handle) => {
                    work.handle = Some(handle);
                    match operation {
                        Operation::Refresh => {
                            self.refresh_status = Status::Running {
                                id,
                                cancellation_requested: false,
                            }
                        }
                        Operation::Launch => {
                            self.launch_status = Status::Running {
                                id,
                                cancellation_requested: false,
                            }
                        }
                    }
                }
                Err(error) => {
                    let error = ConsumerError::ThreadStart {
                        code: error.raw_os_error(),
                    };
                    match operation {
                        Operation::Refresh => self.refresh_status = Status::Failed { id, error },
                        Operation::Launch => self.launch_status = Status::Failed { id, error },
                    }
                    *slot = None;
                }
            }
        }
        Ok(())
    }
    /// Return false if this ID no longer occupies the lane. Running work
    /// keeps its slot until joined; cancellation cannot authorize another scan.
    pub fn cancel(&mut self, operation: Operation, id: OperationId) -> Result<bool, ConsumerError> {
        self.open()?;
        let slot = match operation {
            Operation::Refresh => &mut self.refresh_work,
            Operation::Launch => &mut self.launch_work,
        };
        let Some(work) = slot else { return Ok(false) };
        if work.id != id {
            return Ok(false);
        }
        work.cancel.cancel();
        let pending = work.handle.is_none();
        match operation {
            Operation::Refresh => {
                self.refresh_status = if pending {
                    Status::Cancelled(id)
                } else {
                    Status::Running {
                        id,
                        cancellation_requested: true,
                    }
                }
            }
            Operation::Launch => {
                self.launch_status = if pending {
                    Status::Cancelled(id)
                } else {
                    Status::Running {
                        id,
                        cancellation_requested: true,
                    }
                }
            }
        }
        if pending {
            *slot = None;
        }
        Ok(true)
    }
    fn finish(&mut self, completion: Completion) -> CompletionNotice {
        let (operation, id) = match &completion {
            Completion::Refresh(id, _) => (Operation::Refresh, *id),
            Completion::Launch(id, _) => (Operation::Launch, *id),
        };
        let slot = match operation {
            Operation::Refresh => &mut self.refresh_work,
            Operation::Launch => &mut self.launch_work,
        };
        let mut notice = CompletionNotice {
            operation,
            id,
            stale_discarded: false,
        };
        if self.closed
            || !slot
                .as_ref()
                .is_some_and(|w| w.id == id && w.handle.is_some())
        {
            notice.stale_discarded = true;
            if !self.closed {
                self.last_discarded = Some(notice);
            }
            return notice;
        }
        let mut work = slot.take().unwrap();
        // A real result or unwind fallback was enqueued. Capture/thread cleanup
        // may still be running. No state lock is held; join the worker, never
        // an accepted application, and dispose any join payload outside unwind.
        join_worker(work.handle.take().unwrap());
        match completion {
            Completion::Refresh(_, result) => {
                self.refresh_status = match result {
                    _ if work.cancel.is_cancelled() => Status::Cancelled(id),
                    Ok(catalog) => {
                        let value = RefreshSummary {
                            entries: catalog.entries.len(),
                            issues: catalog.issues.len(),
                            truncated: catalog.truncated,
                        };
                        self.snapshot = Some(Snapshot {
                            generation: id,
                            catalog,
                        });
                        self.selection = None;
                        Status::Succeeded { id, value }
                    }
                    Err(ConsumerError::Service(error)) if error.kind == ErrorKind::Cancelled => {
                        Status::Cancelled(id)
                    }
                    Err(error) => Status::Failed { id, error },
                };
            }
            Completion::Launch(_, result) => {
                self.launch_status = match result {
                    Ok(value) => Status::Succeeded { id, value },
                    Err(ConsumerError::Service(error)) if error.kind == ErrorKind::Cancelled => {
                        Status::Cancelled(id)
                    }
                    Err(error) => Status::Failed { id, error },
                }
            }
        }
        notice
    }
    /// Bounded, caller-driven drain; no internal polling/timer subscription.
    pub fn drain_ready(&mut self) -> Result<Vec<CompletionNotice>, ConsumerError> {
        self.open()?;
        let mut notices = Vec::new();
        for _ in 0..2 {
            match self.receiver.try_recv() {
                Ok(completion) => notices.push(self.finish(completion)),
                Err(_) => break,
            }
        }
        Ok(notices)
    }
    /// Block for one completion on a headless caller thread; pending work alone
    /// returns None rather than waiting for a worker that was never started.
    pub fn wait(&mut self) -> Result<Option<CompletionNotice>, ConsumerError> {
        self.open()?;
        if ![&self.refresh_work, &self.launch_work]
            .iter()
            .any(|w| w.as_ref().is_some_and(|w| w.handle.is_some()))
        {
            return Ok(None);
        }
        let completion = self
            .receiver
            .recv()
            .map_err(|_| ConsumerError::WorkerPanicked)?;
        Ok(Some(self.finish(completion)))
    }
    /// Nonblocking disposal begins here. No results can update the snapshot
    /// afterwards. shutdown/Drop must still join all retained running workers.
    pub fn begin_shutdown(&mut self) {
        if self.closed {
            return;
        }
        self.closed = true;
        for slot in [&mut self.refresh_work, &mut self.launch_work] {
            if let Some(work) = slot {
                work.cancel.cancel();
                if work.handle.is_none() {
                    *slot = None;
                }
            }
        }
        self.snapshot = None;
        self.selection = None;
        self.refresh_status = Status::Disposed;
        self.launch_status = Status::Disposed;
    }
    /// Idempotent join; cannot preempt blocked filesystem/COM/CreateProcess calls.
    /// Accepted applications continue; these are worker joins, not app termination.
    pub fn shutdown(&mut self) {
        self.begin_shutdown();
        for mut work in [self.refresh_work.take(), self.launch_work.take()]
            .into_iter()
            .flatten()
        {
            if let Some(handle) = work.handle.take() {
                join_worker(handle);
            }
        }
        while self.receiver.try_recv().is_ok() {}
    }
}
impl Drop for Consumer {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests;
