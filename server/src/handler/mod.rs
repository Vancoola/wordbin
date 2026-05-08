use crate::handler::word::{active_word_handler, add_word_handler, word_count_handler};
use crate::state::AppState;
use axum::Router;
use axum::routing::{get, post};

pub mod word;

pub fn word_router() -> Router<AppState> {
    Router::new()
        .route("/add", post(add_word_handler))
        .route("/count", get(word_count_handler))
        .route("/active", get(active_word_handler))
}

#[utoipa::path(get, path = "/healthz", tag = "status")]
pub async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
