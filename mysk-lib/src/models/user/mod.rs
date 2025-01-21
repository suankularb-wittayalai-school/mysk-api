use crate::{
    models::{enums::UserRole, traits::GetById as _, user::db::DbUser},
    prelude::*,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

pub mod db;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub email: String,
    pub role: UserRole,
    pub is_admin: bool,
    pub onboarded: bool,
    pub permissions: Vec<String>,
}

impl User {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Self> {
        let user = DbUser::get_by_id(pool, id).await?;
        let permissions = DbUser::get_user_permissions(pool, user.id).await?;

        Ok(Self {
            id: user.id,
            created_at: user.created_at,
            email: user.email,
            role: user.role,
            is_admin: user.is_admin,
            onboarded: user.onboarded,
            permissions,
        })
    }

    pub async fn get_by_email(pool: &PgPool, email: &str) -> Result<Self> {
        let id = DbUser::get_by_email(pool, email).await?;

        Self::get_by_id(pool, id).await
    }
}
