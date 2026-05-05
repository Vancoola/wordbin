mod boot;
mod error;
mod handler;
mod state;

use crate::boot::run_app;
use tracing::{Level, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    start_message();

    run_app().await?;

    Ok(())
}

fn start_message() {
    info!("---------------------------");
    info!("  wordbin v{}", env!("CARGO_PKG_VERSION"));
    info!("  by {}", env!("CARGO_PKG_AUTHORS"));
    info!("  → http://{}", 123);
    info!("---------------------------");
}
