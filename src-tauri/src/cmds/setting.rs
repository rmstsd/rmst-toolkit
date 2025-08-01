use serde_json::json;
use serde_json::to_value;
use serde_json::to_writer_pretty;
use serde_json::Value;
use std::fs;
use std::fs::File;
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

use crate::{commands::SettingData, store::AppData};

static Setting_Key: &str = "setting";

#[tauri::command]
pub fn importSetting(app: AppHandle) -> Result<(), ()> {
  let file_path = app.dialog().file().add_filter("", &["json"]).blocking_pick_file();

  if let Some(path) = file_path {
    let path: String = path.to_string();

    println!("{path:#?}");

    let opened = fs::File::open(path);

    if let Ok(val) = opened {
      // json 里的 数据和 结构体不一致会导致 error
      let data: Result<SettingData, serde_json::Error> = serde_json::from_reader(val);

      if let Ok(data) = data {
        dbg!(&data);
        saveSetting(app, data);

        return Ok(());
      }
    }
  }

  return Err(());
}

#[tauri::command]
pub fn exportSetting(app: AppHandle) -> Result<(), String> {
  dbg!("exportSetting");
  let file_path = app
    .dialog()
    .file()
    .set_file_name("cfg-2.json")
    .add_filter("", &["json"])
    .blocking_save_file();

  if let Some(file_path) = file_path {
    println!("{file_path:#?}");
    dbg!(file_path.to_string());

    let appData = app.state::<AppData>();
    let store = &appData.store;
    let ans = store.get(Setting_Key).unwrap_or(json!({}));

    // let st = to_string(&ans.unwrap_or(json!({})));
    let writer = File::create(file_path.to_string()).unwrap();
    to_writer_pretty(writer, &ans);

    return Ok(());
  }

  return Err("".to_string());
}

#[tauri::command]
pub fn saveSetting(app: AppHandle, settingData: SettingData) {
  let appData = app.state::<AppData>();
  let store = &appData.store;

  dbg!("saveSetting", &settingData);

  let val = to_value(settingData);

  match val {
    Ok(value) => {
      dbg!(&value);

      store.set(Setting_Key, value);
    }
    Err(err) => {
      dbg!(&err);
    }
  }
}

#[tauri::command]
pub fn getSetting(app: AppHandle) -> Value {
  let appData = app.state::<AppData>();
  let val = appData.store.get(Setting_Key);

  match val {
    Some(val) => val,
    None => Value::Null,
  }
}

#[tauri::command]
pub async fn clearStore(app: AppHandle) -> Result<(), String> {
  let appData = app.state::<AppData>();

  let store = &appData.store;
  store.delete(Setting_Key);

  Ok(())
}
