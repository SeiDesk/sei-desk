//! Display-only projection and pure filtering/selection. Launch targets remain
//! exclusively in sei-apps Consumer; UI keys contain identity + generation.
mod browser;
mod identity;
pub mod preferences;
pub mod reveal;
pub mod runtime;
pub mod search;
pub use browser::{Grouping, Presentation, Sort};
use sei_apps::consumer::{Accepted, RefreshSummary, Status};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    pub generation: u64,
    pub id: String,
}
#[derive(Debug)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub executable: String,
    pub shortcut: String,
    pub source: &'static str,
    pub directory: Option<String>,
    pub token: String,
    search: String,
}
#[derive(Debug)]
pub struct Catalog {
    pub generation: u64,
    pub entries: Vec<Entry>,
    pub issues: usize,
    pub truncated: bool,
}
impl Catalog {
    fn project(snapshot: &sei_apps::consumer::Snapshot) -> Self {
        Self {
            generation: snapshot.generation.get(),
            issues: snapshot.catalog.issues.len(),
            truncated: snapshot.catalog.truncated,
            entries: snapshot
                .catalog
                .entries
                .iter()
                .map(|entry| {
                    let filename = entry
                        .request
                        .executable
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy();
                    Entry {
                        token: identity::token(&entry.id),
                        id: entry.id.clone(),
                        name: entry.display_name.clone(),
                        executable: entry.request.executable.display().to_string(),
                        shortcut: entry.shortcut.display().to_string(),
                        source: match entry.source {
                            sei_apps::Source::UserStartMenu => "User Start menu",
                            sei_apps::Source::CommonStartMenu => "Shared Start menu",
                        },
                        directory: entry
                            .request
                            .working_directory
                            .as_ref()
                            .map(|p| p.display().to_string()),
                        search: format!("{}\n{}", entry.display_name, filename).to_lowercase(),
                    }
                })
                .collect(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Frame {
    pub revision: u64,
    pub ack: u64,
    pub catalog: Option<Arc<Catalog>>,
    pub refresh: Status<RefreshSummary>,
    pub launch: Status<Accepted>,
    pub launch_name: Option<String>,
    pub error: Option<sei_apps::consumer::ConsumerError>,
    pub stale: bool,
    pub cancel_pending: bool,
    /// Zero denotes management requests; companion lifetimes use unique tokens.
    pub launch_origin: u64,
    pub refresh_origin: u64,
    pub error_origin: u64,
}
impl Default for Frame {
    fn default() -> Self {
        Self {
            revision: 0,
            ack: 0,
            catalog: None,
            refresh: Status::Idle,
            launch: Status::Idle,
            launch_name: None,
            error: None,
            stale: false,
            cancel_pending: false,
            launch_origin: 0,
            refresh_origin: 0,
            error_origin: 0,
        }
    }
}
pub fn busy<T>(status: &Status<T>) -> bool {
    matches!(status, Status::Pending(_) | Status::Running { .. })
}
pub fn error_text(error: &sei_apps::consumer::ConsumerError) -> String {
    use sei_apps::{
        ErrorKind,
        consumer::{ConsumerError, Operation},
    };
    match error {
        ConsumerError::Service(error) => {
            let explanation = match error.kind {
                _ if error.code == Some(740) => {
                    "Administrator permission is required. Open this application through Windows with the appropriate permission."
                }
                ErrorKind::Missing => {
                    "The application or working directory is missing. Refresh the catalog and try again."
                }
                ErrorKind::AccessDenied => "Windows denied access to this application or source.",
                ErrorKind::Invalid => "Windows could not use this application target.",
                ErrorKind::Unsupported => "This application target is not supported.",
                ErrorKind::Cancelled => "The operation was cancelled.",
                ErrorKind::Limit => "The service reached its scan limit.",
                ErrorKind::Native => "Windows could not complete the operation.",
                ErrorKind::ProcessCreation => "Windows could not start this application.",
            };
            error.code.map_or_else(
                || explanation.into(),
                |code| format!("{explanation} (Windows error {code})"),
            )
        }
        ConsumerError::NoSelection => "Select a current application before launching.".into(),
        ConsumerError::UnknownEntry | ConsumerError::StaleSelection => {
            "The catalog changed. Select an application from the current results.".into()
        }
        ConsumerError::Busy(Operation::Refresh) => "A catalog refresh is already running.".into(),
        ConsumerError::Busy(Operation::Launch) => "A launch is already pending.".into(),
        ConsumerError::Closed => "Applications is shutting down.".into(),
        ConsumerError::WorkerPanicked => {
            "The application service could not finish. Try again.".into()
        }
        ConsumerError::ThreadStart { .. } => {
            "Could not start the background application worker. Try again.".into()
        }
        ConsumerError::IdExhausted => "Reopen Sei-Desk to continue application operations.".into(),
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Search,
    Filter,
    Presentation,
    Sort,
    Group,
    Results,
    Refresh,
    Launch,
    Cancel,
    Pin,
}
#[derive(Debug, Clone)]
pub enum Action {
    Query(String),
    Select(String),
    Move(i32),
    Refresh,
    Launch,
    Submit,
    Cancel,
    Clear,
    Filter,
    Presentation,
    Sort,
    Group,
    Pin,
    Missing(String),
}
pub struct Model {
    pub frame: Arc<Frame>,
    pub query: String,
    pub filtered: Vec<usize>,
    pub selected: Option<String>,
    pub focus: Option<Focus>,
    pub local_error: Option<String>,
    pub requested: bool,
    refresh_sent: Option<u64>,
    launch_sent: Option<u64>,
    pub pins: Arc<preferences::worker::State>,
    pub pins_pending: bool,
    pub pinned_only: bool,
    pub presentation: Presentation,
    pub sort: Sort,
    pub grouping: Grouping,
    pub missing: Vec<usize>,
    pub missing_selected: Option<String>,
}
impl Default for Model {
    fn default() -> Self {
        Self {
            frame: Arc::new(Frame::default()),
            query: String::new(),
            filtered: Vec::new(),
            selected: None,
            focus: None,
            local_error: None,
            requested: false,
            refresh_sent: None,
            launch_sent: None,
            pins: Arc::new(preferences::worker::State::default()),
            pins_pending: false,
            pinned_only: false,
            presentation: Presentation::List,
            sort: Sort::Name,
            grouping: Grouping::None,
            missing: Vec::new(),
            missing_selected: None,
        }
    }
}
impl Model {
    pub fn apply_frame(&mut self, frame: Arc<Frame>) {
        if frame.revision <= self.frame.revision {
            return;
        }
        let changed = self.frame.catalog.as_ref().map(|c| c.generation)
            != frame.catalog.as_ref().map(|c| c.generation);
        self.frame = frame;
        if changed {
            self.filter();
        }
        if self.refresh_sent.is_some_and(|seq| self.frame.ack >= seq) {
            self.refresh_sent = None;
        }
        if self.launch_sent.is_some_and(|seq| self.frame.ack >= seq) {
            self.launch_sent = None;
        }
    }
    pub fn edit(&mut self, query: String) {
        self.query = query.chars().take(256).collect();
        self.filter();
    }
    pub fn filter(&mut self) {
        let query = self.query.trim().to_lowercase();
        self.filtered = self
            .frame
            .catalog
            .as_ref()
            .map(|c| {
                c.entries
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| {
                        e.search.contains(&query) && (!self.pinned_only || self.is_pinned(e))
                    })
                    .map(|(index, _)| index)
                    .collect()
            })
            .unwrap_or_default();
        self.order();
        self.missing = if self.pinned_only {
            self.pins
                .pins
                .iter()
                .enumerate()
                .filter(|(_, pin)| {
                    pin.name.to_lowercase().contains(&query)
                        && !self
                            .frame
                            .catalog
                            .as_ref()
                            .is_some_and(|c| c.entries.iter().any(|e| e.token == pin.token))
                })
                .map(|(i, _)| i)
                .collect()
        } else {
            vec![]
        };
        self.order_missing();
        let remembered = self
            .missing_selected
            .clone()
            .or_else(|| self.selected.as_deref().map(identity::token));
        if let Some(token) = remembered {
            if self
                .missing
                .iter()
                .any(|&i| self.pins.pins[i].token == token)
            {
                self.selected = None;
                self.missing_selected = Some(token);
                return;
            }
            if self.missing_selected.is_some()
                && let Some(catalog) = &self.frame.catalog
                && let Some(&index) = self
                    .filtered
                    .iter()
                    .find(|&&i| catalog.entries[i].token == token)
            {
                self.selected = Some(catalog.entries[index].id.clone());
            }
        }
        self.missing_selected = None;
        if !self.filtered.iter().any(|&i| {
            self.frame
                .catalog
                .as_ref()
                .is_some_and(|c| Some(&c.entries[i].id) == self.selected.as_ref())
        }) {
            self.selected = self
                .filtered
                .first()
                .and_then(|&i| self.frame.catalog.as_ref().map(|c| c.entries[i].id.clone()));
            if self.selected.is_none() {
                self.missing_selected = self
                    .missing
                    .first()
                    .map(|&i| self.pins.pins[i].token.clone());
            }
        }
    }
    pub fn selected(&self) -> Option<&Entry> {
        if self.missing_selected.is_some() {
            return None;
        }
        self.frame.catalog.as_ref()?.entries.iter().find(|entry| {
            Some(&entry.id) == self.selected.as_ref()
                && self.filtered.iter().any(|&i| {
                    self.frame
                        .catalog
                        .as_ref()
                        .is_some_and(|c| c.entries[i].id == entry.id)
                })
        })
    }
    pub fn key(&self) -> Option<Key> {
        Some(Key {
            generation: self.frame.catalog.as_ref()?.generation,
            id: self.selected()?.id.clone(),
        })
    }
    pub fn move_selection(&mut self, direction: i32) {
        let len = self.filtered.len() + self.missing.len();
        if len == 0 {
            self.selected = None;
            self.missing_selected = None;
            return;
        }
        let index = self
            .missing_selected
            .as_ref()
            .and_then(|t| {
                self.missing
                    .iter()
                    .position(|&i| self.pins.pins[i].token == *t)
                    .map(|i| i + self.filtered.len())
            })
            .or_else(|| {
                self.filtered.iter().position(|&i| {
                    self.frame
                        .catalog
                        .as_ref()
                        .is_some_and(|c| Some(&c.entries[i].id) == self.selected.as_ref())
                })
            });
        let next = index.map_or(0, |index| {
            (index as i64 + i64::from(direction)).clamp(0, len as i64 - 1) as usize
        });
        if next >= self.filtered.len() {
            self.selected = None;
            self.missing_selected = Some(
                self.pins.pins[self.missing[next - self.filtered.len()]]
                    .token
                    .clone(),
            );
            return;
        }
        self.missing_selected = None;
        self.selected = self
            .frame
            .catalog
            .as_ref()
            .map(|c| c.entries[self.filtered[next]].id.clone());
    }
    pub fn position(&self) -> usize {
        if let Some(token) = &self.missing_selected {
            return self.filtered.len()
                + self
                    .missing
                    .iter()
                    .position(|&i| self.pins.pins[i].token == *token)
                    .unwrap_or(0);
        }
        self.filtered
            .iter()
            .position(|&i| {
                self.frame
                    .catalog
                    .as_ref()
                    .is_some_and(|c| Some(&c.entries[i].id) == self.selected.as_ref())
            })
            .unwrap_or(0)
    }
    pub fn refreshing(&self) -> bool {
        self.refresh_sent.is_some() || busy(&self.frame.refresh)
    }
    pub fn launching(&self) -> bool {
        self.launch_sent.is_some() || busy(&self.frame.launch)
    }
    pub fn mark_refresh(&mut self, seq: u64) {
        self.refresh_sent = Some(seq);
        self.requested = true;
    }
    pub fn mark_launch(&mut self, seq: u64) {
        self.launch_sent = Some(seq);
    }
}
#[cfg(test)]
pub(crate) mod tests;
