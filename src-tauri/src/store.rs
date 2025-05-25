use tauri_plugin_store::{Store, StoreExt};

use std::{mem::MaybeUninit, sync::Mutex};

use tauri::App;

// Example with serde_json::Value as the value type:
// pub static mut globalStore: MaybeUninit<Store<String>> = MaybeUninit::uninit();

static Store_Key: &str = "store.json";
pub fn initStore(app: &mut App) {
  unsafe {
    // globalStore = app.store(Store_Key).unwrap();
  }
}
