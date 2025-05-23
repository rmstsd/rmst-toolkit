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
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .invoke_handler(tauri::generate_handler![check_update, get_version])
    .setup(|app| {
      #[cfg(desktop)]
      app
        .handle()
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}));

      {
        //
        use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
        use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
        let m2 = MenuItem::with_id(app, "setting", "设置", true, None::<&str>)?;
        let restart = MenuItem::with_id(app, "restart", "重启", true, None::<&str>)?;
        let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
        let separator = &PredefinedMenuItem::separator(app).unwrap();
        let menu = Menu::with_items(app, &[&m2, separator, &restart, &quit_i])?;
        let tray = TrayIconBuilder::new()
          .menu(&menu)
          .show_menu_on_left_click(false)
          .icon(app.default_window_icon().unwrap().clone())
          .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
              app.exit(0);
            }
            "restart" => {
              app.restart();
            }
            _ => {
              println!("未匹配 {:?}", event.id)
            }
          })
          .on_tray_icon_event(|tray, evt| match evt {
            TrayIconEvent::Click {
              position,
              rect,
              button: MouseButton::Right,
              button_state: MouseButtonState::Up,
              ..
            } => {
              dbg!(&position);
            }
            _ => {}
          })
          .build(app)?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
