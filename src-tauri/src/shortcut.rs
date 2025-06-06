use tauri::utils::config::Position;
use tauri::{App, AppHandle, LogicalPosition, PhysicalPosition};
use tauri::{Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::window::{WIN_LABEL_OPEN_FOLDER, WIN_LABEL_QUICK_INPUT, WIN_LABEL_SETTING};

pub fn create_shortcut(app: &mut App) {
  let alt_space_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);
  let alt_v_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyV);
  let alt_r_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyR);

  let _ = app.handle().plugin(
    tauri_plugin_global_shortcut::Builder::new()
      .with_handler(move |_app: &AppHandle, shortcut, event| {
        if shortcut == &alt_space_shortcut {
          match event.state() {
            ShortcutState::Pressed => {
              let ww = _app.get_webview_window(WIN_LABEL_OPEN_FOLDER).unwrap();

              let setPos = || {
                let mainMonitor = ww.primary_monitor().unwrap().unwrap();
                let monitorSize = mainMonitor.size();
                let wwWidth = ww.outer_size().unwrap().width;

                let x: u32 = monitorSize.width / 2 - wwWidth / 2;
                let y: u32 = monitorSize.height / 4;
                let pos = ww.outer_position().unwrap();
                ww.set_position(PhysicalPosition { x, y });
              };

              let is_visible = ww.is_visible().unwrap_or_default();
              if is_visible {
                if ww.is_focused().expect("is_focused msg") {
                  ww.hide().unwrap();
                } else {
                  setPos();
                  ww.set_focus().unwrap();
                }
              } else {
                setPos();
                ww.show().unwrap();
                ww.set_focus().unwrap();
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
              let ww = _app.get_webview_window(WIN_LABEL_QUICK_INPUT).unwrap();

              if ww.is_visible().unwrap_or(false) {
                ww.hide().unwrap();
              } else {
                let pos = ww.cursor_position().unwrap();
                ww.set_position(pos).unwrap();
                ww.show().unwrap();
                ww.set_focus().unwrap();
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
              let ww = _app.get_webview_window(WIN_LABEL_SETTING).unwrap();
              ww.unminimize().unwrap();
              ww.show().unwrap();
              ww.set_focus().unwrap();

              let clipboard_text = _app.clipboard().read_text().unwrap_or_default();
              let text = clipboard_text.trim().to_string();

              _app.emit_to(WIN_LABEL_SETTING, "showQrCode", text).unwrap();
            }
            ShortcutState::Released => {
              // println!("Ctrl-N Released!");
            }
          }
        }
      })
      .build(),
  );

  let _ = app.global_shortcut().register(alt_space_shortcut);
  let _ = app.global_shortcut().register(alt_v_shortcut);
  let _ = app.global_shortcut().register(alt_r_shortcut);
}
