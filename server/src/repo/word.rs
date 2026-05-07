use crate::model::word::Word;
use crate::model::word::object::WordId;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use wordbin_types::CreateWord;

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

pub async fn active_words(pool: &SqlitePool) -> anyhow::Result<Vec<Word>> {
    let words = sqlx::query_as!(
        Word,
        "SELECT id, word, source, status, added_at, notes
         FROM words
         WHERE status != 'known'
         ORDER BY added_at DESC"
    )
    .fetch_all(pool)
    .await?;
    Ok(words)
}
