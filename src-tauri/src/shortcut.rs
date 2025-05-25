use tauri::{App, AppHandle};
use tauri::{Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

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
              let open_folder_window = _app.get_webview_window("openFolder").unwrap();

              let is_visible = open_folder_window.is_visible().unwrap_or_default();
              if is_visible {
                if open_folder_window.is_focused().expect("is_focused msg") {
                  open_folder_window.hide().unwrap();
                } else {
                  open_folder_window.set_focus().unwrap();
                }
              } else {
                open_folder_window.show().unwrap();
                open_folder_window.set_focus().unwrap();
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
              let ww = _app.get_webview_window("setting").unwrap();
              ww.unminimize().unwrap();
              ww.show().unwrap();
              ww.set_focus().unwrap();

              let clipboard_text = _app.clipboard().read_text().unwrap_or_default();
              let text = clipboard_text.trim().to_string();

              _app.emit_to("setting", "showQrCode", text).unwrap();
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
