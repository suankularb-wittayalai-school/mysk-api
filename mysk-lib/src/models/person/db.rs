use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::models::common::traits::GetById;

use super::enums::shirt_size::ShirtSize;

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
pub struct Person {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub prefix_th: String,
    pub prefix_en: Option<String>,
    pub first_name_th: String,
    pub first_name_en: Option<String>,
    pub last_name_th: String,
    pub last_name_en: Option<String>,
    pub middle_name_th: Option<String>,
    pub middle_name_en: Option<String>,
    pub nickname_th: Option<String>,
    pub nickname_en: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub citizen_id: Option<String>,
    pub profile: Option<String>,
    pub pants_size: Option<String>,
    pub shirt_size: Option<ShirtSize>,
}

#[async_trait]
impl GetById for Person {
    async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Person,
            r#"
            SELECT id, created_at, prefix_th, prefix_en, first_name_th, first_name_en, last_name_th, last_name_en, middle_name_th, middle_name_en, nickname_th, nickname_en, birthdate, citizen_id, profile, pants_size, shirt_size AS "shirt_size: _" FROM people
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    async fn get_by_ids(
        pool: &sqlx::PgPool,
        ids: Vec<sqlx::types::Uuid>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Person,
            r#"
            SELECT id, created_at, prefix_th, prefix_en, first_name_th, first_name_en, last_name_th, last_name_en, middle_name_th, middle_name_en, nickname_th, nickname_en, birthdate, citizen_id, profile, pants_size, shirt_size AS "shirt_size: _" FROM people
            WHERE id = ANY($1)
            "#,
            &ids
        )
        .fetch_all(pool)
        .await
    }
}
