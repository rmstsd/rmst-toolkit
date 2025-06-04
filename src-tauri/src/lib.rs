#![allow(warnings)]

mod commands;
mod shortcut;
mod store;
mod tray;
mod utils;
mod window;

mod updater;

use store::initStore;
use tauri::AppHandle;
use tauri::Emitter; // 特质
use tauri::Manager; // 特质
use tauri::WindowEvent;
use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};
use window::WIN_LABEL_OPEN_FOLDER;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_os::init())
    .plugin(
      tauri_plugin_log::Builder::new()
        .timezone_strategy(TimezoneStrategy::UseLocal)
        .targets([
          Target::new(TargetKind::Stdout),
          Target::new(TargetKind::LogDir { file_name: None }),
          Target::new(TargetKind::Webview),
        ])
        .build(),
    )
    .invoke_handler(tauri::generate_handler![
      commands::openWin,
      commands::importSetting,
      commands::exportSetting,
      commands::saveSetting,
      commands::getSetting,
      commands::clearStore,
      commands::getHistoryOpenedUrls,
      commands::clearHistoryOpenedUrls,
      commands::killPort,
      commands::getProjectNamesTree,
      commands::openFolderEditor,
      commands::hideDirWindow,
      commands::setDirWindowSize,
      commands::page_loaded,
      commands::hideWindow,
      commands::CopyAndPaste,
      commands::updateQuickInputWindowSize,
      commands::hideQuickInputWindow,
      commands::get_package_info,
      commands::checkUpdate,
      commands::download_and_install,
      commands::saveCommands,
      commands::getCommands,
      commands::execCommand,
    ])
    .on_window_event(|window, evt| {
      match evt {
        WindowEvent::CloseRequested { api, .. } => match window.label() {
          window::WIN_LABEL_OPEN_FOLDER
          | window::WIN_LABEL_SETTING
          | window::WIN_LABEL_QUICK_INPUT => {
            api.prevent_close();
            window.hide();
          }
          _ => {}
        },
        WindowEvent::Focused(focused) => {
          if window.label() == WIN_LABEL_OPEN_FOLDER {
            if !focused {
              // let closure = || println!("异步任务");
              // let hand = tokio::spawn(async move {
              //   sleep(Duration::from_millis(1000)).await;
              //   closure();
              // });

              // window.hide();
            }
          }

          let app = window.app_handle();
          app
            .emit_to(WIN_LABEL_OPEN_FOLDER, "focusChanged", focused)
            .unwrap();
        }
        _ => {}
      };
    })
    .on_webview_event(|window, event| {
      dbg!(&event);
    })
    .on_page_load(|ww, payload| {
      dbg!(&payload.event());
    })
    .setup(|app| {
      initStore(app);

      window::create_window(app);

      #[cfg(desktop)]
      app.handle().plugin(tauri_plugin_single_instance::init(
        |app: &AppHandle, args, cwd| {},
      ));

      tray::create_tray(app);
      shortcut::create_shortcut(app);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
