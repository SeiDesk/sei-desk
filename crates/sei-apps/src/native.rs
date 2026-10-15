//! All COM unsafe boundaries are thread-local and scoped to a finite worker.
use crate::{Error, ErrorKind, Icon, Issue, LinkReader, Root, Shortcut, Source};
use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
    rc::Rc,
};
use windows::{
    Win32::{System::Com::*, UI::Shell::*},
    core::{Interface, PCWSTR},
};

pub(crate) struct Reader(PhantomData<Rc<()>>);
fn native_error(operation: &'static str, error: windows::core::Error) -> Error {
    let code = error.code().0;
    let kind = match code as u32 {
        0x80070002 | 0x80070003 => ErrorKind::Missing,
        0x80070005 => ErrorKind::AccessDenied,
        _ => ErrorKind::Native,
    };
    Error {
        kind,
        operation,
        code: Some(code),
    }
}
impl Reader {
    pub(crate) fn new() -> Result<Self, Error> {
        // SAFETY: Called on an operation-owned thread; guard is !Send/!Sync,
        // ensuring balanced uninitialization on this same thread, including S_FALSE.
        unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok() }
            .map_err(|e| native_error("CoInitializeEx", e))?;
        Ok(Self(PhantomData))
    }
    pub(crate) fn roots(&self) -> (Vec<Root>, Vec<Issue>) {
        let mut roots = Vec::new();
        let mut issues = Vec::new();
        for (source, id) in [
            (Source::UserStartMenu, FOLDERID_Programs),
            (Source::CommonStartMenu, FOLDERID_CommonPrograms),
        ] {
            // SAFETY: Valid known-folder GUID, current user, no folder creation.
            match unsafe { SHGetKnownFolderPath(&id, KF_FLAG_DONT_VERIFY, None) } {
                Ok(pointer) => {
                    // SAFETY: Successful API returns NUL-terminated task-allocated
                    // storage. Copy before freeing, including conversion failures.
                    let text = unsafe { pointer.to_string() };
                    // SAFETY: Free exactly the allocation returned above.
                    unsafe { CoTaskMemFree(Some(pointer.0.cast())) };
                    match text {
                        Ok(text) => roots.push(Root {
                            source,
                            path: PathBuf::from(text),
                        }),
                        Err(_) => issues.push(Issue {
                            source,
                            path: PathBuf::new(),
                            error: Error::new(ErrorKind::Invalid, "known folder encoding"),
                        }),
                    }
                }
                Err(e) => issues.push(Issue {
                    source,
                    path: PathBuf::new(),
                    error: native_error("SHGetKnownFolderPath", e),
                }),
            }
        }
        (roots, issues)
    }
}
impl Drop for Reader {
    fn drop(&mut self) {
        // SAFETY: One successful initialize per !Send owner, same thread.
        unsafe { CoUninitialize() };
    }
}
fn string(buffer: &[u16]) -> Result<String, Error> {
    let end = buffer
        .iter()
        .position(|v| *v == 0)
        .ok_or_else(|| Error::new(ErrorKind::Limit, "COM string length"))?;
    if end == buffer.len() - 1 {
        return Err(Error::new(
            ErrorKind::Limit,
            "possibly truncated COM string",
        ));
    }
    String::from_utf16(&buffer[..end])
        .map_err(|_| Error::new(ErrorKind::Invalid, "COM string encoding"))
}
impl LinkReader for Reader {
    fn read(&mut self, path: &Path) -> Result<Shortcut, Error> {
        use std::os::windows::ffi::OsStrExt;
        let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        if wide[..wide.len() - 1].contains(&0) {
            return Err(Error::new(ErrorKind::Invalid, "shortcut path"));
        }
        // SAFETY: COM apartment is owned by self. Only released ShellLink class
        // is instantiated; persisted file is loaded read-only, never Resolve or Save.
        let link: IShellLinkW = unsafe { CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER) }
            .map_err(|e| native_error("ShellLink", e))?;
        let persist: IPersistFile = link.cast().map_err(|e| native_error("IPersistFile", e))?;
        // SAFETY: Live NUL-terminated path and read-only mode; interfaces drop
        // before apartment owner; COM references never cross threads.
        unsafe { persist.Load(PCWSTR(wide.as_ptr()), STGM_READ) }
            .map_err(|e| native_error("shortcut Load", e))?;
        let data: IShellLinkDataList = link
            .cast()
            .map_err(|e| native_error("IShellLinkDataList", e))?;
        // SAFETY: Valid loaded COM interface, no external pointer arguments.
        let flags = unsafe { data.GetFlags() }.map_err(|e| native_error("shortcut flags", e))?;
        // These semantics cannot be reproduced by plain CreateProcess.
        let excluded = SLDF_RUNAS_USER.0
            | SLDF_HAS_DARWINID.0
            | SLDF_RUN_WITH_SHIMLAYER.0
            | SLDF_RUN_IN_SEPARATE.0;
        if flags & excluded as u32 != 0 {
            return Err(Error::new(ErrorKind::Unsupported, "special shortcut flags"));
        }
        let mut target = vec![0xffff; 32768];
        let mut arguments = vec![0xffff; 32768];
        let mut directory = vec![0xffff; 32768];
        let mut icon = vec![0xffff; 32768];
        let mut index = 0;
        // SAFETY: Sized mutable UTF-16 buffers and valid output index; null find
        // data is allowed. RAWPATH deliberately avoids tracking/repair searches.
        unsafe {
            link.GetPath(&mut target, std::ptr::null_mut(), SLGP_RAWPATH.0 as u32)
                .map_err(|e| native_error("shortcut target", e))?;
            link.GetArguments(&mut arguments)
                .map_err(|e| native_error("shortcut arguments", e))?;
            link.GetWorkingDirectory(&mut directory)
                .map_err(|e| native_error("shortcut working directory", e))?;
            link.GetIconLocation(&mut icon, &mut index)
                .map_err(|e| native_error("shortcut icon metadata", e))?;
        }
        let directory = string(&directory)?;
        let icon = string(&icon)?;
        Ok(Shortcut {
            executable: PathBuf::from(string(&target)?),
            arguments: string(&arguments)?,
            working_directory: if directory.is_empty() {
                None
            } else {
                Some(directory.into())
            },
            icon: if icon.is_empty() {
                None
            } else {
                Some(Icon {
                    path: icon.into(),
                    index,
                })
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cancellation, Limits, discover};
    #[test]
    fn real_com_shortcut_read_and_malformed_rejection() {
        let mut reader = Reader::new().unwrap();
        let temp = crate::tests::Temp::new();
        let exe = temp.0.join("fixture.exe");
        std::fs::write(&exe, b"not executed").unwrap();
        let path = temp.0.join("Fixture.lnk");
        // SAFETY: Own thread apartment and temporary fixture paths; references
        // and UTF-16 inputs outlive synchronous calls. No Resolve/launch occurs.
        unsafe {
            let link: IShellLinkW =
                CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER).unwrap();
            let target: Vec<u16> = exe
                .to_str()
                .unwrap()
                .encode_utf16()
                .chain(Some(0))
                .collect();
            link.SetPath(PCWSTR(target.as_ptr())).unwrap();
            link.SetArguments(windows::core::w!("\"two words\" /literal:&|%"))
                .unwrap();
            let persist: IPersistFile = link.cast().unwrap();
            let filename: Vec<u16> = path
                .to_str()
                .unwrap()
                .encode_utf16()
                .chain(Some(0))
                .collect();
            persist.Save(PCWSTR(filename.as_ptr()), true).unwrap();
        }
        let actual = reader.read(&path).unwrap();
        assert_eq!(actual.arguments, "\"two words\" /literal:&|%");
        assert_eq!(actual.executable, exe);
        std::fs::write(temp.0.join("Malformed.lnk"), b"bad link").unwrap();
        let catalog = discover(
            &[Root {
                source: Source::UserStartMenu,
                path: temp.0.clone(),
            }],
            &mut reader,
            Limits::default(),
            &Cancellation::default(),
            None,
        )
        .unwrap();
        assert_eq!(catalog.entries.len(), 1);
        assert_eq!(catalog.issues.len(), 1);
    }

    #[test]
    #[ignore = "requires this workspace's built SEI_APPS_FIXTURE; launches only that fixture"]
    fn real_shortcut_arguments_launch_and_special_flags_rejection() {
        let mut reader = Reader::new().unwrap();
        let temp = crate::tests::Temp::new();
        let fixture = PathBuf::from(std::env::var_os("SEI_APPS_FIXTURE").expect("fixture path"));
        assert_eq!(fixture.file_name().unwrap(), "launch_fixture.exe");
        let output = temp.0.join("raw received.txt");
        let path = temp.0.join("Fixture.lnk");
        let arguments = format!(
            "\"{}\" \"two words\" \"\" \"&|%PATH%$()\"",
            output.display()
        );
        let wide = |s: &str| s.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        // SAFETY: Operation-owned apartment, local test-built executable and
        // owned fixture files; inputs live through synchronous COM calls.
        unsafe {
            let link: IShellLinkW =
                CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER).unwrap();
            link.SetPath(PCWSTR(wide(fixture.to_str().unwrap()).as_ptr()))
                .unwrap();
            link.SetArguments(PCWSTR(wide(&arguments).as_ptr()))
                .unwrap();
            link.SetWorkingDirectory(PCWSTR(wide(temp.0.to_str().unwrap()).as_ptr()))
                .unwrap();
            let persist: IPersistFile = link.cast().unwrap();
            persist
                .Save(PCWSTR(wide(path.to_str().unwrap()).as_ptr()), true)
                .unwrap();
            let metadata = reader.read(&path).unwrap();
            let entry =
                crate::catalog::normalize(Source::UserStartMenu, path.clone(), metadata).unwrap();
            let mut receipt = crate::launch_async(entry.request, Cancellation::default())
                .unwrap()
                .join()
                .unwrap()
                .unwrap();
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
            loop {
                if let Some(status) = receipt.child.try_wait().unwrap() {
                    assert!(status.success());
                    break;
                }
                if std::time::Instant::now() >= deadline {
                    let _ = receipt.child.kill();
                    let _ = receipt.child.wait();
                    panic!("owned fixture timeout");
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            let expected: Vec<std::ffi::OsString> = ["two words", "", "&|%PATH%$()"]
                .into_iter()
                .map(Into::into)
                .collect();
            assert_eq!(
                std::fs::read_to_string(&output).unwrap(),
                format!("{expected:?}\n{:?}", temp.0)
            );
            let flags: IShellLinkDataList = link.cast().unwrap();
            flags
                .SetFlags(flags.GetFlags().unwrap() | SLDF_RUNAS_USER.0 as u32)
                .unwrap();
            persist
                .Save(PCWSTR(wide(path.to_str().unwrap()).as_ptr()), true)
                .unwrap();
            assert_eq!(reader.read(&path).unwrap_err().kind, ErrorKind::Unsupported);
        }
    }
}
