use chrono::{DateTime, Utc};

use crate::prelude::*;
use mysk_lib_derives::BaseQuery;
use mysk_lib_macros::traits::db::BaseQuery;

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow, BaseQuery)]
#[base_query(query = "SELECT subject_groups.id, created_at, name_th, name_en FROM subject_groups")]
pub struct DbSubjectGroup {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: String,
    pub name_en: String,
}
impl DbSubjectGroup {
    pub async fn get_by_id(pool: &sqlx::PgPool, id: i64) -> Result<Self> {
        let res = sqlx::query_as::<_, DbSubjectGroup>(
            format!("{} WHERE subject_groups.id = $1", Self::base_query()).as_str(),
        )
        .bind(id)
        // sqlx::query_as!(DbContact, r#"SELECT id, created_at, name_th, name_en, type as "type: _", value, include_students, include_teachers, include_parents FROM contacts WHERE id = $1"#, id)
        .fetch_one(pool)
        .await;

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbSubjectGroup::get_by_id".to_string(),
            )),
        }
    }

    pub async fn get_by_ids(pool: &sqlx::PgPool, ids: Vec<i64>) -> Result<Vec<Self>> {
        let res = sqlx::query_as::<_, DbSubjectGroup>(
            format!("{} WHERE subject_groups.id = ANY($1)", Self::base_query()).as_str(),
        )
        .bind(ids)
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbSubjectGroup::get_by_ids".to_string(),
            )),
        }
    }
}
