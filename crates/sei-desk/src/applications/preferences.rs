//! Validated, versioned pins; no launch paths or argument material is persisted.
pub mod worker;
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};
pub const MAX_PINS: usize = 256;
pub const MAX_LABEL: usize = 1024;
pub const MAX_BYTES: u64 = 600_000;
const HEADER: &str = "sei-desk-pins-v1\n";
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pin {
    pub token: String,
    pub name: String,
}
fn validate(pins: &[Pin]) -> Result<(), String> {
    if pins.len() > MAX_PINS {
        return Err("At most 256 applications can be pinned.".into());
    }
    let mut seen = BTreeSet::new();
    for pin in pins {
        if pin.token.len() != 64
            || !pin
                .token
                .bytes()
                .all(|v| v.is_ascii_digit() || (b'a'..=b'f').contains(&v))
            || pin.name.trim().is_empty()
            || pin.name.len() > MAX_LABEL
            || pin.name.chars().any(char::is_control)
            || !seen.insert(&pin.token)
        {
            return Err("Pins contain an invalid or duplicate identity or label.".into());
        }
    }
    Ok(())
}
pub fn encode(pins: &[Pin]) -> Result<Vec<u8>, String> {
    validate(pins)?;
    let mut text = String::from(HEADER);
    for pin in pins {
        text.push_str(&pin.token);
        text.push('\t');
        text.push_str(&pin.name);
        text.push('\n');
    }
    Ok(text.into_bytes())
}
pub fn decode(bytes: &[u8]) -> Result<Vec<Pin>, String> {
    if bytes.len() as u64 > MAX_BYTES {
        return Err("Pins file exceeds the size limit.".into());
    }
    let text = std::str::from_utf8(bytes).map_err(|_| "Pins file is not UTF-8.")?;
    let body = text
        .strip_prefix(HEADER)
        .ok_or("Pins file is malformed or uses an unsupported version.")?;
    if !body.is_empty() && !body.ends_with('\n') {
        return Err("Pins file is incomplete.".into());
    }
    let mut pins = Vec::new();
    for line in body.lines() {
        if pins.len() == MAX_PINS {
            return Err("Pins file contains too many pins.".into());
        }
        let (token, name) = line.split_once('\t').ok_or("Invalid pin record.")?;
        pins.push(Pin {
            token: token.into(),
            name: name.into(),
        });
    }
    validate(&pins)?;
    Ok(pins)
}
pub fn user_path() -> Result<PathBuf, String> {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .ok_or("LOCALAPPDATA is unavailable; favorites cannot be saved.")?;
    Ok(base.join("SeiDesk").join("pins-v1.txt"))
}
fn read(path: &Path) -> Result<Option<Vec<u8>>, String> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("Cannot read pins: {e}")),
    };
    let mut bytes = Vec::new();
    file.take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("Cannot read pins: {e}"))?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err(
            "Pins file exceeds 600 KB; the original is preserved. Move it aside before restarting."
                .into(),
        );
    }
    Ok(Some(bytes))
}
pub struct Store {
    path: PathBuf,
    expected: Option<Vec<u8>>,
    pub writable: bool,
}
impl Store {
    pub fn load(path: PathBuf) -> (Self, Vec<Pin>, Option<String>) {
        let mut store = Self {
            path,
            expected: None,
            writable: true,
        };
        match read(&store.path) {
            Ok(bytes) => {
                store.expected = bytes;
                if let Some(bytes) = &store.expected {
                    match decode(bytes) {
                        Ok(pins) => return (store, pins, None),
                        Err(error) => {
                            // Unknown versions may be valid future data; never rewrite them.
                            if !bytes.starts_with(HEADER.as_bytes()) {
                                store.writable = false;
                                return (
                                    store,
                                    vec![],
                                    Some(format!(
                                        "{error} Original preserved; move it aside before restarting."
                                    )),
                                );
                            }
                            match backup(&store.path, bytes) {
                                Ok(path) => {
                                    return (
                                        store,
                                        vec![],
                                        Some(format!(
                                            "{error} Recovery copy saved as {}. You may save new pins.",
                                            path.file_name().unwrap_or_default().to_string_lossy()
                                        )),
                                    );
                                }
                                Err(e) => {
                                    store.writable = false;
                                    return (
                                        store,
                                        vec![],
                                        Some(format!("{error} {e} Original preserved.")),
                                    );
                                }
                            }
                        }
                    }
                }
                (store, vec![], None)
            }
            Err(error) => {
                store.writable = false;
                (store, vec![], Some(error))
            }
        }
    }
    pub fn save(&mut self, pins: &[Pin]) -> Result<(), String> {
        if !self.writable {
            return Err("Pins are read-only until the configuration problem is resolved and Sei-Desk is restarted.".into());
        }
        let bytes = encode(pins)?;
        let parent = self.path.parent().ok_or("Invalid pins location.")?;
        fs::create_dir_all(parent).map_err(|e| format!("Cannot create pins directory: {e}"))?;
        let lock_path = self.path.with_extension("lock");
        let lock_file=OpenOptions::new().write(true).create_new(true).open(&lock_path)
            .map_err(|e|format!("Pins are locked by another instance or a previous interrupted save. Restart after resolving {}: {e}",lock_path.display()))?;
        let _lock = Temporary {
            path: lock_path,
            _file: Some(lock_file),
        };
        if read(&self.path)? != self.expected {
            return Err(
                "Pins changed outside this window. Restart Sei-Desk to reload them before saving."
                    .into(),
            );
        }
        let temp_path = self.path.with_extension("pending");
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
            .map_err(|e| {
                format!("Cannot create temporary pins file; existing pins retained: {e}")
            })?;
        let mut temporary = Temporary {
            path: temp_path,
            _file: Some(file),
        };
        let file = temporary._file.as_mut().expect("owned temporary file");
        file.write_all(&bytes)
            .and_then(|()| file.sync_all())
            .map_err(|e| format!("Cannot write pins; existing pins retained: {e}"))?;
        temporary._file.take();
        replace(&temporary.path, &self.path)
            .map_err(|e| format!("Cannot replace pins; existing pins retained: {e}"))?;
        self.expected = Some(bytes);
        Ok(())
    }
}
struct Temporary {
    path: PathBuf,
    _file: Option<File>,
}
impl Drop for Temporary {
    fn drop(&mut self) {
        self._file.take();
        let _ = fs::remove_file(&self.path);
    }
}
fn backup(path: &Path, bytes: &[u8]) -> Result<PathBuf, String> {
    // Fixed slots bound new recovery files across process lifetimes. Reuse an
    // identical complete copy so repeated starts with unchanged invalid data
    // neither grow the directory nor exhaust recovery capacity.
    for index in 0..32 {
        let candidate = path.with_extension(format!("invalid-{index}"));
        if matches!(read(&candidate), Ok(Some(existing)) if existing == bytes) {
            return Ok(candidate);
        }
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(mut f) => {
                f.write_all(bytes)
                    .and_then(|()| f.sync_all())
                    .map_err(|e| format!("Cannot preserve invalid pins: {e}"))?;
                return Ok(candidate);
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(format!("Cannot preserve invalid pins: {e}")),
        }
    }
    Err("Recovery-copy slots are occupied; original retained.".into())
}
#[cfg(windows)]
fn replace(from: &Path, to: &Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn MoveFileExW(from: *const u16, to: *const u16, flags: u32) -> i32;
    }
    let from: Vec<u16> = from.as_os_str().encode_wide().chain(Some(0)).collect();
    let to: Vec<u16> = to.as_os_str().encode_wide().chain(Some(0)).collect();
    // SAFETY: live NUL-terminated paths; synchronous call retains no pointers.
    if unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), 0x1 | 0x8) } == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}
#[cfg(not(windows))]
fn replace(from: &Path, to: &Path) -> std::io::Result<()> {
    fs::rename(from, to)
}

#[cfg(test)]
mod tests;
