use crate::error::AppError;
use crate::handler::CreateWord;
use crate::model::WordId;
use crate::repo::word::create_word;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, debug_handler};

#[debug_handler]
#[utoipa::path(context_path = "/word", post, path = "/add", tag = "word")]
pub async fn add_word(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateWord>,
) -> Result<(StatusCode, Json<WordId>), AppError> {
    let word_id = create_word(payload, &app_state.db).await?;
    Ok((StatusCode::CREATED, Json(word_id)))
}
