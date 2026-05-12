use crate::model::word::Word;
use crate::model::word::object::WordId;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use wordbin_types::word::CreateWord;

pub async fn create_new_word(word: CreateWord, pool: &SqlitePool) -> anyhow::Result<WordId> {
    let added = OffsetDateTime::now_utc();

    let word_id = sqlx::query!(
        "INSERT INTO words (word, source, notes, status, added_at) VALUES (?, ?, ?, ?, ?)",
        word.word,
        word.source,
        word.notes,
        "new",
        added,
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(WordId::new(word_id))
}

pub async fn word_count(pool: &SqlitePool) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM words")
        .fetch_one(pool)
        .await?;
    Ok(count)
}

pub async fn active_words(
    pool: &SqlitePool,
    limit: i64,
    offset: i64,
    status: Option<String>,
) -> anyhow::Result<Vec<Word>> {
    match status {
        Some(s) => {
            sqlx::query_as!(
                Word,
                "SELECT id, word, source, status, added_at, notes
             FROM words WHERE status = ?
             ORDER BY added_at DESC LIMIT ? OFFSET ?",
                s,
                limit,
                offset
            )
            .fetch_all(pool)
            .await
        }
        None => {
            sqlx::query_as!(
                Word,
                "SELECT id, word, source, status, added_at, notes
             FROM words
             ORDER BY added_at DESC LIMIT ? OFFSET ?",
                limit,
                offset
            )
            .fetch_all(pool)
            .await
        }
    }
    .map_err(Into::into)
}
