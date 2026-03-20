use serde::{Deserialize, Serialize};
use tauri::LogicalSize;
use tauri::WebviewWindow;
use tokio::process::Command;

#[tauri::command]
pub async fn hideWindow(window: tauri::Window) -> Result<(), tauri::Error> {
  window.hide()?;
  Ok(())
}

#[tauri::command]
pub fn setWindowSize(webview: WebviewWindow, width: Option<f64>, height: Option<f64>) -> Result<(), tauri::Error> {
  let ww = webview;
  let scale_factor = ww.scale_factor()?;
  let size: LogicalSize<f64> = ww.inner_size()?.to_logical(scale_factor);

  let width = width.unwrap_or(size.width);
  let height = height.unwrap_or(size.height);

  ww.set_size(LogicalSize { width, height })?;

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

#[tauri::command]
pub fn open_in_explorer(path: String) {
  let path = path.replace("/", r"\");
  Command::new("explorer")
    // .args(["/select,", path.as_str()]) // 不进入 且 选中
    .args([path.as_str()]) // 进入
    .spawn()
    .unwrap();
}
