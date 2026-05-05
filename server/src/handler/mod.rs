use crate::error::AppError;
use crate::state::AppState;
use axum::debug_handler;
use axum::extract::State;
use axum::http::StatusCode;

#[debug_handler]
pub async fn add_word(State(app_state): State<AppState>) -> Result<StatusCode, AppError> {
    Ok(StatusCode::CREATED)
}
