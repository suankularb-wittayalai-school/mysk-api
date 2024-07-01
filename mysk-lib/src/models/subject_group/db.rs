use crate::prelude::*;
use chrono::{DateTime, Utc};
use mysk_lib_derives::BaseQuery;
use mysk_lib_macros::traits::db::BaseQuery;
use serde::Deserialize;
use sqlx::{query_as, FromRow, PgPool};

#[derive(Debug, Clone, Deserialize, FromRow, BaseQuery)]
#[base_query(query = "SELECT id, created_at, name_th, name_en FROM subject_groups")]
pub struct DbSubjectGroup {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: String,
    pub name_en: String,
}

// We're not using the GetById derive here because the ID of this table is an integer not a UUID.
impl DbSubjectGroup {
    pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Self> {
        let res = query_as::<_, DbSubjectGroup>(
            format!("{} WHERE subject_groups.id = $1", Self::base_query()).as_str(),
        )
        .bind(id)
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

    pub async fn get_by_ids(pool: &PgPool, ids: Vec<i64>) -> Result<Vec<Self>> {
        let res = query_as::<_, DbSubjectGroup>(
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
