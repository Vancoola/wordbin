use crate::model::review::{Card, Grade};
use crate::model::word::object::WordId;
use anyhow::Context;
use fsrs::{FSRS, MemoryState};
use sqlx::SqlitePool;
use time::OffsetDateTime;

pub async fn create_card(word_id: &WordId, pool: &SqlitePool) -> anyhow::Result<()> {
    let word_id_inner = word_id.0;
    let now = OffsetDateTime::now_utc();
    sqlx::query!(
        "INSERT INTO cards (word_id, next_review) VALUES (?, ?)",
        word_id_inner,
        now
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn submit_review(
    word_id: &WordId,
    grade: Grade,
    fsrs: &FSRS,
    pool: &SqlitePool,
) -> anyhow::Result<()> {
    let card = load_card(word_id, pool)
        .await?
        .context("card not found for word")?;
    let now = OffsetDateTime::now_utc();
    let outcome = card.review(grade, now, fsrs)?;

    let word_id_inner = word_id.0;
    let stability = outcome.card.memory.map(|m| m.stability as f64);
    let difficulty = outcome.card.memory.map(|m| m.difficulty as f64);
    let lapses = outcome.card.lapses as i64;
    let last_review = outcome.card.last_review;
    let next_review = outcome.card.next_review;
    let rating = outcome.rating as i64;
    let delta_t = outcome.delta_t as i64;

    let mut tx = pool.begin().await?;

    sqlx::query!(
        "UPDATE cards
         SET stability = ?, difficulty = ?, lapses = ?, last_review = ?, next_review = ?
         WHERE word_id = ?",
        stability,
        difficulty,
        lapses,
        last_review,
        next_review,
        word_id_inner,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        "INSERT INTO reviews (word_id, rating, delta_t, reviewed_at) VALUES (?, ?, ?, ?)",
        word_id_inner,
        rating,
        delta_t,
        now,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

async fn load_card(word_id: &WordId, pool: &SqlitePool) -> anyhow::Result<Option<Card>> {
    let word_id_inner = word_id.0;
    let row = sqlx::query!(
        r#"SELECT stability, difficulty, lapses, last_review, next_review
           FROM cards WHERE word_id = ?"#,
        word_id_inner
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| {
        let memory = match (r.stability, r.difficulty) {
            (Some(s), Some(d)) => Some(MemoryState {
                stability: s as f32,
                difficulty: d as f32,
            }),
            _ => None,
        };
        Card {
            word_id: word_id.clone(),
            memory,
            lapses: r.lapses as u32,
            last_review: r.last_review,
            next_review: r.next_review,
        }
    }))
}

pub async fn review_count(pool: &SqlitePool) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM reviews")
        .fetch_one(pool)
        .await?;
    Ok(count)
}
