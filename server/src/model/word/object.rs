use serde::Serialize;

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

#[derive(Debug, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct WordId(i64);
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
