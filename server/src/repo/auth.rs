use crate::auth::create_token;
use crate::cli::RoleArg;
use crate::config::AppConfig;
use crate::crypto::hash_token;
use sqlx::SqlitePool;
use time::{Duration, OffsetDateTime};

pub async fn create_client_token(
    app_config: &AppConfig,
    pool: &SqlitePool,
    name: String,
    role_arg: RoleArg,
    ttl_days: Option<i64>,
) -> anyhow::Result<(i64, String)> {
    let token = create_token(&app_config.security.jwt.secret)?;
    let hash = hash_token(&token);

    let expires_at = ttl_days
        .map(|d| OffsetDateTime::now_utc() + Duration::days(d))
        .unwrap_or(OffsetDateTime::now_utc() + Duration::days(1));

    let created_at = OffsetDateTime::now_utc();

    let is_admin = match role_arg {
        RoleArg::Admin => true,
        RoleArg::User => false,
    };
    let id = sqlx::query!(
        "INSERT INTO tokens (name, token_hash, is_revoked, is_admin, created_at, expires_at) VALUES (?, ?, ?, ?, ?, ?)",
        name,
        hash,
        false,
        is_admin,
        created_at,
        expires_at,
    ).execute(pool).await?.last_insert_rowid();
    Ok((id, token))
}
