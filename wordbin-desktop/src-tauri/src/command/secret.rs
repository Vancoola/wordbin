use keyring_core::Entry;

#[tauri::command]
pub fn get_secret(service: &str, key: &str) -> Result<String, String> {
    Entry::new(service, key)
        .and_then(|e| e.get_password())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_secret(service: &str, key: &str, secret: &str) -> Result<(), String> {
    let entry = Entry::new(service, key).map_err(|e| e.to_string())?;
    entry.set_password(secret).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_secret(service: &str, key: &str) -> Result<(), String> {
    Entry::new(service, key)
        .and_then(|e| e.delete_credential())
        .map_err(|e| e.to_string())
}
