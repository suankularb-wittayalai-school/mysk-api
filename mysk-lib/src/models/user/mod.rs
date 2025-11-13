use crate::{
    models::{enums::UserRole, traits::GetById as _, user::db::DbUser},
    prelude::*,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, query_scalar};
use uuid::Uuid;

pub mod db;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UserMeta {
    Student { student_id: Uuid },
    Teacher { teacher_id: Uuid },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub email: String,
    pub role: UserRole,
    pub meta: Option<UserMeta>,
    pub is_admin: bool,
    pub onboarded: bool,
    pub permissions: Vec<String>,
}

impl User {
    pub async fn get_by_id(conn: &mut PgConnection, id: Uuid, meta: Option<Uuid>) -> Result<Self> {
        let user = DbUser::get_by_id(conn, id).await?;
        let permissions = DbUser::get_user_permissions(conn, user.id).await?;
        let meta = match user.role {
            UserRole::Student if meta.is_some() => Some(UserMeta::Student {
                student_id: meta.unwrap(),
            }),
            UserRole::Student => Some(UserMeta::Student {
                student_id: query_scalar!(
                    "\
                    SELECT s.id \
                    FROM students AS s JOIN users AS u ON u.id = s.user_id \
                    WHERE u.id = $1\
                    ",
                    user.id,
                )
                .fetch_one(conn)
                .await?,
            }),
            UserRole::Teacher if meta.is_some() => Some(UserMeta::Teacher {
                teacher_id: meta.unwrap(),
            }),
            UserRole::Teacher => Some(UserMeta::Teacher {
                teacher_id: query_scalar!(
                    "\
                    SELECT t.id \
                    FROM teachers AS t JOIN users AS u ON u.id = t.user_id \
                    WHERE u.id = $1\
                    ",
                    user.id,
                )
                .fetch_one(conn)
                .await?,
            }),
            _ => None,
        };

        Ok(Self {
            id: user.id,
            meta,
            created_at: user.created_at,
            email: user.email,
            role: user.role,
            is_admin: user.is_admin,
            onboarded: user.onboarded,
            permissions,
        })
    }

    pub async fn get_by_email(conn: &mut PgConnection, email: &str) -> Result<Self> {
        let user_id = DbUser::get_id_by_email(conn, email).await?;

        Self::get_by_id(conn, user_id, None).await
    }
}
