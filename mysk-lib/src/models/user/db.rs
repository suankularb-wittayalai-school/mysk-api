use crate::{models::enums::UserRole, prelude::*};
use chrono::{DateTime, Utc};
use mysk_lib_macros::GetById;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgConnection, query, query_scalar};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, FromRow, GetById, Serialize)]
#[from_query(query = "SELECT id, created_at, email, role, is_admin, onboarded FROM users")]
pub struct DbUser {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub email: String,
    pub role: UserRole,
    pub is_admin: bool,
    pub onboarded: bool,
}

impl DbUser {
    pub async fn get_id_by_email(conn: &mut PgConnection, email: &str) -> Result<Uuid> {
        let res = query_scalar!("SELECT id FROM users WHERE email = $1", email)
            .fetch_one(conn)
            .await?;

        Ok(res)
    }

    pub async fn get_user_permissions(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<Vec<String>> {
        let res = query!(
            "\
            SELECT permissions.name FROM user_permissions \
            JOIN permissions ON user_permissions.permission_id = permissions.id \
            WHERE user_permissions.user_id = $1\
            ",
            user_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(res.into_iter().map(|row| row.name).collect())
    }
}
