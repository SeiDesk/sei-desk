use crate::{Cancellation, Error, ErrorKind};
#[cfg(windows)]
use std::process::{Command, Stdio};
use std::{
    ffi::OsString,
    path::{Component, Path, PathBuf},
    process::Child,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arguments {
    Structured(Vec<OsString>),
    /// Only IShellLink metadata should use this; never append query/user text.
    ShortcutRaw(String),
}
impl Arguments {
    pub(crate) fn raw(&self) -> Option<&str> {
        if let Self::ShortcutRaw(s) = self {
            Some(s)
        } else {
            None
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchRequest {
    pub executable: PathBuf,
    pub arguments: Arguments,
    pub working_directory: Option<PathBuf>,
}
impl LaunchRequest {
    pub fn executable(
        executable: PathBuf,
        arguments: Vec<OsString>,
        working_directory: Option<PathBuf>,
    ) -> Result<Self, Error> {
        let value = Self {
            executable,
            arguments: Arguments::Structured(arguments),
            working_directory,
        };
        value.validate_syntax()?;
        Ok(value)
    }
    pub(crate) fn shortcut(
        executable: PathBuf,
        arguments: String,
        working_directory: Option<PathBuf>,
    ) -> Result<Self, Error> {
        let value = Self {
            executable,
            arguments: Arguments::ShortcutRaw(arguments),
            working_directory,
        };
        value.validate_syntax()?;
        Ok(value)
    }
    fn validate_syntax(&self) -> Result<(), Error> {
        local_path(&self.executable)?;
        // Shell/script hosts require a separate reviewed launch contract.
        let name = self
            .executable
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if [
            "cmd.exe",
            "powershell.exe",
            "pwsh.exe",
            "wscript.exe",
            "cscript.exe",
            "mshta.exe",
            "explorer.exe",
        ]
        .contains(&name.as_str())
        {
            return Err(Error::new(ErrorKind::Unsupported, "shell/script host"));
        }
        if !self
            .executable
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"))
        {
            return Err(Error::new(ErrorKind::Unsupported, "executable extension"));
        }
        if let Some(dir) = &self.working_directory {
            local_path(dir)?;
        }
        let valid = match &self.arguments {
            Arguments::Structured(args) => {
                args.len() <= 256
                    && args
                        .iter()
                        .all(|s| s.to_str().is_some_and(|v| !v.contains('\0')))
                    && args
                        .iter()
                        .filter_map(|s| s.to_str())
                        .map(|s| s.encode_utf16().count())
                        .sum::<usize>()
                        < 6000
            }
            Arguments::ShortcutRaw(raw) => {
                !raw.contains('\0') && raw.encode_utf16().count() < 16000
            }
        };
        if !valid {
            return Err(Error::new(ErrorKind::Invalid, "arguments"));
        }
        Ok(())
    }
    pub fn validate(&self) -> Result<(), Error> {
        self.validate_syntax()?;
        regular_file(&self.executable)?;
        if let Some(dir) = &self.working_directory {
            let meta =
                std::fs::symlink_metadata(dir).map_err(|e| Error::io("working directory", e))?;
            if !meta.is_dir() || crate::catalog::reparse(&meta) {
                return Err(Error::new(ErrorKind::Invalid, "working directory"));
            }
        }
        Ok(())
    }
}
pub(crate) fn local_path(path: &Path) -> Result<(), Error> {
    let text = path
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::Invalid, "path encoding"))?;
    if text.encode_utf16().count() > 4096
        || !path.is_absolute()
        || text.contains('\0')
        || path.components().any(|c| matches!(c, Component::ParentDir))
    {
        return Err(Error::new(ErrorKind::Invalid, "absolute path"));
    }
    #[cfg(windows)]
    {
        use std::path::Prefix;
        if !matches!(path.components().next(), Some(Component::Prefix(p)) if matches!(p.kind(), Prefix::Disk(_)))
        {
            return Err(Error::new(ErrorKind::Unsupported, "network/device path"));
        }
        if text[2..].contains(':') {
            return Err(Error::new(ErrorKind::Invalid, "alternate stream"));
        }
    }
    Ok(())
}
fn regular_file(path: &Path) -> Result<(), Error> {
    let meta = std::fs::symlink_metadata(path).map_err(|e| Error::io("target metadata", e))?;
    if !meta.is_file() || crate::catalog::reparse(&meta) {
        return Err(Error::new(ErrorKind::Invalid, "regular target"));
    }
    Ok(())
}

/// Acceptance means CreateProcess succeeded, not application readiness or focus.
/// Caller owns Child; dropping it closes handles but does not kill/wait the app.
#[derive(Debug)]
pub struct LaunchReceipt {
    pub child: Child,
}
pub fn launch(request: LaunchRequest, cancel: &Cancellation) -> Result<LaunchReceipt, Error> {
    cancel.check()?;
    request.validate()?;
    #[cfg(not(windows))]
    {
        return Err(Error::new(ErrorKind::Unsupported, "Windows launch"));
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let mut command = Command::new(&request.executable);
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        // CREATE_NO_WINDOW: no console popup for console targets. No runas,
        // association lookup, cmd /c, PowerShell, or PATH executable search.
        command.creation_flags(0x08000000);
        match &request.arguments {
            Arguments::Structured(args) => {
                command.args(args);
            }
            Arguments::ShortcutRaw(raw) => {
                command.raw_arg(raw);
            }
        }
        let cwd = request
            .working_directory
            .as_deref()
            .or_else(|| request.executable.parent())
            .ok_or_else(|| Error::new(ErrorKind::Invalid, "working directory"))?;
        command.current_dir(cwd);
        cancel.check()?;
        let child = command.spawn().map_err(|e| Error::io("CreateProcess", e))?;
        Ok(LaunchReceipt { child })
    }
}
