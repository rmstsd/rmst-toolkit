use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::App;
use tauri::Manager;

use crate::window::WIN_LABEL_SETTING;

pub fn create_tray(app: &mut App) {
  let m2 = MenuItem::with_id(app, "setting", "设置", true, None::<&str>).unwrap();
  let restart = MenuItem::with_id(app, "restart", "重启", true, None::<&str>).unwrap();
  let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>).unwrap();
  let separator = &PredefinedMenuItem::separator(app).unwrap();
  let menu = Menu::with_items(app, &[&m2, separator, &restart, &quit_i]).unwrap();

  let _ = TrayIconBuilder::new()
    .menu(&menu)
    .show_menu_on_left_click(false)
    .icon(app.default_window_icon().unwrap().clone())
    .on_menu_event(|app, event| {
      match event.id.as_ref() {
        "setting" => {
          let setting_window = app.get_webview_window(WIN_LABEL_SETTING);

          if let Some(ww) = setting_window {
            if ww.is_minimized().unwrap_or(false) {
              let _ = ww.unminimize();
            }

            let _ = ww.show();
            let _ = ww.set_focus();
          }
        }
        "quit" => {
          app.exit(0);
        }
        "restart" => {
          app.restart();
        }
        _ => {
          println!("未匹配 {:?}", event.id)
        }
      };
    })
    .on_tray_icon_event(|tray, evt| {
      match evt {
        TrayIconEvent::Click {
          position,
          button: MouseButton::Right,
          button_state: MouseButtonState::Up,
          ..
        } => {
          dbg!(&position);
        }
        _ => {}
      };
    })
    .build(app);
}
