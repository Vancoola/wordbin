use serde::{Deserialize, Serialize};
use crate::cli::RoleArg;

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
}
impl From<RoleArg> for Role {
    fn from(role: RoleArg) -> Self {
        match role {
            RoleArg::Admin => Role::Admin,
            RoleArg::User => Role::User,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub jti: String,
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
}
