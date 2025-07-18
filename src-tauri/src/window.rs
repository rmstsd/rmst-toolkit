use tauri::{window::Color, App, AppHandle, WebviewUrl, WebviewWindowBuilder};

pub const WIN_LABEL_SETTING: &str = "setting";
pub const WIN_LABEL_OPEN_FOLDER: &str = "openFolder";
pub const WIN_LABEL_QUICK_INPUT: &str = "quickInput";
pub const WIN_LABEL_Tray_Menu: &str = "trayMenu";

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
  .expect("#setting webview_window create 错误");

  {
    let ww = WebviewWindowBuilder::new(
      app,
      WIN_LABEL_OPEN_FOLDER,
      WebviewUrl::App("index.html/#openFolder".into()),
    )
    .visible(false)
    .resizable(false)
    .always_on_top(true)
    .decorations(false)
    .skip_taskbar(true)
    .center()
    .build()
    .expect("#openFolder webview_window create error 错误");
  }

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
  .resizable(false)
  .build()
  .expect("#quickInput webview_window create error 错误");

  let mut ww = WebviewWindowBuilder::new(
    app,
    WIN_LABEL_Tray_Menu,
    WebviewUrl::App("index.html/#trayMenu".into()),
  )
  .skip_taskbar(true)
  .visible(false)
  .inner_size(400.0, 200.0)
  .maximizable(false)
  .minimizable(false)
  .always_on_top(true)
  .decorations(false)
  .resizable(false);

  #[cfg(target_os = "windows")]
  {
    ww = ww.transparent(true);
  }

  ww.shadow(false)
    .build()
    .expect("#trayMenu webview_window create error 错误");

  let machine_kind = if cfg!(unix) {
    "unix"
  } else if cfg!(windows) {
    "windows"
  } else {
    "unknown"
  };

  println!("I'm running on a {} machine!", machine_kind);

  if cfg!(target_os = "macos") {
    println!("macos");
  } else {
    println!("other os");
  }
}
