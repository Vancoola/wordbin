use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWord {
    pub word: String,
    pub source: String,
    pub notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WordCreatedId(pub i64);

#[derive(Debug, Serialize, Deserialize)]
pub struct WordResponse {
    pub id: i64,
    pub word: String,
    pub source: String,
    pub status: String,
    pub added_at: String,
}