use crate::handler::CreateWord;
use crate::model::word::object::WordId;
use sqlx::SqlitePool;
use time::OffsetDateTime;

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
