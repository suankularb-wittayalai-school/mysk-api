use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::enums::user_role::UserRole;

use super::common::traits::{BaseQuery, GetById};

pub mod enums;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub email: String,
    pub role: UserRole,
    pub is_admin: bool,
    pub onboarded: bool,
}

impl BaseQuery for User {
    fn base_query() -> &'static str {
        r#"SELECT id, created_at, email, role, is_admin, onboarded FROM users"#
    }
}

impl GetById for User {
    async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>(format!("{} WHERE id = $1", Self::base_query()).as_str())
            .bind(id)
            .fetch_one(pool)
            .await
    }

    async fn get_by_ids(pool: &sqlx::PgPool, id: Vec<Uuid>) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>(format!("{} WHERE id = ANY($1)", Self::base_query()).as_str())
            .bind(id)
            .fetch_all(pool)
            .await
    }
}
