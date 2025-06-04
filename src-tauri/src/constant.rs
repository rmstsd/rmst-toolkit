use tauri::{App, AppHandle, WebviewUrl, WebviewWindowBuilder};

pub const WIN_LABEL_SETTING: &str = "setting";
pub const WIN_LABEL_OPEN_FOLDER: &str = "openFolder";
pub const WIN_LABEL_QUICK_INPUT: &str = "quickInput";

pub fn create_window(app: &mut App) {
  let app = app.handle();

  WebviewWindowBuilder::new(
    app,
    WIN_LABEL_SETTING,
    WebviewUrl::App("index.html/#setting".into()),
  )
  .title("设置")
  .inner_size(1200.0, 800.0)
  .visible(false)
  .center()
  .build()
  .expect("webview_window create error 啊");

  WebviewWindowBuilder::new(
    app,
    WIN_LABEL_OPEN_FOLDER,
    WebviewUrl::App("index.html/#openFolder".into()),
  )
  .visible(false)
  .resizable(false)
  .decorations(false)
  .skip_taskbar(true)
  .center()
  .build()
  .expect("webview_window create error 啊");

  WebviewWindowBuilder::new(
    app,
    WIN_LABEL_QUICK_INPUT,
    WebviewUrl::App("index.html/#quickInput".into()),
  )
  .skip_taskbar(false)
  .visible(false)
  .center()
  .inner_size(400.0, 200.0)
  .maximizable(false)
  .minimizable(false)
  .always_on_top(true)
  .decorations(false)
  .build()
  .expect("webview_window create error 啊");
}
