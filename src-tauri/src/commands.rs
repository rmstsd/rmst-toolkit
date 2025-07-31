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

// #[tauri::command]
// pub async fn setDirWindowSize(app: tauri::AppHandle, height: f64) -> Result<(), String> {
//   let win = app
//     .get_webview_window(WIN_LABEL_OPEN_FOLDER)
//     .expect("对方过后发过火");

//   win.set_size(Size::Logical(LogicalSize {
//     width: 800.0,
//     height,
//   }));
//   Ok(())
// }

// #[tauri::command]
// pub async fn updateQuickInputWindowSize(
//   app: AppHandle,
//   size: LogicalSize<f64>,
// ) -> Result<(), String> {
//   let ww = app.get_webview_window(WIN_LABEL_QUICK_INPUT).unwrap();

//   ww.set_size(Size::Logical(LogicalSize {
//     width: size.width,
//     height: size.height,
//   }));

//   Ok(())
// }

#[tauri::command]
pub async fn CopyAndPaste(app: AppHandle, content: &str) -> Result<(), String> {
  app.clipboard().write_text(content).unwrap();

  let ww = app.get_webview_window(WIN_LABEL_QUICK_INPUT).unwrap();
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandItem {
  label: String,
  cmd: String,        // node
  arg: String,        // index.js
  currentDir: String, // E:/rmst-sd
}

static Commands_Key: &str = "commands";

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
  let cmdName = commandItem.cmd.clone();
  let mut cmd = Command::new(commandItem.cmd);
  cmd.current_dir(target_dir); // 设置工作目录
  let args: Vec<&str> = commandItem.arg.split_whitespace().collect();
  for arg in args {
    cmd.arg(arg);
  }

  // 执行命令并处理输出
  match cmd.status() {
    Ok(status) => {
      if status.success() {
        println!("命令执行成功");

        Ok(())
      } else {
        let output: Result<std::process::Output, io::Error> = cmd.output();
        if let Ok(op) = output {
          let err = String::from_utf8(op.stderr).unwrap_or_default();
          info!("{}", err);
          return Err(err);
        } else {
          let str = format!("命令执行失败，退出码：{}", status);
          info!("{}", str);

          return Err(str);
        }
      }
    }
    Err(e) => {
      eprintln!("启动进程失败：{}", e);

      let error_msg = format!("Cmd execute command '{} {}", cmdName, e);
      info!("{}", error_msg);

      Err(error_msg)
    }
  }
}

#[tauri::command]
pub fn trayMenu(app: tauri::AppHandle, menuKey: &str) {
  dbg!(&menuKey);

  let tm_window = app.get_webview_window(window::WIN_LABEL_Tray_Menu).unwrap();
  tm_window.hide();

  match menuKey {
    "setting" => {
      let setting_window = app.get_webview_window(window::WIN_LABEL_SETTING);

      if let Some(ww) = setting_window {
        if ww.is_minimized().unwrap_or(false) {
          let _ = ww.unminimize();
        }

        let _ = ww.show();
        let _ = ww.set_focus();
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
  }
}
