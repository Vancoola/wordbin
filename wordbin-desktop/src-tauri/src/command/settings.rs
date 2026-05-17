use shared_types::settings::Settings;
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub async fn get_setup_done(app: tauri::AppHandle) -> bool {
    let store = app.store("settings.json").unwrap();
    store
        .get("setup_done")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

#[tauri::command]
pub async fn set_setup_done(app: tauri::AppHandle, done: bool) -> Result<(), String> {
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    store.set("setup_done", done);
    store.save().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_settings(app: tauri::AppHandle) -> Settings {
    let store = app.store("settings.json").unwrap();
    store
        .get("settings")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default()
}

#[tauri::command]
pub async fn set_settings(app: tauri::AppHandle, settings: Settings) -> Result<(), String> {
    println!("Setting {}", settings.server_url);
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    store.set("settings", serde_json::to_value(&settings).unwrap());
    store.save().map_err(|e| e.to_string())
}
