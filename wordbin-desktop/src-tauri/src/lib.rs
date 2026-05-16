use tauri_plugin_store::StoreExt;

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