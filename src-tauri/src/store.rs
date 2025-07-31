use std::sync::Arc;
use tauri::Manager;
use tauri::{App, Wry};
use tauri_plugin_store::{Store, StoreExt};

static Store_Key: &str = "store.json";

pub struct AppData {
  pub store: Arc<Store<Wry>>,
}

pub fn initStore(app: &mut App) {
  app.manage(AppData {
    store: app.store(Store_Key).unwrap(),
  });
}
