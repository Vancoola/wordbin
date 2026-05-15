pub mod object;

use crate::model::word::object::{Status, WordId};
use serde::Serialize;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use wordbin_types::word::WordResponse;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Word {
    pub id: WordId,
    pub value: String,
    pub source: String,
    #[serde(with = "time::serde::rfc3339")]
    pub added_at: OffsetDateTime,
    pub notes: Option<String>,
    pub status: Status,
}

impl From<Word> for WordResponse {
    fn from(value: Word) -> Self {
        Self {
            id: value.id.0,
            word: value.value,
            source: value.source,
            status: value.status.to_string(),
            added_at: value.added_at.format(&Rfc3339).unwrap_or_default(),
        }
    }
}
