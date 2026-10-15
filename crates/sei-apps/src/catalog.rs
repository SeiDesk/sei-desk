use crate::{Cancellation, Error, ErrorKind, LaunchRequest};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::mpsc::SyncSender,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Source {
    UserStartMenu,
    CommonStartMenu,
}
#[derive(Debug, Clone)]
pub struct Root {
    pub source: Source,
    pub path: PathBuf,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Icon {
    pub path: PathBuf,
    pub index: i32,
}
#[derive(Debug, Clone)]
pub struct Shortcut {
    pub executable: PathBuf,
    /// Windows command-line suffix from IShellLink, never tokenized or shell-expanded.
    pub arguments: String,
    pub working_directory: Option<PathBuf>,
    pub icon: Option<Icon>,
}
pub trait LinkReader {
    fn read(&mut self, path: &Path) -> Result<Shortcut, Error>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: String,
    pub display_name: String,
    pub source: Source,
    pub shortcut: PathBuf,
    pub request: LaunchRequest,
    pub icon: Option<Icon>,
}
#[derive(Debug, Clone)]
pub struct Issue {
    pub source: Source,
    pub path: PathBuf,
    pub error: Error,
}
#[derive(Debug, Default)]
pub struct Catalog {
    pub entries: Vec<Entry>,
    pub issues: Vec<Issue>,
    pub visited: usize,
    pub truncated: bool,
}
#[derive(Debug, Clone, Copy)]
pub struct Progress {
    pub visited: usize,
    pub accepted: usize,
}
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_nodes: usize,
    pub max_depth: usize,
    pub max_link_bytes: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_nodes: 8192,
            max_depth: 8,
            max_link_bytes: 1024 * 1024,
        }
    }
}

pub(crate) fn path_key(path: &Path) -> Result<String, Error> {
    let normalized: PathBuf = path.components().collect();
    let value = normalized
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::Invalid, "path encoding"))?;
    Ok(value.replace('/', "\\").to_ascii_lowercase())
}
fn part(value: &str) -> String {
    format!("{}:{value}", value.len())
}
pub(crate) fn normalize(source: Source, path: PathBuf, link: Shortcut) -> Result<Entry, Error> {
    let display_name = path
        .file_stem()
        .and_then(|v| v.to_str())
        .unwrap_or("")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if display_name.is_empty() {
        return Err(Error::new(ErrorKind::Invalid, "display name"));
    }
    let request = LaunchRequest::shortcut(link.executable, link.arguments, link.working_directory)?;
    request.validate()?;
    let id = format!(
        "win-exe-v1:{}{}{}",
        part(&path_key(&request.executable)?),
        part(request.arguments.raw().unwrap_or("")),
        part(&path_key(
            request
                .working_directory
                .as_deref()
                .or_else(|| request.executable.parent())
                .unwrap_or(Path::new(""))
        )?)
    );
    Ok(Entry {
        id,
        display_name,
        source,
        shortcut: path,
        request,
        icon: link.icon,
    })
}

/// Blocking, bounded, injectable discovery for tests/custom explicit roots. The
/// production entry point is refresh_async. Never executes a shortcut or Resolve.
pub fn discover(
    roots: &[Root],
    reader: &mut impl LinkReader,
    limits: Limits,
    cancel: &Cancellation,
    progress: Option<&SyncSender<Progress>>,
) -> Result<Catalog, Error> {
    cancel.check()?;
    if limits.max_nodes == 0
        || limits.max_nodes > 8192
        || limits.max_depth > 8
        || limits.max_link_bytes > 1024 * 1024
    {
        return Err(Error::new(ErrorKind::Invalid, "discovery limits"));
    }
    let mut catalog = Catalog::default();
    let mut unique = BTreeMap::<String, Entry>::new();
    let mut roots = roots.to_vec();
    roots.sort_by_key(|r| (r.source, r.path.clone()));
    // The public fixture API is also bounded, not an arbitrary disk scanner.
    if roots.len() > 2 {
        return Err(Error::new(ErrorKind::Invalid, "root count"));
    }
    for root in roots {
        let mut stack = vec![(root.path, 0)];
        while let Some((directory, depth)) = stack.pop() {
            cancel.check()?;
            let mut issue = |path, error| {
                catalog.issues.push(Issue {
                    source: root.source,
                    path,
                    error,
                })
            };
            if let Err(error) = crate::launch::local_path(&directory) {
                issue(directory, error);
                continue;
            }
            match fs::symlink_metadata(&directory) {
                Ok(meta) if !reparse(&meta) && meta.is_dir() => (),
                Ok(_) => {
                    issue(
                        directory,
                        Error::new(ErrorKind::Unsupported, "reparse/non-directory source"),
                    );
                    continue;
                }
                Err(e) => {
                    issue(directory, Error::io("source metadata", e));
                    continue;
                }
            }
            let items = match fs::read_dir(&directory) {
                Ok(items) => items,
                Err(e) => {
                    issue(directory, Error::io("read source", e));
                    continue;
                }
            };
            let mut paths = Vec::new();
            for item in items {
                cancel.check()?;
                if catalog.visited >= limits.max_nodes {
                    catalog.truncated = true;
                    issue(
                        directory.clone(),
                        Error::new(ErrorKind::Limit, "node budget"),
                    );
                    break;
                }
                catalog.visited += 1;
                match item {
                    Ok(item) => paths.push(item.path()),
                    Err(e) => issue(directory.clone(), Error::io("read entry", e)),
                }
            }
            // Reject the overflowing directory as a whole: do not select an
            // arbitrary read_dir prefix whose order varies by filesystem.
            if catalog.truncated {
                break;
            }
            paths.sort();
            let mut directories = Vec::new();
            for path in paths {
                cancel.check()?;
                let meta = match fs::symlink_metadata(&path) {
                    Ok(m) => m,
                    Err(e) => {
                        issue(path, Error::io("entry metadata", e));
                        continue;
                    }
                };
                if reparse(&meta) {
                    issue(path, Error::new(ErrorKind::Unsupported, "reparse entry"));
                    continue;
                }
                if meta.is_dir() {
                    if depth < limits.max_depth {
                        directories.push((path, depth + 1));
                    } else {
                        issue(path, Error::new(ErrorKind::Limit, "depth budget"));
                    }
                    continue;
                }
                if !meta.is_file()
                    || !path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("lnk"))
                {
                    continue;
                }
                if meta.len() > limits.max_link_bytes {
                    issue(path, Error::new(ErrorKind::Limit, "shortcut size"));
                    continue;
                }
                match reader
                    .read(&path)
                    .and_then(|link| normalize(root.source, path.clone(), link))
                {
                    Ok(entry) => {
                        // Root precedence and sorted traversal pick one stable
                        // representative. Different args/cwd remain distinct.
                        unique.entry(entry.id.clone()).or_insert(entry);
                    }
                    Err(error) => issue(path, error),
                }
            }
            stack.extend(directories.into_iter().rev());
            if let Some(progress) = progress {
                let _ = progress.try_send(Progress {
                    visited: catalog.visited,
                    accepted: unique.len(),
                });
            }
        }
        if catalog.truncated {
            break;
        }
    }
    cancel.check()?;
    catalog.entries = unique.into_values().collect();
    catalog.entries.sort_by_key(|e| {
        (
            e.display_name.to_lowercase(),
            e.display_name.clone(),
            e.id.clone(),
        )
    });
    Ok(catalog)
}
pub(crate) fn reparse(meta: &fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        meta.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    {
        meta.file_type().is_symlink()
    }
}
