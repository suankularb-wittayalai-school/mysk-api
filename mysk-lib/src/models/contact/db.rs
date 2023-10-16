use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::common::traits::{BaseQuery, GetById};

use super::enums::contact_type::ContactType;

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
pub struct DbContact {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: Option<String>,
    pub name_en: Option<String>,
    pub r#type: ContactType,
    pub value: String,
    pub include_students: Option<bool>,
    pub include_teachers: Option<bool>,
    pub include_parents: Option<bool>,
}

impl BaseQuery for DbContact {
    fn base_query() -> &'static str {
        r#"SELECT id, created_at, name_th, name_en, type, value, include_students, include_teachers, include_parents FROM contacts"#
    }
}

#[async_trait]
impl GetById for DbContact {
    async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, DbContact>(format!("{} WHERE id = $1", Self::base_query()).as_str())
            .bind(id)
            // sqlx::query_as!(DbContact, r#"SELECT id, created_at, name_th, name_en, type as "type: _", value, include_students, include_teachers, include_parents FROM contacts WHERE id = $1"#, id)
            .fetch_one(pool)
            .await
    }

    async fn get_by_ids(pool: &sqlx::PgPool, ids: Vec<Uuid>) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, DbContact>(
            format!("{} WHERE students.id = ANY($1)", Self::base_query()).as_str(),
        )
        .bind(ids)
        .fetch_all(pool)
        .await
    }
}
