use serde::Serialize;

#[derive(Debug, Serialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Status {
    New,
    Known,
    Learning,
}

#[derive(Debug, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct WordId(i64);
impl WordId {
    pub fn new(id: i64) -> Self {
        WordId(id)
    }
}
