use std::{path::Path, process::Command};

#[tauri::command]
pub fn open_with_terminal(projectPath: String) -> Result<(), String> {
  let basename = Path::new(&projectPath).file_name().unwrap().to_string_lossy().to_string();
  Command::new("wt")
    .args(["-d", &projectPath, "--title", &basename, "--suppressApplicationTitle"])
    .spawn()
    .map_err(|e| format!("Failed to open Windows Terminal: {}", e))?;

  Ok(())
}
