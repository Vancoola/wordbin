use crate::handler::word::{active_word_handler, add_word_handler};
use crate::state::AppState;
use axum::Router;
use axum::routing::{get, post};
use serde::Deserialize;
use utoipa::ToSchema;

pub mod word;

pub fn word_router() -> Router<AppState> {
    Router::new()
        .route("/add", post(add_word_handler))
        .route("/active", get(active_word_handler))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWord {
    pub word: String,
    pub source: String,
    pub notes: String,
}
