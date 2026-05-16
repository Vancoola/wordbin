use tauri_plugin_store::StoreExt;
use shared_types::setting::Settings;

#[tauri::command]
async fn get_setup_done(app: tauri::AppHandle) -> bool {
    let store = app.store("settings.json").unwrap();
    store.get("setup_done")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

#[tauri::command]
async fn set_setup_done(app: tauri::AppHandle, done: bool) -> Result<(), String> {
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    store.set("setup_done", done);
    store.save().map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_settings(app: tauri::AppHandle) -> Settings {
    let store = app.store("settings.json").unwrap();
    store.get("settings")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default()
}

#[tauri::command]
async fn set_settings(app: tauri::AppHandle, settings: Settings) -> Result<(), String> {
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    store.set("settings", serde_json::to_value(&settings).unwrap());
    store.save().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            get_setup_done,
            set_setup_done,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}