use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub jti: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
}
