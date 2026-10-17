use super::*;
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Dir(PathBuf);
impl Dir {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "sei-pins-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn file(&self) -> PathBuf {
        self.0.join("pins.txt")
    }
}
impl Drop for Dir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
fn pin() -> Pin {
    Pin {
        token: "a".repeat(64),
        name: "Béta Notes".into(),
    }
}
#[test]
fn first_run_and_valid_round_trip_do_not_store_launch_material() {
    let dir = Dir::new();
    let (mut store, pins, error) = Store::load(dir.file());
    assert!(pins.is_empty());
    assert!(error.is_none());
    store.save(&[pin()]).unwrap();
    let (_, loaded, error) = Store::load(dir.file());
    assert_eq!(loaded, vec![pin()]);
    assert!(error.is_none());
    assert_eq!(
        fs::read_to_string(dir.file()).unwrap(),
        format!("sei-desk-pins-v1\n{}\tBéta Notes\n", "a".repeat(64))
    );
}
#[test]
fn malformed_partial_and_invalid_utf8_are_rejected() {
    for bytes in [
        b"sei-desk-pins-v1\na\tname\n".as_slice(),
        b"sei-desk-pins-v1\npartial",
        b"sei-desk-pins-v1\n\xff\n",
    ] {
        assert!(decode(bytes).is_err());
    }
    assert!(decode(&vec![b'a'; MAX_BYTES as usize + 1]).is_err());
}
#[test]
fn duplicate_ids_bad_labels_and_pin_limits_are_validated() {
    assert!(encode(&[pin(), pin()]).is_err());
    for token in ["".into(), "f".repeat(63), "G".repeat(64)] {
        assert!(
            encode(&[Pin {
                token,
                name: "Utility".into()
            }])
            .is_err()
        );
    }
    for name in [
        "".into(),
        "line\nbreak".into(),
        "tab\there".into(),
        "x".repeat(MAX_LABEL + 1),
    ] {
        assert!(
            encode(&[Pin {
                token: "b".repeat(64),
                name
            }])
            .is_err()
        );
    }
    let pins: Vec<_> = (0..MAX_PINS)
        .map(|i| Pin {
            token: format!("{i:064x}"),
            name: "Utility".into(),
        })
        .collect();
    assert_eq!(decode(&encode(&pins).unwrap()).unwrap(), pins);
    let mut extra = pins;
    extra.push(Pin {
        token: "f".repeat(64),
        name: "Extra".into(),
    });
    assert!(encode(&extra).is_err());
}
#[test]
fn invalid_v1_is_backed_up_before_a_new_valid_save() {
    let dir = Dir::new();
    let bad = b"sei-desk-pins-v1\npartial";
    fs::write(dir.file(), bad).unwrap();
    let (mut store, pins, error) = Store::load(dir.file());
    assert!(pins.is_empty());
    assert!(store.writable);
    assert!(error.unwrap().contains("Recovery copy"));
    store.save(&[pin()]).unwrap();
    let backup = fs::read_dir(&dir.0)
        .unwrap()
        .map(|e| e.unwrap().path())
        .find(|p| {
            p.extension()
                .unwrap_or_default()
                .to_string_lossy()
                .starts_with("invalid-")
        })
        .unwrap();
    assert_eq!(fs::read(backup).unwrap(), bad);
}
#[test]
fn future_version_oversize_and_unreadable_config_are_preserved_read_only() {
    for content in [
        b"sei-desk-pins-v2\nfuture".to_vec(),
        vec![b'x'; MAX_BYTES as usize + 1],
    ] {
        let dir = Dir::new();
        fs::write(dir.file(), &content).unwrap();
        let (mut store, _, error) = Store::load(dir.file());
        assert!(error.is_some());
        assert!(!store.writable);
        assert!(store.save(&[pin()]).is_err());
        assert_eq!(fs::read(dir.file()).unwrap(), content);
    }
    let dir = Dir::new();
    fs::create_dir(dir.file()).unwrap();
    let (store, _, error) = Store::load(dir.file());
    assert!(!store.writable);
    assert!(error.is_some());
}
#[test]
fn interrupted_temp_and_lock_do_not_erase_valid_pins_or_foreign_files() {
    let dir = Dir::new();
    let (mut store, _, _) = Store::load(dir.file());
    store.save(&[pin()]).unwrap();
    let original = fs::read(dir.file()).unwrap();
    for extension in ["pending", "lock"] {
        let path = dir.file().with_extension(extension);
        fs::write(&path, b"previous interrupted write").unwrap();
        assert!(store.save(&[]).is_err());
        assert_eq!(fs::read(dir.file()).unwrap(), original);
        assert!(path.exists());
        fs::remove_file(path).unwrap();
    }
}
#[test]
fn atomic_replacement_reloads_and_cleans_owned_temporary_files() {
    let dir = Dir::new();
    let (mut store, _, _) = Store::load(dir.file());
    store.save(&[pin()]).unwrap();
    store.save(&[]).unwrap();
    assert!(decode(&fs::read(dir.file()).unwrap()).unwrap().is_empty());
    assert_eq!(fs::read_dir(&dir.0).unwrap().count(), 1);
}
#[test]
fn another_instance_cannot_overwrite_newer_configuration() {
    let dir = Dir::new();
    let (mut first, _, _) = Store::load(dir.file());
    let (mut second, _, _) = Store::load(dir.file());
    first.save(&[pin()]).unwrap();
    assert!(second.save(&[]).unwrap_err().contains("changed outside"));
    assert_eq!(decode(&fs::read(dir.file()).unwrap()).unwrap(), vec![pin()]);
    assert!(!dir.file().with_extension("lock").exists());
}
#[cfg(windows)]
#[test]
fn failed_native_replace_preserves_last_file_and_cleans_temp() {
    use std::os::windows::fs::OpenOptionsExt;
    let dir = Dir::new();
    let (mut store, _, _) = Store::load(dir.file());
    store.save(&[pin()]).unwrap();
    let held = OpenOptions::new()
        .read(true)
        .share_mode(3)
        .open(dir.file())
        .unwrap();
    assert!(store.save(&[]).is_err());
    assert_eq!(decode(&fs::read(dir.file()).unwrap()).unwrap(), vec![pin()]);
    assert!(!dir.file().with_extension("pending").exists());
    assert!(!dir.file().with_extension("lock").exists());
    drop(held);
    store.save(&[]).unwrap();
}
