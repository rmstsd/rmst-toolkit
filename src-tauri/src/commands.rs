use enigo::{
  Direction::{Click, Press, Release},
  Enigo, Key, Keyboard, Settings,
};
use log::info;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_value;
use serde_json::json;
use serde_json::to_value;
use serde_json::Value;
use std::fs::metadata;
use std::fs::read_dir;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::vec;
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::{
  cmds::setting::getSetting,
  store::AppData,
  window::{self, WIN_LABEL_QUICK_INPUT},
};

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
pub struct NodeModulesFolder {
  pub path: Option<String>,
  pub selected: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingData {
  cmdPath: Option<String>,
  editorPaths: Option<Vec<String>>,
  projectPaths: Option<Vec<String>>,
  notes: Option<Vec<String>>,

  nodeModulesFolders: Option<Vec<NodeModulesFolder>>,
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
          Command::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output()?;
        }
      }
    }
  }
  #[cfg(target_os = "linux")]
  {
    // 在 Linux 上查找占用指定端口的进程 ID
    let output = Command::new("sh").arg("-c").arg(&format!("lsof -t -i:{}", port)).output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
      if let Ok(pid) = line.parse::<u32>() {
        // 杀死找到的进程
        Command::new("kill").args(&["-9", &pid.to_string()]).output()?;
      }
    }
  }
  #[cfg(target_os = "macos")]
  {
    // 在 macOS 上查找占用指定端口的进程 ID
    let output = Command::new("sh").arg("-c").arg(&format!("lsof -t -i:{}", port)).output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
      if let Ok(pid) = line.parse::<u32>() {
        // 杀死找到的进程
        Command::new("kill").args(&["-9", &pid.to_string()]).output()?;
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
    nodeModulesFolders: Some(vec![]),
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
        .filter(|item| !blackList.contains(&item.as_str()) && !blackStartWithChar.iter().any(|char| item.starts_with(char)))
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
pub async fn openFolderEditor(app: tauri::AppHandle, projectPath: String, editorPath: String) -> Result<(), String> {
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
pub async fn CopyAndPaste(app: AppHandle, content: &str) -> Result<(), tauri::Error> {
  app.clipboard().write_text(content).unwrap();

  let ww = app.get_webview_window(WIN_LABEL_QUICK_INPUT).unwrap();
  ww.hide()?;

  // use std::{thread, time};
  // thread::sleep(time::Duration::from_millis(100));

  let mut enigo = Enigo::new(&Settings::default()).unwrap();
  // Paste
  enigo.key(Key::Control, Press);
  enigo.key(Key::Unicode('v'), Click);
  enigo.key(Key::Control, Release);

  Ok(())
}

#[tauri::command]
pub fn trayMenu(app: tauri::AppHandle, ww: tauri::WebviewWindow, menuKey: &str) -> Result<(), tauri::Error> {
  dbg!(&menuKey);

  ww.hide()?;

  match menuKey {
    "setting" => {
      let setting_window = app.get_webview_window(window::WIN_LABEL_SETTING);

      if let Some(ww) = setting_window {
        if ww.is_minimized().unwrap_or(false) {
          ww.unminimize()?;
        }

        ww.show()?;
        ww.set_focus()?;
      }
    }
    "restart" => {
      app.restart();
    }
    "quit" => {
      app.exit(0);
    }
    _ => {
      dbg!(&"未匹配的 menuKey");
    }
  };

  Ok(())
}
