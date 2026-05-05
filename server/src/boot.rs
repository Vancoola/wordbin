use crate::handler::add_word;
use crate::state::AppState;
use axum::Router;
use axum::routing::post;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{SqlitePool, migrate};
use std::str::FromStr;

pub(crate) async fn run_app() -> anyhow::Result<()> {
    let pool = database().await?;

    let state = AppState { db: pool.clone() };

    let app = Router::new()
        .route("/word/add", post(add_word))
        .with_state(state);

    let addr = format!("{}:{}", "0.0.0.0", 1234);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn database() -> anyhow::Result<SqlitePool> {
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::from_str("sqlite://words.db")?.create_if_missing(true),
    )
    .await?;

    migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
