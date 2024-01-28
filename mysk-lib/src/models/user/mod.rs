use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::enums::user_role::UserRole;

use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};

pub mod enums;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, BaseQuery, GetById)]
#[base_query(query = "SELECT id, created_at, email, role, is_admin, onboarded FROM users")]
pub struct User {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub email: String,
    pub role: UserRole,
    pub is_admin: bool,
    pub onboarded: bool,
}

// impl BaseQuery for User {
//     fn base_query() -> &'static str {
//         r#"SELECT id, created_at, email, role, is_admin, onboarded FROM users"#
//     }
// }

// impl GetById for User {
//     async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
//         sqlx::query_as::<_, User>(format!("{} WHERE id = $1", Self::base_query()).as_str())
//             .bind(id)
//             .fetch_one(pool)
//             .await
//     }

//     async fn get_by_ids(pool: &sqlx::PgPool, id: Vec<Uuid>) -> Result<Vec<Self>, sqlx::Error> {
//         sqlx::query_as::<_, User>(format!("{} WHERE id = ANY($1)", Self::base_query()).as_str())
//             .bind(id)
//             .fetch_all(pool)
//             .await
//     }
// }

impl User {
    pub async fn get_by_email(pool: &sqlx::PgPool, email: &str) -> Option<Self> {
        sqlx::query_as::<_, User>(format!("{} WHERE email = $1", Self::base_query()).as_str())
            .bind(email)
            .fetch_optional(pool)
            .await
            .unwrap()
    }
}
