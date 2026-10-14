//! Developer-only, explicit PID/HWND probe for the Stage 2A window. Never linked
//! into sei-desk. No arbitrary keys/text, no hooks, no activation and no polling.
#[cfg(windows)]
fn probe() -> Result<(), String> {
    use windows_sys::Win32::{
        Foundation::*,
        Graphics::Gdi::ClientToScreen,
        UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
    };
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        return Err(
            "usage: stage2a_probe PID HWND bounds|tab|back|enter|escape|space|sample|caret".into(),
        );
    }
    let process: u32 = args[0].parse().map_err(|_| "Invalid PID")?;
    let target = args[1].parse::<usize>().map_err(|_| "Invalid HWND")? as HWND;
    // SAFETY: Opaque HWND is queried, never dereferenced. Check target identity/title
    // and exact foreground before any input; do not activate or target another app.
    unsafe {
        let mut owner = 0;
        GetWindowThreadProcessId(target, &mut owner);
        let mut title = [0u16; 128];
        let length = GetWindowTextW(target, title.as_mut_ptr(), title.len() as i32);
        if owner != process
            || length <= 0
            || String::from_utf16_lossy(&title[..length as usize])
                != "Sei-Desk — Stage 2A validation"
        {
            return Err("Target is not the identified Stage 2A window".into());
        }
        if GetForegroundWindow() != target {
            return Err("Target does not own native foreground; no input/capture permitted".into());
        }
        if args[2] == "bounds" {
            let mut area = RECT::default();
            let mut origin = POINT::default();
            if GetClientRect(target, &mut area) == 0 || ClientToScreen(target, &mut origin) == 0 {
                return Err("Client bounds query failed".into());
            }
            println!(
                "{{\"X\":{},\"Y\":{},\"Width\":{},\"Height\":{}}}",
                origin.x,
                origin.y,
                area.right - area.left,
                area.bottom - area.top
            );
            return Ok(());
        }
    }
    fn vk(code: u16, up: bool) -> INPUT {
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: code,
                    wScan: 0,
                    dwFlags: if up { KEYEVENTF_KEYUP } else { 0 },
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        }
    }
    fn pair(code: u16) -> Vec<INPUT> {
        vec![vk(code, false), vk(code, true)]
    }
    fn unicode(value: &str) -> Vec<INPUT> {
        value
            .encode_utf16()
            .flat_map(|scan| {
                [false, true].map(|up| INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: 0,
                            wScan: scan,
                            dwFlags: KEYEVENTF_UNICODE | if up { KEYEVENTF_KEYUP } else { 0 },
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                })
            })
            .collect()
    }
    let input = match args[2].as_str() {
        "tab" => pair(VK_TAB),
        "enter" => pair(VK_RETURN),
        "escape" => pair(VK_ESCAPE),
        "space" => pair(VK_SPACE),
        "back" => vec![
            vk(VK_SHIFT, false),
            vk(VK_TAB, false),
            vk(VK_TAB, true),
            vk(VK_SHIFT, true),
        ],
        "sample" => unicode("sample"),
        "caret" => {
            let mut keys = pair(VK_LEFT);
            keys.extend(pair(VK_LEFT));
            keys.extend(unicode("x"));
            keys
        }
        _ => return Err("Unsupported probe command".into()),
    };
    // SAFETY: Fixed key commands only, paired down/up events in an initialized INPUT
    // buffer. Recheck exact foreground immediately before injection; this remains
    // non-atomic with user focus changes, so the test procedure requires no interaction.
    let sent = unsafe {
        if GetForegroundWindow() != target {
            return Err("Foreground changed; no keys sent".into());
        }
        SendInput(
            input.len() as u32,
            input.as_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        )
    };
    println!("send_input_count={sent} expected={}", input.len());
    if sent != input.len() as u32 {
        if args[2] == "back" {
            // SAFETY: Best-effort Shift release after partial injection; do not leave
            // a synthetic modifier held down. No character or shell shortcut is sent.
            unsafe {
                SendInput(1, &vk(VK_SHIFT, true), std::mem::size_of::<INPUT>() as i32);
            }
        }
        return Err("SendInput incomplete (UIPI/session cause is not established)".into());
    }
    Ok(())
}

fn main() -> std::process::ExitCode {
    #[cfg(windows)]
    match probe() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("probe_blocked: {error}");
            std::process::ExitCode::FAILURE
        }
    }
    #[cfg(not(windows))]
    {
        eprintln!("Windows required");
        std::process::ExitCode::FAILURE
    }
}
