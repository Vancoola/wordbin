pub mod object;

use crate::model::word::object::{Status, WordId};
use time::OffsetDateTime;

#[derive(Debug)]
pub struct Word {
    pub id: WordId,
    pub word: String,
    pub source: String,
    pub added_at: OffsetDateTime,
    pub notes: Option<String>,
    pub status: Status,
}
