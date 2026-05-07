use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Status {
    New,
    Known,
    Learning,
}
impl From<String> for Status {
    fn from(s: String) -> Self {
        match s.as_str() {
            "known" => Status::Known,
            "learning" => Status::Learning,
            _ => Status::New,
        }
    }
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::New => write!(f, "new"),
            Status::Known => write!(f, "known"),
            Status::Learning => write!(f, "learning"),
        }
    }
}

#[derive(Debug, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct WordId(pub i64);
impl WordId {
    pub fn new(id: i64) -> Self {
        WordId(id)
    }
}
impl From<i64> for WordId {
    fn from(id: i64) -> Self {
        WordId(id)
    }
}
