use log::info;
use serde::Deserialize;
use serde::Serialize;
use tauri::ipc::Channel;
use tauri::AppHandle;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
  needUpdate: bool,
  current_version: String,
  version: String,
}

static mut updatePlay: Option<Update> = None;

use tauri_plugin_updater::{Update, UpdaterExt};
#[tauri::command]
pub async fn checkUpdate(app: tauri::AppHandle) -> tauri_plugin_updater::Result<UpdateInfo> {
  if let Some(update) = app.app_handle().updater()?.check().await? {
    info!("有更新");
    unsafe {
      updatePlay = Some(update.clone());
    };

    let update_info = UpdateInfo {
      needUpdate: true,
      current_version: update.current_version,
      version: update.version,
    };
    return Ok(update_info);
  }
  info!("没有更新");

  Ok(UpdateInfo {
    needUpdate: false,
    current_version: "".to_string(),
    version: "".to_string(),
  })
}

#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data")]
pub enum DownloadEvent {
  Started { content_length: Option<u64> },
  Progress { chunk_length: usize },
  Finished,
}
#[tauri::command]
pub async fn download_and_install(app: AppHandle, on_event: Channel<DownloadEvent>) {
  unsafe {
    let mut started = false;

    match updatePlay.as_ref() {
      Some(update) => {
        info!("开始下载 update");
        // Use `update` as a reference here
        // .download_and_install
        update
          .download_and_install(
            |chunk_length, content_length| {
              if !started {
                on_event.send(DownloadEvent::Started { content_length });
                started = true;
              }

              on_event.send(DownloadEvent::Progress { chunk_length });
              // downloaded += chunk_length;
              // println!("downloaded {downloaded} from {content_length:?}");
            },
            || {
              println!("download finished");
              info!("rust -> download finished");

              on_event.send(DownloadEvent::Finished);
            },
          )
          .await;

        println!("update installed");
        info!("rust -> update installed");
        // app.restart();
      }
      None => {}
    }
  }
}
