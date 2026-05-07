use crate::error::AppError;
use crate::handler::CreateWord;
use crate::model::word::Word;
use crate::model::word::object::WordId;
use crate::repo::review::create_review;
use crate::repo::word::{active_words, create_new_word};
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, debug_handler};

#[debug_handler]
#[utoipa::path(context_path = "/word", post, path = "/add", tag = "word")]
pub async fn add_word_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateWord>,
) -> Result<(StatusCode, Json<WordId>), AppError> {
    let word_id = create_new_word(payload, &app_state.db).await?;
    create_review(&word_id, &app_state.db).await?;

    Ok((StatusCode::CREATED, Json(word_id)))
}

#[debug_handler]
#[utoipa::path(context_path = "/word", get, path = "/active", tag = "word")]
pub async fn active_word_handler(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Word>>), AppError> {
    let words = active_words(&app_state.db).await?;
    Ok((StatusCode::OK, Json(words)))
}
