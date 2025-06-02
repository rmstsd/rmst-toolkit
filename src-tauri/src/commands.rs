use enigo::{
  Button, Coordinate,
  Direction::{Click, Press, Release},
  Enigo, Key, Keyboard, Mouse, Settings,
};
use log::info;
// use port_killer::kill;
use rand::random;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_reader;
use serde_json::from_value;
use serde_json::json;
use serde_json::to_string;
use serde_json::to_value;
use serde_json::Value;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::vec;
use std::{fs, path::PathBuf};
use std::{fs::metadata, sync::Arc};
use std::{fs::read_dir, sync::Mutex};
use tauri::image::Image;
use tauri::webview::PageLoadEvent;
use tauri::AppHandle;
use tauri::Listener;
use tauri::LogicalSize;
use tauri::Manager;
use tauri::Size;
use tauri::WebviewWindow;
use tauri::Wry;
use tauri::{image, EventLoopMessage};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::{Store, StoreExt};
// use urlencoding::encode;

static Setting_Key: &str = "setting";
static HistoryOpenedUrls_Key: &str = "historyOpenedUrls";

static Commands_Key: &str = "commands";

#[tauri::command]
pub fn importSetting(app: AppHandle) {
  dbg!(&7785);
  let file_path = app.dialog().file().blocking_pick_file();

  match file_path {
    Some(path) => {
      let path: String = path.to_string();

      println!("{path:#?}");

      // let content = fs::read_to_string(path).expect("Unable to read file");
      // let data: SettingData = serde_json::from_str(content.as_str()).unwrap();
      // dbg!(&data);

      let opened = fs::File::open(path);

      match opened {
        Ok(val) => {
          // json 里的 数据和 结构体不一致会导致 error
          let data: Result<SettingData, serde_json::Error> = serde_json::from_reader(val);

          match data {
            Ok(val) => {
              dbg!(&val);
              saveSetting(app, val);
            }
            Err(err) => {
              dbg!(&err);
            }
          }
        }
        Err(err) => {
          dbg!(&err);
        }
      }

      // let content: SettingData = serde_json::from_reader(fs::File::open(path).unwrap()).unwrap();
      // saveSetting(app, content);
    }
    None => {}
  }
}

#[tauri::command]
pub fn exportSetting(app: AppHandle) {
  dbg!("exportSetting");
  let file_path = app
    .dialog()
    .file()
    .set_file_name("cfg-2.json")
    .add_filter("My Filter", &["json"])
    .blocking_save_file()
    .unwrap();

  println!("{file_path:#?}");
  dbg!(file_path.to_string());

  let appData = app.state::<AppData>();
  let store = &appData.store;
  let ans = store.get(Setting_Key);

  dbg!(&ans);

  let st = to_string(&ans.unwrap_or(json!({})));

  fs::write(file_path.to_string(), st.unwrap_or_default()).expect("Unable to read file");
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

fn readFile(path: &str) -> io::Result<String> {
  let mut f = File::open(path)?;
  let mut buffer: String = String::new();

  f.read_to_string(&mut buffer)?;

  Ok(buffer)
}

#[tauri::command]
pub async fn page_loaded(
  app: tauri::AppHandle,
  window: tauri::Window,
  title: String,
  icon: String,
) -> Result<(), String> {
  info!("Page loaded with title: {}, {}", title, icon);
  window.set_title(title.as_str());

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

#[tauri::command(async)]
pub fn openWin(app: AppHandle, url: String) {
  dbg!(&url);

  let label: i32 = random();
  let label = label.to_string();

  let ww: WebviewWindow =
    tauri::WebviewWindowBuilder::new(&app, label, tauri::WebviewUrl::App(url.clone().into()))
      .title("rmst-tools")
      .inner_size(1200.0, 800.0)
      .build()
      .expect("webview_window create error 啊");

  let data = readFile("resources/ww-script.js").unwrap_or_default();
  if !data.is_empty() {
    ww.eval(data.as_str());
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
  }
}

#[tauri::command]
pub fn getHistoryOpenedUrls(app: AppHandle) -> Value {
  let appData = app.state::<AppData>();
  let store = &appData.store;
  let val = store.get(HistoryOpenedUrls_Key);

  match val {
    Some(val) => val,
    None => {
      let emp = serde_json::from_value(json!([])).unwrap();

      emp
    }
  }
}

#[tauri::command]
pub async fn clearHistoryOpenedUrls(app: AppHandle) -> Result<(), String> {
  let appData = app.state::<AppData>();
  let store = &appData.store;
  store.delete(HistoryOpenedUrls_Key);
  Ok(())
}

#[tauri::command]
pub fn killPort(port: u16) -> Result<bool, bool> {
  dbg!(&port);
  let r = kill_process_by_port(port);
  match r {
    Ok(()) => Ok(true),
    Err(err) => {
      dbg!(&err);

      Err(false)
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingData {
  cmdPath: Option<String>,
  editorPaths: Option<Vec<String>>,
  projectPaths: Option<Vec<String>>,
  notes: Option<Vec<String>>,
}

use std::process::Command;
fn kill_process_by_port(port: u16) -> Result<(), std::io::Error> {
  #[cfg(target_os = "windows")]
  {
    // 在 Windows 上查找占用指定端口的进程 ID
    let output = Command::new("cmd")
      .args(&["/C", &format!("netstat -ano | findstr :{}", port)])
      .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
      let parts: Vec<&str> = line.split_whitespace().collect();
      if parts.len() >= 5 {
        let pid_str = parts[4];
        if let Ok(pid) = pid_str.parse::<u32>() {
          // 杀死找到的进程
          Command::new("taskkill")
            .args(&["/F", "/PID", &pid.to_string()])
            .output()?;
        }
      }
    }
  }
  #[cfg(target_os = "linux")]
  {
    // 在 Linux 上查找占用指定端口的进程 ID
    let output = Command::new("sh")
      .arg("-c")
      .arg(&format!("lsof -t -i:{}", port))
      .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
      if let Ok(pid) = line.parse::<u32>() {
        // 杀死找到的进程
        Command::new("kill")
          .args(&["-9", &pid.to_string()])
          .output()?;
      }
    }
  }
  #[cfg(target_os = "macos")]
  {
    // 在 macOS 上查找占用指定端口的进程 ID
    let output = Command::new("sh")
      .arg("-c")
      .arg(&format!("lsof -t -i:{}", port))
      .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
      if let Ok(pid) = line.parse::<u32>() {
        // 杀死找到的进程
        Command::new("kill")
          .args(&["-9", &pid.to_string()])
          .output()?;
      }
    }
  }
  Ok(())
}

#[tauri::command]
pub fn getProjectNamesTree(app: AppHandle) -> Value {
  let blackList = vec!["$RECYCLE.BIN", "System Volume Information"];
  let blackStartWithChar = vec!["_", "$", ".", "-"];

  let val: Value = getSetting(app);

  let settingData: SettingData = from_value(val).unwrap_or(SettingData {
    cmdPath: Some("".to_string()),
    editorPaths: Some(vec![]),
    projectPaths: Some(vec![]),
    notes: Some(vec![]),
  });

  let projectPaths: Vec<String> = settingData.projectPaths.unwrap_or(vec![]);

  let nv: Vec<NamesTree> = projectPaths
    .into_iter()
    .filter(|item| {
      if let Ok(md) = metadata(Path::new(item)) {
        md.is_dir()
      } else {
        false
      }
    })
    .map(|item: String| {
      let name = item.replace(r"\", "/");
      let path = Path::new(item.as_str());

      let children = read_dir(path)
        .expect("msg")
        .into_iter()
        .map(|item| item.unwrap().file_name().to_string_lossy().to_string())
        .filter(|item| {
          !blackList.contains(&item.as_str())
            && !blackStartWithChar.iter().any(|char| item.starts_with(char))
        })
        .collect();

      let nt = NamesTree { name, children };

      return nt;
    })
    .collect();

  to_value(nv).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
struct NamesTree {
  name: String,
  children: Vec<String>,
}

#[tauri::command]
pub async fn openFolderEditor(
  app: tauri::AppHandle,
  projectPath: String,
  editorPath: String,
) -> Result<(), String> {
  dbg!(&projectPath);
  dbg!(&editorPath);

  // 执行外部命令示例
  let output = Command::new(editorPath)
    .arg(projectPath)
    .output()
    .expect("Failed to execute command");

  if output.status.success() {
    println!("打开成功");
  } else {
    println!("打开失败");
  }
  Ok(())
}

#[tauri::command]
pub async fn hideDirWindow(app: tauri::AppHandle, window: tauri::Window) -> Result<(), String> {
  let win = app
    .get_webview_window("openFolder")
    .expect("经济技术电饭锅");
  win.hide();
  Ok(())
}

#[tauri::command]
pub async fn setDirWindowSize(app: tauri::AppHandle, height: f64) -> Result<(), String> {
  let win = app
    .get_webview_window("openFolder")
    .expect("对方过后发过火");

  win.set_size(Size::Logical(LogicalSize {
    width: 800.0,
    height,
  }));
  Ok(())
}

#[tauri::command]
pub async fn hideWindow(window: tauri::Window) -> Result<(), String> {
  window.hide();
  Ok(())
}

#[tauri::command]
pub async fn CopyAndPaste(app: AppHandle, content: &str) -> Result<(), String> {
  app.clipboard().write_text(content).unwrap();

  let ww = app.get_webview_window("quickInput").unwrap();
  ww.hide();

  use std::{thread, time};
  thread::sleep(time::Duration::from_millis(100));

  let mut enigo = Enigo::new(&Settings::default()).unwrap();
  // Paste
  enigo.key(Key::Control, Press);
  enigo.key(Key::Unicode('v'), Click);
  enigo.key(Key::Control, Release);

  Ok(())
}

#[tauri::command]
pub async fn updateQuickInputWindowSize(
  app: AppHandle,
  size: LogicalSize<f64>,
) -> Result<(), String> {
  let ww = app.get_webview_window("quickInput").unwrap();

  ww.set_size(Size::Logical(LogicalSize {
    width: size.width,
    height: size.height,
  }));

  Ok(())
}

#[tauri::command]
pub async fn hideQuickInputWindow(app: AppHandle) -> Result<(), String> {
  let ww = app.get_webview_window("quickInput").unwrap();

  ww.hide();
  Ok(())
}

#[tauri::command]
pub async fn get_package_info(app: AppHandle) -> Result<AppInfo, String> {
  let pi = app.package_info();

  dbg!(&pi);

  let vvv = &pi.version;
  let vec: Vec<String> = vec![&vvv.major, &vvv.minor, &vvv.patch]
    .into_iter()
    .map(|item| item.to_string())
    .collect();
  let version = vec.join(".");

  let info = AppInfo {
    name: pi.name.clone(),
    version: version,
    authors: pi.authors,
    description: pi.description,
    crate_name: pi.crate_name,
  };

  Ok(info)
}

#[derive(Serialize, Deserialize)]
pub struct AppInfo {
  name: String,
  version: String,
  authors: &'static str,
  description: &'static str,
  crate_name: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
  needUpdate: bool,
  current_version: String,
  version: String,
}

static mut updatePlay: Option<Update> = None;

use tauri_plugin_updater::{Update, UpdaterExt};
#[tauri::command]
pub async fn checkUpdate(app: tauri::AppHandle) -> tauri_plugin_updater::Result<UpdateInfo> {
  if let Some(update) = app.app_handle().updater()?.check().await? {
    info!("有更新");
    unsafe {
      updatePlay = Some(update.clone());
    };

    let update_info = UpdateInfo {
      needUpdate: true,
      current_version: update.current_version,
      version: update.version,
    };
    return Ok(update_info);
  }
  info!("没有更新");

  Ok(UpdateInfo {
    needUpdate: false,
    current_version: "".to_string(),
    version: "".to_string(),
  })
}

use tauri::{ipc::Channel, State};

use crate::store::AppData;

#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data")]
pub enum DownloadEvent {
  Started { content_length: Option<u64> },
  Progress { chunk_length: usize },
  Finished,
}
#[tauri::command]
pub async fn download_and_install(app: AppHandle, on_event: Channel<DownloadEvent>) {
  unsafe {
    let mut started = false;

    match updatePlay.as_ref() {
      Some(update) => {
        info!("开始下载 update");
        // Use `update` as a reference here
        // .download_and_install
        update
          .download_and_install(
            |chunk_length, content_length| {
              if !started {
                on_event.send(DownloadEvent::Started { content_length });
                started = true;
              }

              on_event.send(DownloadEvent::Progress { chunk_length });
              // downloaded += chunk_length;
              // println!("downloaded {downloaded} from {content_length:?}");
            },
            || {
              println!("download finished");
              info!("rust -> download finished");

              on_event.send(DownloadEvent::Finished);
            },
          )
          .await;

        println!("update installed");
        info!("rust -> update installed");
        // app.restart();
      }
      None => {}
    }
  }
}

// async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
//   if let Some(update) = app.updater()?.check().await? {
//     let mut downloaded = 0;

//     info!("rust -> 下载并安装");
//     // alternatively we could also call update.download() and update.install() separately
//     update
//       .download_and_install(
//         |chunk_length, content_length| {
//           downloaded += chunk_length;
//           println!("downloaded {downloaded} from {content_length:?}");
//         },
//         || {
//           println!("download finished");
//           info!("rust -> download finished");
//         },
//       )
//       .await?;

//     println!("update installed");
//     info!("rust -> update installed");
//     app.restart();
//   }

//   Ok(())
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandItem {
  label: String,
  cmd: String,        // node
  arg: String,        // index.js
  currentDir: String, // E:/rmst-sd
}

#[tauri::command]
pub async fn saveCommands(
  app: tauri::AppHandle,
  commands: Option<Vec<CommandItem>>,
) -> Result<(), String> {
  let appData = app.state::<AppData>();
  let store = &appData.store;

  let value = to_value(commands).unwrap_or(json!([]));

  store.set(Commands_Key, value);
  Ok(())
}

#[tauri::command]
pub async fn getCommands(app: tauri::AppHandle) -> Result<Vec<CommandItem>, String> {
  let appData = app.state::<AppData>();
  let store = &appData.store;

  let value = store.get(Commands_Key).unwrap_or(json!([]));

  let commands: Vec<CommandItem> = from_value(value).unwrap_or_default();

  Ok(commands)
}

#[tauri::command]
pub async fn execCommand(app: tauri::AppHandle, label: String) -> Result<(), String> {
  let appData = app.state::<AppData>();
  let store = &appData.store;

  let value = store.get(Commands_Key).unwrap_or(json!([]));

  let commands: Vec<CommandItem> = from_value(value).unwrap_or_default();

  let cmdItem = commands.iter().find(|item| item.label == label);

  match cmdItem {
    Some(cmdItem) => {
      let bb = CommandItem {
        label: cmdItem.label.clone(),
        cmd: cmdItem.cmd.clone(),
        arg: cmdItem.arg.clone(),
        currentDir: cmdItem.currentDir.clone(),
      };
      let result = execCommandItem(bb);

      dbg!(&result);
      return result;
    }
    None => {}
  }

  Ok(())
}

fn execCommandItem(commandItem: CommandItem) -> Result<(), String> {
  // 指定目标目录（可替换为实际路径）
  let target_dir = PathBuf::from(commandItem.currentDir);

  // 构建命令：node sc.js
  let mut cmd = Command::new(commandItem.cmd);
  let args: Vec<&str> = commandItem.arg.split_whitespace().collect();

  for arg in args {
    cmd.arg(arg);
  }

  cmd.current_dir(target_dir); // 设置工作目录

  // 执行命令并处理输出
  match cmd.status() {
    Ok(status) => {
      if status.success() {
        println!("命令执行成功");

        Ok(())
      } else {
        let str = format!("命令执行失败，退出码：{}", status.code().unwrap_or(-1));
        dbg!(&str);

        Err(str)
      }
    }
    Err(e) => {
      eprintln!("启动进程失败：{}", e);
      Err("启动进程失败：".to_string())
    }
  }
}

// #[tauri::command]
// pub async fn check_update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
//   if let Some(update) = app.updater()?.check().await? {
//     let mut downloaded = 0;

//     // alternatively we could also call update.download() and update.install() separately
//     update
//       .download_and_install(
//         |chunk_length, content_length| {
//           downloaded += chunk_length;
//           println!("downloaded {downloaded} from {content_length:?}");
//         },
//         || {
//           println!("download finished");
//         },
//       )
//       .await?;

//     println!("update installed");
//     app.restart();
//   }

//   Ok(())
// }
