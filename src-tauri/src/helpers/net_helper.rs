use std::process::{Command, Stdio};

#[tauri::command]
pub fn check_connection() -> bool {
    let mut command = Command::new("ping");

    #[cfg(target_os = "windows")]
    #[allow(unused_imports)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }

    command.arg("8.8.8.8").stdout(Stdio::null()).stderr(Stdio::null());

    let child = command.spawn().expect("Failed to spawn ping");

    let output = child.wait_with_output().expect("ping");

    if output.status.success() {
        return true
    }

    false
}