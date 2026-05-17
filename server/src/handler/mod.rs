use crate::handler::server::server_status;
use crate::handler::word::{active_word_handler, add_word_handler, word_count_handler};
use crate::state::AppState;
use axum::Router;
use axum::routing::{get, post};

mod auth;
pub mod server;
pub mod word;

pub fn word_router() -> Router<AppState> {
    Router::new()
        .route("/add", post(add_word_handler))
        .route("/count", get(word_count_handler))
        .route("/active", get(active_word_handler))
}

pub fn server_router() -> Router<AppState> {
    Router::new().route("/status", get(server_status))
}
