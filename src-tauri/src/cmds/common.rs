use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri::{LogicalSize, WebviewWindow};

#[tauri::command]
pub async fn hideWindow(window: tauri::Window) -> Result<(), tauri::Error> {
  window.hide()?;
  Ok(())
}

#[tauri::command]
pub fn setWindowSize(app: tauri::AppHandle, label: &str, width: u32, height: u32) -> Result<(), Option> {
  dbg!(label, width, height);

  let ww = app.get_webview_window(label)?;
  ww.set_size(LogicalSize { width, height }).unwrap();

  Ok(())
}

#[tauri::command]
pub async fn get_package_info(app: tauri::AppHandle) -> Result<AppInfo, String> {
  let pi = app.package_info();

  dbg!(&pi);

  let vvv = &pi.version;
  let vec: Vec<String> = vec![&vvv.major, &vvv.minor, &vvv.patch]
    .into_iter()
    .map(|item| item.to_string())
    .collect();
  let version = vec.join(".");

  let info = AppInfo {
    name: pi.name.clone(),
    version: version,
    authors: pi.authors,
    description: pi.description,
    crate_name: pi.crate_name,
  };

  Ok(info)
}

#[derive(Serialize, Deserialize)]
pub struct AppInfo {
  name: String,
  version: String,
  authors: &'static str,
  description: &'static str,
  crate_name: &'static str,
}
