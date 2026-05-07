use crate::model::word::object::WordId;
use sqlx::SqlitePool;
use time::{Duration, OffsetDateTime};

pub async fn create_review(word_id: &WordId, pool: &SqlitePool) -> anyhow::Result<()> {
    let next_review = OffsetDateTime::now_utc() + Duration::seconds(10);

    sqlx::query!(
        "INSERT INTO reviews (word_id, result, reviewed_at) VALUES (?, ?, ?)",
        word_id,
        "hard",
        next_review
    )
    .execute(pool)
    .await?;
    Ok(())
}
