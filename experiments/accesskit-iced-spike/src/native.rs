//! All HWND operations and thread-affine provider ownership live here.
use accesskit::{ActionHandler, ActionRequest, ActivationHandler, TreeUpdate};
use accesskit_windows::{HWND, SubclassingAdapter};
use iced::futures::channel::mpsc;
use std::{
    cell::RefCell,
    sync::{Arc, Mutex, OnceLock},
};
use windows_sys::Win32::UI::HiDpi::GetDpiForWindow;
use windows_sys::Win32::{
    Foundation::HWND as RawHwnd,
    System::Threading::GetCurrentThreadId,
    UI::WindowsAndMessaging::{GetWindowThreadProcessId, IsWindowVisible},
};

static RECEIVER: OnceLock<Mutex<Option<mpsc::Receiver<ActionRequest>>>> = OnceLock::new();
thread_local! { static OWNER: RefCell<Option<Owner>> = const { RefCell::new(None) }; }
struct Owner {
    hwnd: usize,
    adapter: SubclassingAdapter,
    snapshot: Arc<Mutex<TreeUpdate>>,
    generation: u64,
}
struct Initial(Arc<Mutex<TreeUpdate>>);
impl ActivationHandler for Initial {
    fn request_initial_tree(&mut self) -> Option<TreeUpdate> {
        self.0.lock().ok().map(|tree| tree.clone())
    }
}
struct Actions(mpsc::Sender<ActionRequest>);
impl ActionHandler for Actions {
    fn do_action(&mut self, request: ActionRequest) {
        // Bounded nonblocking queue. No UI-thread call, lock or model mutation.
        if self.0.try_send(request).is_err() {
            eprintln!("accessibility_action_dropped: queue full or closed");
        }
    }
}
pub fn prepare() -> mpsc::Sender<ActionRequest> {
    let (sender, receiver) = mpsc::channel(32);
    assert!(RECEIVER.set(Mutex::new(Some(receiver))).is_ok());
    sender
}
pub fn actions() -> mpsc::Receiver<ActionRequest> {
    RECEIVER
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .take()
        .expect("single stable subscription")
}

pub fn install(
    hwnd: usize,
    initial: TreeUpdate,
    sender: mpsc::Sender<ActionRequest>,
) -> Result<(), String> {
    let raw = hwnd as RawHwnd;
    // SAFETY: Handle comes from iced's live window callback. Queries only; no foreign HWND is accepted.
    unsafe {
        if GetWindowThreadProcessId(raw, std::ptr::null_mut()) != GetCurrentThreadId() {
            return Err("not the owning UI thread".into());
        }
        if IsWindowVisible(raw) != 0 {
            return Err("window already visible; refusing late attachment".into());
        }
    }
    OWNER.with(|owner| {
        let mut owner = owner.borrow_mut();
        if owner.is_some() {
            return Err("provider already installed".into());
        }
        let snapshot = Arc::new(Mutex::new(initial));
        let adapter =
            SubclassingAdapter::new(HWND(raw), Initial(snapshot.clone()), Actions(sender));
        *owner = Some(Owner {
            hwnd,
            adapter,
            snapshot,
            generation: 0,
        });
        eprintln!("provider_attached_before_show hwnd={hwnd}");
        Ok(())
    })
}
pub fn scale() -> f64 {
    OWNER.with(|owner| {
        owner.borrow().as_ref().map_or(1.0, |owner| {
            // SAFETY: Thread-affine owner retains the live iced HWND until detach; query only.
            let dpi = unsafe { GetDpiForWindow(owner.hwnd as RawHwnd) };
            f64::from(dpi.max(96)) / 96.0
        })
    })
}
pub fn publish(generation: u64, next: TreeUpdate) {
    let events = OWNER.with(|owner| {
        let mut owner = owner.borrow_mut();
        let owner = owner.as_mut()?;
        if generation < owner.generation {
            return None;
        }
        owner.generation = generation;
        let delta = {
            let mut snapshot = owner.snapshot.lock().expect("snapshot poisoned");
            let delta = crate::semantics::incremental(&snapshot, &next);
            *snapshot = next;
            delta
        };
        owner.adapter.update_if_active(|| delta)
    });
    // Release owner/snapshot borrows before external provider/event callbacks.
    if let Some(events) = events {
        events.raise();
    }
}
pub fn detach() {
    let owner = OWNER.with(|owner| owner.borrow_mut().take());
    drop(owner);
    eprintln!("provider_detached");
}
