#![allow(non_upper_case_globals, non_snake_case)]

//

mod cmds;
mod commands;
mod shortcut;
mod store;
mod tray;
mod utils;
mod window;

use tauri::is_dev;
use tauri::AppHandle;
use tauri::Emitter; // 特质
use tauri::Manager; // 特质
use tauri::WindowEvent;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};

use crate::{cmds::command, cmds::common, cmds::open_win, cmds::setting, cmds::updater};
use store::initStore;
use window::WIN_LABEL_OPEN_FOLDER;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_autostart::init(
      MacosLauncher::LaunchAgent,
      Some(vec!["--flag1", "--flag2"]), /* arbitrary number of args to pass to your app */
    ))
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
      open_win::openWin,
      open_win::page_loaded,
      open_win::getHistoryOpenedUrls,
      open_win::clearHistoryOpenedUrls,
      //
      common::hideWindow,
      common::setWindowSize,
      common::get_package_info,
      //
      setting::importSetting,
      setting::exportSetting,
      setting::saveSetting,
      setting::getSetting,
      setting::clearStore,
      //
      command::saveCommands,
      command::getCommands,
      command::execCommand,
      command::removeFolder,
      //
      updater::checkUpdate,
      updater::download_and_install,
      //
      commands::getProjectNamesTree,
      commands::openFolderEditor,
      //
      commands::killPort,
      //
      commands::CopyAndPaste,
      //
      commands::trayMenu,
    ])
    .on_window_event(|window, evt| {
      match evt {
        WindowEvent::CloseRequested { api, .. } => match window.label() {
          window::WIN_LABEL_OPEN_FOLDER
          | window::WIN_LABEL_SETTING
          | window::WIN_LABEL_QUICK_INPUT
          | window::WIN_LABEL_Tray_Menu => {
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
          app.emit_to(WIN_LABEL_OPEN_FOLDER, "focusChanged", focused).unwrap();
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

      let idDev = is_dev();
      if !idDev {
        let autostart_manager = app.autolaunch();
        let _ = autostart_manager.enable();
        // 检查 enable 状态
        // println!(
        //   "registered for autostart? {}",
        //   autostart_manager.is_enabled().unwrap()
        // );
        // // 禁用 autostart
        // let _ = autostart_manager.disable();
      }

      #[cfg(desktop)]
      app
        .handle()
        .plugin(tauri_plugin_single_instance::init(|app: &AppHandle, args, cwd| {}));

      tray::create_tray(app);
      shortcut::create_shortcut(app);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
