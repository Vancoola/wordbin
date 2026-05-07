pub mod object;

use crate::model::word::object::{Status, WordId};
use serde::Serialize;
use time::OffsetDateTime;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Word {
    pub id: WordId,
    pub word: String,
    pub source: String,
    #[serde(with = "time::serde::rfc3339")]
    pub added_at: OffsetDateTime,
    pub notes: Option<String>,
    pub status: Status,
}
