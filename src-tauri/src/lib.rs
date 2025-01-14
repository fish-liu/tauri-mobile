use serde_json::json;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_store::{Store, StoreExt};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet<R: tauri::Runtime>(app: tauri::AppHandle<R>,name: &str) -> String {

    let storage = app.state::<Storage<R>>();
    storage.store.set("chat_type".to_string(), json!("chat_type"));

    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[allow(unused)]
struct Storage<R: tauri::Runtime> {
    store: Arc<Store<R>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            greet
        ])
        .setup(|app| {
            let store = app.store("app_data.json").unwrap();
            app.manage(Storage { store });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
