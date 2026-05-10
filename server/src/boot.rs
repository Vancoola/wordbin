use crate::auth::create_token;
use crate::config::AppConfig;
use crate::handler::{health_check, word_router};
use crate::openapi::ApiDoc;
use crate::state::AppState;
use axum::Router;
use axum::http::Method;
use axum::routing::get;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{SqlitePool, migrate};
use std::str::FromStr;
use std::time::Duration;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tracing::{info, trace};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub(crate) async fn run_app(app_config: AppConfig, pool: SqlitePool) -> anyhow::Result<()> {
    let cors = cors_layer()?;

    let token = create_token(&app_config.security.jwt.secret)?;
    info!("API token: {}", token);

    let state = AppState {
        db: pool.clone(),
        app_config: app_config.clone(),
    };

    let app = Router::new()
        .route("/healthz", get(health_check))
        .nest("/word", word_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", app_config.server.host, app_config.server.port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

pub async fn database() -> anyhow::Result<SqlitePool> {
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
