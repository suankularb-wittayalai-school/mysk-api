use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::query;
use uuid::Uuid;

use crate::models::common::traits::{BaseQuery, GetById};

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
pub struct DbClassroom {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub number: i64,
    pub year: i64,
    pub main_room: String,
}

impl BaseQuery for DbClassroom {
    fn base_query() -> &'static str {
        r#"SELECT id, created_at, number, year, main_room FROM classrooms"#
    }
}

#[async_trait]
impl GetById for DbClassroom {
    async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, DbClassroom>(format!("{} WHERE id = $1", Self::base_query()).as_str())
            .bind(id)
            .fetch_one(pool)
            .await
    }

    async fn get_by_ids(pool: &sqlx::PgPool, ids: Vec<Uuid>) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, DbClassroom>(
            format!("{} WHERE id = ANY($1)", Self::base_query()).as_str(),
        )
        .bind(ids)
        .fetch_all(pool)
        .await
    }
}
