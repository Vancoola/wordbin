use crate::error::AppError;
use crate::extract::auth::Authenticated;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, debug_handler};
use wordbin_types::server::ServerStatus;
use crate::repo::review::review_count;
use crate::repo::word::word_count;

#[utoipa::path(get, path = "/healthz", tag = "server")]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[debug_handler]
#[utoipa::path(
    context_path = "/server",
    get,
    path = "/status",
    tag = "server",
    security(
        ("api_jwt_token" = [])
    )
)]
pub async fn server_status(
    _: Authenticated,
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<ServerStatus>), AppError> {

    let words = word_count(&app_state.db).await?;
    let reviews = review_count(&app_state.db).await?;

    let status = ServerStatus {
        version: format!("{}", env!("CARGO_PKG_VERSION")),
        word_count: words,
        review_count: reviews,
        learned_count: 0,
    };
    Ok((StatusCode::OK, Json(status)))
}
