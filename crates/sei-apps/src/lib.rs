//! Headless, explicit-refresh application catalog. No iced or accessibility coupling.
mod catalog;
mod launch;
#[cfg(windows)]
mod native;
#[cfg(test)]
mod tests;

pub use catalog::{
    Catalog, Entry, Icon, Issue, Limits, LinkReader, Progress, Root, Shortcut, Source, discover,
};
pub use launch::{Arguments, LaunchReceipt, LaunchRequest, launch};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc::SyncSender,
};
use std::thread::JoinHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Cancelled,
    Missing,
    AccessDenied,
    Invalid,
    Unsupported,
    Limit,
    Native,
    ProcessCreation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub kind: ErrorKind,
    pub operation: &'static str,
    pub code: Option<i32>,
}
impl Error {
    pub(crate) fn new(kind: ErrorKind, operation: &'static str) -> Self {
        Self {
            kind,
            operation,
            code: None,
        }
    }
    pub(crate) fn io(operation: &'static str, error: std::io::Error) -> Self {
        let kind = match error.raw_os_error() {
            Some(2 | 3) => ErrorKind::Missing,
            Some(5 | 740) => ErrorKind::AccessDenied,
            Some(193 | 216) => ErrorKind::Invalid,
            _ => match error.kind() {
                std::io::ErrorKind::NotFound => ErrorKind::Missing,
                std::io::ErrorKind::PermissionDenied => ErrorKind::AccessDenied,
                _ if operation == "CreateProcess" => ErrorKind::ProcessCreation,
                _ => ErrorKind::Native,
            },
        };
        Self {
            kind,
            operation,
            code: error.raw_os_error(),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:?} (native code {:?})",
            self.operation, self.kind, self.code
        )
    }
}
impl std::error::Error for Error {}

#[derive(Clone, Default)]
pub struct Cancellation(Arc<AtomicBool>);
impl Cancellation {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
    pub(crate) fn check(&self) -> Result<(), Error> {
        if self.is_cancelled() {
            Err(Error::new(ErrorKind::Cancelled, "cancel"))
        } else {
            Ok(())
        }
    }
}

/// One finite worker; caller owns/join its handle. Progress is optional and lossy
/// under backpressure; the returned Catalog is authoritative. No implicit retries.
pub fn refresh_async(
    cancel: Cancellation,
    progress: Option<SyncSender<Progress>>,
) -> std::io::Result<JoinHandle<Result<Catalog, Error>>> {
    std::thread::Builder::new()
        .name("sei-catalog".into())
        .spawn(move || {
            cancel.check()?;
            #[cfg(windows)]
            {
                let mut reader = native::Reader::new()?;
                let (roots, initial_issues) = reader.roots();
                let mut catalog = discover(
                    &roots,
                    &mut reader,
                    Limits::default(),
                    &cancel,
                    progress.as_ref(),
                )?;
                catalog.issues.splice(0..0, initial_issues);
                Ok(catalog)
            }
            #[cfg(not(windows))]
            {
                let _ = progress;
                Err(Error::new(ErrorKind::Unsupported, "Windows catalog"))
            }
        })
}

/// Cancellation is checked before creation only. A successful spawn is never
/// reported as cancelled, even if cancellation races with process creation.
pub fn launch_async(
    request: LaunchRequest,
    cancel: Cancellation,
) -> std::io::Result<JoinHandle<Result<LaunchReceipt, Error>>> {
    std::thread::Builder::new()
        .name("sei-launch".into())
        .spawn(move || launch(request, &cancel))
}
