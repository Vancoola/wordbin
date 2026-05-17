use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStatus {
    pub version: String,
    pub word_count: i64,
    pub review_count: i64,
    pub learned_count: i64,
}