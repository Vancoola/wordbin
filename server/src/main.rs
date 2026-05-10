mod boot;
mod config;
mod error;
mod handler;
mod model;
mod openapi;
mod repo;
mod state;
mod extract;

use crate::boot::run_app;
use crate::config::{AppConfig, load_config};
use std::str::FromStr;
use tracing::{Level, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = load_config();

    tracing_subscriber::fmt()
        .with_max_level(Level::from_str(&config.server.tracing_level).unwrap_or(Level::INFO))
        .init();
    start_message(&config);

    run_app(config).await
}

fn start_message(config: &AppConfig) {
    info!("---------------------------");
    info!("  wordbin v{}", env!("CARGO_PKG_VERSION"));
    info!("  by {}", env!("CARGO_PKG_AUTHORS"));
    info!("  → http://{}:{}", config.server.host, config.server.port);
    info!("---------------------------");
}
