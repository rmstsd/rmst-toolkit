use log::info;
use tauri::AppHandle;

use tauri::Emitter; // 特质
use tauri::Manager; // 特质

use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};
use tauri_plugin_updater::UpdaterExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn check_update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
  if let Some(update) = app.updater()?.check().await? {
    let mut downloaded = 0;

    info!("开始下载");
    // alternatively we could also call update.download() and update.install() separately
    update
      .download_and_install(
        |chunk_length, content_length| {
          downloaded += chunk_length;
          // info!("downloaded {downloaded} from {content_length:?}");
        },
        || {
          info!("下载完成");
        },
      )
      .await?;

    info!("update installed");
    app.restart();
  } else {
    info!("no update available");
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
    .plugin(tauri_plugin_clipboard_manager::init())
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
    .invoke_handler(tauri::generate_handler![check_update, get_version])
    .setup(|app| {
      #[cfg(desktop)]
      app.handle().plugin(tauri_plugin_single_instance::init(
        |app: &AppHandle, args, cwd| {},
      ));

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

      {
        use tauri_plugin_global_shortcut::{
          Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
        };

        let alt_space_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);
        let alt_v_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyV);
        let alt_r_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyR);

        app.handle().plugin(
          tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app: &AppHandle, shortcut, event| {
              if shortcut == &alt_space_shortcut {
                match event.state() {
                  ShortcutState::Pressed => {
                    let openFolderWindows = _app.get_webview_window("openFolder").unwrap();

                    let isVisible = openFolderWindows.is_visible().unwrap_or_default();
                    if isVisible {
                      if openFolderWindows.is_focused().expect("is_focused msg") {
                        openFolderWindows.hide();
                      } else {
                        openFolderWindows.set_focus();
                      }
                    } else {
                      openFolderWindows.show();
                      openFolderWindows.set_focus();
                    }
                  }
                  ShortcutState::Released => {
                    // println!("Ctrl-N Released!");
                  }
                }
              }
              if shortcut == &alt_v_shortcut {
                match event.state() {
                  ShortcutState::Pressed => {
                    let ww = _app.get_webview_window("quickInput").unwrap();

                    if ww.is_visible().unwrap_or(false) {
                      ww.hide();
                    } else {
                      let pos = ww.cursor_position().unwrap();
                      ww.set_position(pos);
                      ww.show();
                      ww.set_focus();
                    }
                  }
                  ShortcutState::Released => {
                    // println!("Ctrl-N Released!");
                  }
                }
              }
              if shortcut == &alt_r_shortcut {
                dbg!(&"alt + r");
                match event.state() {
                  ShortcutState::Pressed => {
                    let ww = _app.get_webview_window("setting").unwrap();
                    ww.unminimize();
                    ww.show();
                    ww.set_focus();

                    let clipboard_text = _app.clipboard().read_text().unwrap_or_default();
                    let text = clipboard_text.trim().to_string();

                    _app.emit_to("setting", "showQrCode", text);
                  }
                  ShortcutState::Released => {
                    // println!("Ctrl-N Released!");
                  }
                }
              }
            })
            .build(),
        )?;

        app.global_shortcut().register(alt_space_shortcut);
        app.global_shortcut().register(alt_v_shortcut);
        app.global_shortcut().register(alt_r_shortcut);
      }

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
