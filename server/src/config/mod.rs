use figment::Figment;
use figment::providers::{Format, Toml};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub tracing_level: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
}

pub fn load_config() -> AppConfig {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let app_toml = manifest_dir.join("App.toml");

    Figment::new()
        .merge(Toml::file(app_toml))
        .extract()
        .expect("Can't extract config")
}
