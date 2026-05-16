use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Settings {
    pub theme: String,
    pub language: String,
    pub server_url: String,
}