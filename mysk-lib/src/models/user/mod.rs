use self::enums::user_role::UserRole;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

pub mod enums;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(query = "SELECT id, created_at, email, role, is_admin, onboarded FROM users")]
pub struct User {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub email: String,
    pub role: UserRole,
    pub is_admin: bool,
    pub onboarded: bool,
}

impl User {
    pub async fn get_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>> {
        let res =
            sqlx::query_as::<_, User>(format!("{} WHERE email = $1", Self::base_query()).as_str())
                .bind(email)
                .fetch_optional(pool)
                .await
                .map_err(|e| {
                    Error::InternalSeverError(e.to_string(), "User::get_by_email".to_string())
                })?;

        Ok(res)
    }
}
