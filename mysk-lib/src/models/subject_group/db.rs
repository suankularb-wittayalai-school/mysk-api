use chrono::{DateTime, Utc};
use mysk_lib_derives::BaseQuery;
use mysk_lib_macros::traits::db::BaseQuery;

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow, BaseQuery)]
#[base_query(query = "SELECT id, created_at, name_th, name_en FROM subject_groups")]
pub struct DbSubjectGroup {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: String,
    pub name_en: String,
}
impl DbSubjectGroup {
    pub async fn get_by_id(pool: &sqlx::PgPool, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, DbSubjectGroup>(
            format!("{} WHERE id = $1", Self::base_query()).as_str(),
        )
        .bind(id)
        // sqlx::query_as!(DbContact, r#"SELECT id, created_at, name_th, name_en, type as "type: _", value, include_students, include_teachers, include_parents FROM contacts WHERE id = $1"#, id)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_ids(pool: &sqlx::PgPool, ids: Vec<i64>) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, DbSubjectGroup>(
            format!("{} WHERE id = ANY($1)", Self::base_query()).as_str(),
        )
        .bind(ids)
        .fetch_all(pool)
        .await
    }
}
