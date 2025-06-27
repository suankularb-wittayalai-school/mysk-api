use chrono::{DateTime, Utc};
use mysk_lib_macros::GetById;
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, FromRow, GetById)]
#[from_query(
    id = "i64",
    query = "SELECT id, created_at, name_th, name_en FROM subject_groups"
)]
pub struct DbSubjectGroup {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: String,
    pub name_en: String,
}
