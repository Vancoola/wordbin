use crate::handler::word::add_word;
use crate::state::AppState;
use axum::Router;
use axum::routing::post;
use serde::Deserialize;
use utoipa::ToSchema;

pub mod word;

pub fn word_router() -> Router<AppState> {
    Router::new().route("/add", post(add_word))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWord {
    pub word: String,
    pub source: String,
    pub notes: String,
}
