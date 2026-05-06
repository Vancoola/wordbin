use crate::handler::word_router;
use crate::openapi::ApiDoc;
use crate::state::AppState;
use axum::Router;
use axum::http::Method;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{SqlitePool, migrate};
use std::str::FromStr;
use std::time::Duration;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tracing::trace;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub(crate) async fn run_app() -> anyhow::Result<()> {
    let pool = database().await?;
    let cors = cors_layer()?;

    let state = AppState { db: pool.clone() };

    let app = Router::new()
        .nest("/word", word_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
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

    trace!("SQLite database connected");
    migrate!("./migrations").run(&pool).await?;
    trace!("Migrated DB");

    Ok(pool)
}

fn cors_layer() -> anyhow::Result<CorsLayer> {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(["http://localhost:8080".parse()?]))
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
            Method::PATCH,
        ]))
        .allow_headers(AllowHeaders::list([
            "content-type".parse()?,
            "authorization".parse()?,
            "accept".parse()?,
            "x-requested-with".parse()?,
            "access-control-allow-origin".parse()?,
        ]))
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));
    Ok(cors)
}
