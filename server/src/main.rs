mod auth;
mod boot;
mod cli;
mod config;
mod crypto;
mod error;
mod extract;
mod handler;
mod model;
mod openapi;
mod repo;
mod state;

use crate::boot::{database, run_app};
use crate::cli::{Cli, Command, TokenCmd};
use crate::config::{AppConfig, load_config};
use clap::Parser;
use std::str::FromStr;
use tracing::{Level, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();
    let config = load_config();

    tracing_subscriber::fmt()
        .with_max_level(Level::from_str(&config.server.tracing_level).unwrap_or(Level::INFO))
        .init();

    let pool = database().await?;

    match cli.command {
        Some(Command::Token { cmd }) => match cmd {
            TokenCmd::Issue {
                role,
                name,
                ttl_days,
            } => {
                let (id, token) =
                    repo::auth::create_client_token(&config, &pool, name, role, ttl_days).await?;
                println!(
                    "New admin token (id={id}). Save it — it won't be shown again:"
                );
                println!("{token}");
                Ok(())
            }
        },
        None => {
            start_message(&config);
            run_app(config, pool).await
        }
    }
}

fn start_message(config: &AppConfig) {
    info!("---------------------------");
    info!("  wordbin v{}", env!("CARGO_PKG_VERSION"));
    info!("  by {}", env!("CARGO_PKG_AUTHORS"));
    info!("  → http://{}:{}", config.server.host, config.server.port);
    info!("---------------------------");
}
