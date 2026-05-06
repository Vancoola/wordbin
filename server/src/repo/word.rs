use crate::handler::CreateWord;
use crate::model::{Word, WordId};
use sqlx::SqlitePool;

pub async fn create_word(word: CreateWord, pool: &SqlitePool) -> anyhow::Result<WordId> {
    let word_id = sqlx::query!(
        "INSERT INTO words (word, source, notes) VALUES (?, ?, ?)",
        word.word,
        word.source,
        word.notes
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(WordId::new(word_id))
}
