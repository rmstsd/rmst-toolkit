use std::process::Command;
use tokio::time::{interval, Duration};

const Proxy_Addr: &str = "http://127.0.0.1:7897";

fn is_proxy_enabled() -> bool {
  use winreg::enums::*;
  use winreg::RegKey;

  let hkcu = RegKey::predef(HKEY_CURRENT_USER);
  let key = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings");

  match key {
    Ok(key) => {
      let val: u32 = key.get_value("ProxyEnable").unwrap_or(0);
      val == 1
    }
    Err(_) => false,
  }
}

fn set_git_proxy() {
  let _ = Command::new("git")
    .args(["config", "--global", "http.proxy", Proxy_Addr])
    .output();
  let _ = Command::new("git")
    .args(["config", "--global", "https.proxy", Proxy_Addr])
    .output();
  log::info!("git 代理已设置: {}", Proxy_Addr);
}

fn unset_git_proxy() {
  let _ = Command::new("git")
    .args(["config", "--global", "--unset", "http.proxy"])
    .output();
  let _ = Command::new("git")
    .args(["config", "--global", "--unset", "https.proxy"])
    .output();
  log::info!("git 代理已移除");
}

pub fn start_proxy_watcher() {
  tokio::spawn(async move {
    let mut ticker = interval(Duration::from_secs(3));
    let mut last_state: Option<bool> = None;

    loop {
      ticker.tick().await;

      let enabled = is_proxy_enabled();

      if last_state != Some(enabled) {
        if enabled {
          set_git_proxy();
        } else {
          unset_git_proxy();
        }
        last_state = Some(enabled);
      }
    }
  });
}
