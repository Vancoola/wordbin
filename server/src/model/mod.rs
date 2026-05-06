use serde::Serialize;
use time::OffsetDateTime;

#[derive(Debug)]
pub struct Word {
    pub id: WordId,
    pub word: String,
    pub source: String,
    pub added_at: OffsetDateTime,
    pub notes: String,
}

#[derive(Debug, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct WordId(i64);
impl WordId {
    pub fn new(id: i64) -> Self {
        WordId(id)
    }
}
