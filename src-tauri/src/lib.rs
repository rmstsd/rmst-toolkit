use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn check_update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
  if let Some(update) = app.updater()?.check().await? {
    let mut downloaded = 0;

    // alternatively we could also call update.download() and update.install() separately
    update
      .download_and_install(
        |chunk_length, content_length| {
          downloaded += chunk_length;
          println!("downloaded {downloaded} from {content_length:?}");
        },
        || {
          println!("download finished");
        },
      )
      .await?;

    println!("update installed");
    app.restart();
  } else {
    println!("no update available");
  }

  Ok(())
}

#[tauri::command]
fn get_version(app: AppHandle) -> String {
  let v = app.package_info().version.clone();

  let mut version = v.to_string();

  dbg!(&version);

  version
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![check_update, get_version])
    .setup(|app| Ok(()))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
