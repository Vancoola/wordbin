mod command;

use crate::command::secret::*;
use crate::command::settings::*;
use keyring_core::set_default_store;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_keyring();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            get_setup_done,
            set_setup_done,
            get_settings,
            set_settings,
            get_secret,
            set_secret,
            delete_secret
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_keyring() {
    #[cfg(target_os = "windows")]
    {
        use windows_native_keyring_store::Store;
        set_default_store(Store::new().expect("Failed to init Windows credential store"));
    }
    #[cfg(target_os = "macos")]
    {
        use apple_native_keyring_store::keychain::Store;
        set_default_store(Store::new().expect("macOS Keychain failed"));
    }
    #[cfg(target_os = "linux")]
    {
        use dbus_secret_service_keyring_store::Store;
        set_default_store(Store::new().expect("Failed to init Secret Service"));
    }
}
