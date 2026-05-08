use crate::error::AppError;
use crate::repo::review::create_review;
use crate::repo::word::{active_words, create_new_word, word_count};
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, debug_handler};
use wordbin_types::{CreateWord, WordCount, WordCreatedId, WordResponse};

#[debug_handler]
#[utoipa::path(context_path = "/word", post, path = "/add", tag = "word")]
pub async fn add_word_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateWord>,
) -> Result<(StatusCode, Json<WordCreatedId>), AppError> {
    let word_id = create_new_word(payload, &app_state.db).await?;
    create_review(&word_id, &app_state.db).await?;

    Ok((StatusCode::CREATED, Json(WordCreatedId(word_id.0))))
}

#[debug_handler]
#[utoipa::path(context_path = "/word", get, path = "/count", tag = "word")]
pub async fn word_count_handler(
    State(app_state): State<AppState>,
) -> Result<Json<WordCount>, AppError> {
    let count = word_count(&app_state.db).await?;
    Ok(Json(WordCount { count }))
}

#[debug_handler]
#[utoipa::path(context_path = "/word", get, path = "/active", tag = "word")]
pub async fn active_word_handler(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<WordResponse>>), AppError> {
    let words = active_words(&app_state.db).await?;
    let response = words.into_iter().map(WordResponse::from).collect();
    Ok((StatusCode::OK, Json(response)))
}
