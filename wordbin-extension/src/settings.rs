use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use crate::i18n::Locale;

const KEY: &str = "wordbin_settings";


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub language: Locale,
    pub server_url: String,
    pub auto_detect_source: bool,
    pub close_after_save: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            language: Locale::en,
            server_url: "http://localhost:3000".to_string(),
            auto_detect_source: true,
            close_after_save: false,
        }
    }
}

pub fn load() -> Settings {
    LocalStorage::get::<Settings>(KEY).unwrap_or_default()
}
pub fn save(settings: &Settings) {
    let _ = LocalStorage::set(KEY, settings);
}