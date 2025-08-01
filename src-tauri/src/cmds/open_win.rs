use log::info;
use rand::random;
use reqwest::Client;
use serde_json::json;

use serde_json::from_value;
use serde_json::Value;
use tauri::image::Image;
use tauri::AppHandle;

use tauri::Manager;
use tauri::WebviewWindow;

use crate::{store::AppData, utils::readFile};

static HistoryOpenedUrls_Key: &str = "historyOpenedUrls";

#[tauri::command(async)]
pub fn openWin(app: AppHandle, url: String) -> Result<(), tauri::Error> {
  let label: i32 = random();
  let label = label.to_string();

  let ww: WebviewWindow = tauri::WebviewWindowBuilder::new(&app, label, tauri::WebviewUrl::App(url.clone().into()))
    .title("rmst-tools")
    .inner_size(1200.0, 800.0)
    .build()
    .expect("webview_window create error 啊");

  let data = readFile("resources/ww-script.js").unwrap_or_default();
  if !data.is_empty() {
    ww.eval(data.as_str())?;
  }

  let appData = app.state::<AppData>();
  let store = &appData.store;
  let listVal = store.get(HistoryOpenedUrls_Key).unwrap_or(json!([]));

  let mut list = from_value::<Vec<String>>(listVal).unwrap();

  if !list.contains(&url) {
    list.insert(0, url);

    if list.len() > 5 {
      list.pop();
    }

    store.set(HistoryOpenedUrls_Key, list);
  };

  Ok(())
}

#[tauri::command]
pub fn getHistoryOpenedUrls(app: AppHandle) -> Result<Value, ()> {
  let appData = app.state::<AppData>();
  let store = &appData.store;

  let val = store.get(HistoryOpenedUrls_Key);

  match val {
    Some(val) => Ok(val),
    None => {
      let emp = serde_json::from_value(json!([])).unwrap();

      Ok(emp)
    }
  }
}

#[tauri::command]
pub async fn clearHistoryOpenedUrls(app: AppHandle) -> Result<(), tauri::Error> {
  let appData = app.state::<AppData>();
  let store = &appData.store;
  store.delete(HistoryOpenedUrls_Key);

  Ok(())
}

#[tauri::command]
pub async fn page_loaded(window: tauri::Window, title: String, icon: String) -> Result<(), tauri::Error> {
  info!("Page loaded with title: {}, {}", title, icon);

  window.set_title(title.as_str())?;

  match download_icon(icon.as_str()).await {
    Ok(image) => {
      if let Err(e) = window.set_icon(image) {
        eprintln!("Failed to set window icon: {}", e);
      }
    }
    Err(e) => eprintln!("Failed to download icon: {}", e),
  }

  Ok(())
}

async fn download_icon(url: &str) -> Result<Image, Box<dyn std::error::Error>> {
  // 使用Tauri的HTTP客户端下载图片
  let response = Client::new().get(url).send().await?;

  // 获取图片字节数据
  let bytes = response.bytes().await?.to_vec();

  // 从字节创建Image对象
  let image = Image::from_bytes(&bytes)?;

  Ok(image)
}
