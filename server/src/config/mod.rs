use figment::Figment;
use figment::providers::{Env, Format, Toml};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub tracing_level: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Secure {
    pub jwt: JwtConfig
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub security: Secure,
}

pub fn load_config() -> AppConfig {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let app_toml = manifest_dir.join("App.toml");

    Figment::new()
        .merge(Toml::file(app_toml))
        .merge(Env::prefixed("WORDBIN_").split("__"))
        .extract()
        .expect("Can't extract config")
}
