use sqlx::SqlitePool;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub app_config: AppConfig
}
