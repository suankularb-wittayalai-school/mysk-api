use crate::{models::enums::UserRole, prelude::*};
use chrono::{DateTime, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{query, FromRow, PgPool};
use uuid::Uuid;

#[derive(BaseQuery, Clone, Debug, Deserialize, FromRow, GetById, Serialize)]
#[base_query(query = "SELECT id, created_at, email, role, is_admin, onboarded FROM users")]
pub struct DbUser {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub email: String,
    pub role: UserRole,
    pub is_admin: bool,
    pub onboarded: bool,
}

impl DbUser {
    pub async fn get_by_email(pool: &PgPool, email: &str) -> Result<Option<Uuid>> {
        let res = query!("SELECT id FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                Error::InternalSeverError(e.to_string(), "DbUser::get_by_email".to_string())
            })?;

        Ok(res.map(|row| row.id))
    }

    pub async fn get_user_permissions(pool: &PgPool, user_id: Uuid) -> Result<Vec<String>> {
        let res = sqlx::query_as::<_, (String,)>(
            "
            SELECT permissions.name
            FROM user_permissions
            JOIN permissions ON user_permissions.permission_id = permissions.id
            WHERE user_permissions.user_id = $1
            ",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            Error::InternalSeverError(e.to_string(), "DbUser::get_user_permissions".to_string())
        })?;

        Ok(res.into_iter().map(|(name,)| name).collect())
    }
}
