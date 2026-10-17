//! One bounded file worker. UI commits pins only after a successful save receipt.
use super::{Pin, Store, user_path};
use iced::futures::{Stream, task::AtomicWaker};
use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
    sync::{
        Arc, Mutex,
        mpsc::{self, SyncSender},
    },
    task::Poll,
    thread::JoinHandle,
};
#[derive(Debug, Clone, Default)]
pub struct State {
    pub revision: u64,
    pub pins: Arc<Vec<Pin>>,
    pub writable: bool,
    pub error: Option<String>,
}
#[derive(Default)]
struct Latest {
    state: Mutex<Option<Arc<State>>>,
    waker: AtomicWaker,
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
pub fn stream(feed: &Feed) -> impl Stream<Item = Arc<State>> + use<> {
    let latest = feed.0.clone();
    iced::futures::stream::poll_fn(move |cx| {
        latest.waker.register(cx.waker());
        latest
            .state
            .lock()
            .expect("pins state poisoned")
            .take()
            .map_or(Poll::Pending, |state| Poll::Ready(Some(state)))
    })
}
enum Command {
    Save(Vec<Pin>),
    Stop,
}
#[derive(Clone)]
pub struct Handle {
    sender: SyncSender<Command>,
    feed: Feed,
}
impl Handle {
    pub fn feed(&self) -> Feed {
        self.feed.clone()
    }
    pub fn save(&self, pins: Vec<Pin>) -> Result<(), String> {
        super::validate(&pins)?;
        self.sender
            .try_send(Command::Save(pins))
            .map_err(|_| "Pins worker is busy or unavailable. Try again.".into())
    }
}
pub struct Owner {
    handle: Handle,
    worker: Option<JoinHandle<()>>,
}
impl Owner {
    pub fn start() -> std::io::Result<Self> {
        Self::at(user_path())
    }
    fn at(path: Result<PathBuf, String>) -> std::io::Result<Self> {
        let (sender, receiver) = mpsc::sync_channel(1);
        let feed = Feed(Arc::new(Latest::default()));
        let handle = Handle {
            sender,
            feed: feed.clone(),
        };
        let worker = std::thread::Builder::new()
            .name("sei-pins".into())
            .spawn(move || {
                let (mut store, pins, error) = match path {
                    Ok(path) => {
                        let (store, pins, error) = Store::load(path);
                        (Some(store), pins, error)
                    }
                    Err(error) => (None, vec![], Some(error)),
                };
                let mut state = State {
                    revision: 1,
                    pins: Arc::new(pins),
                    writable: store.as_ref().is_some_and(|s| s.writable),
                    error,
                };
                let publish = |state: &State| {
                    *feed.0.state.lock().expect("pins state poisoned") =
                        Some(Arc::new(state.clone()));
                    feed.0.waker.wake();
                };
                publish(&state);
                while let Ok(Command::Save(pins)) = receiver.recv() {
                    match store
                        .as_mut()
                        .ok_or_else(|| "Pins storage unavailable.".to_string())
                        .and_then(|s| s.save(&pins))
                    {
                        Ok(()) => {
                            state.pins = Arc::new(pins);
                            state.error = None
                        }
                        Err(error) => state.error = Some(error),
                    }
                    state.revision += 1;
                    publish(&state);
                }
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
impl Drop for Owner {
    fn drop(&mut self) {
        // This owner is dropped only after the UI event loop. A pending save is
        // allowed to finish; Stop is bounded and the exact worker is joined.
        let _ = self.handle.sender.send(Command::Stop);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use iced::futures::StreamExt;
    #[test]
    fn mailbox_coalesces_and_shutdown_flushes_accepted_save() {
        let root = std::env::temp_dir().join(format!("sei-pins-worker-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("pins.txt");
        let owner = Owner::at(Ok(path.clone())).unwrap();
        let handle = owner.handle();
        let mut stream = Box::pin(stream(&handle.feed()));
        let state = iced::futures::executor::block_on(stream.next()).unwrap();
        assert!(state.writable);
        let pin = Pin {
            token: "a".repeat(64),
            name: "Utility".into(),
        };
        handle.save(vec![pin.clone()]).unwrap();
        drop(owner);
        assert_eq!(
            super::super::decode(&std::fs::read(&path).unwrap()).unwrap(),
            vec![pin]
        );
        assert_eq!(
            handle
                .feed
                .0
                .state
                .lock()
                .unwrap()
                .as_ref()
                .unwrap()
                .revision,
            2
        );
        std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn failed_save_publishes_error_without_losing_confirmed_pins() {
        let root = std::env::temp_dir().join(format!("sei-pins-failure-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("pins.txt");
        let pin = Pin {
            token: "b".repeat(64),
            name: "Editor".into(),
        };
        std::fs::write(
            &path,
            super::super::encode(std::slice::from_ref(&pin)).unwrap(),
        )
        .unwrap();
        let owner = Owner::at(Ok(path.clone())).unwrap();
        let handle = owner.handle();
        let mut feed = Box::pin(stream(&handle.feed()));
        let loaded = iced::futures::executor::block_on(feed.next()).unwrap();
        assert_eq!(&*loaded.pins, std::slice::from_ref(&pin));
        std::fs::write(path.with_extension("pending"), b"interrupted").unwrap();
        handle.save(vec![]).unwrap();
        let failed = iced::futures::executor::block_on(feed.next()).unwrap();
        assert!(failed.error.is_some());
        assert_eq!(&*failed.pins, &[pin]);
        drop(owner);
        std::fs::remove_dir_all(root).unwrap();
    }
}
