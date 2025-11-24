use log::error;
use log::info;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_value;
use serde_json::json;
use serde_json::to_value;
use std::io;
use std::path::PathBuf;
use std::process;
use std::process::Command;
use tauri::Manager;

use crate::commands::NodeModulesFolder;
use crate::store::AppData;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandItem {
  label: String,
  cmd: String,        // node
  arg: String,        // index.js
  currentDir: String, // E:/rmst-sd
}

static Commands_Key: &str = "commands";

#[tauri::command]
pub async fn saveCommands(app: tauri::AppHandle, commands: Option<Vec<CommandItem>>) -> Result<(), String> {
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
  let mut cmd = process::Command::new(commandItem.cmd);
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
pub fn removeFolder(nodeModulesFolders: Vec<NodeModulesFolder>) {
  nodeModulesFolders.iter().for_each(|item| {
    if (!item.selected.unwrap_or_default()) {
      return;
    }

    let target_dir = PathBuf::from(item.path.clone().unwrap_or_default());

    let output = Command::new("powershell")
      .current_dir(target_dir)
      .arg("-Command")
      .arg("Remove-Item -Recurse -Force node_modules")
      .output();

    match output {
      Ok(output) => {
        if output.status.success() {
          info!("成功删除目录 'node_modules'");
        } else {
          error!(
            "删除目录 'node_modules' 失败. Error: {:?}",
            String::from_utf8_lossy(&output.stderr)
          );
        }
      }
      Err(e) => {
        error!("执行失败 Failed to execute command. Error: {}", e);
      }
    }
  });
}
