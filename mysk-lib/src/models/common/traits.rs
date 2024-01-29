use sqlx::{pool, PgPool};
use uuid::Uuid;

use super::requests::FetchLevel;
use crate::prelude::*;

pub trait FetchLevelVariant<T> {
    async fn from_table(
        pool: &PgPool,
        table: T,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self>
    where
        Self: Sized;
}

pub trait TopLevelFromTable<T> {
    async fn from_table(
        pool: &pool::Pool<sqlx::Postgres>,
        table: T,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self>
    where
        Self: Sized;
}

pub trait TopLevelGetById {
    async fn get_by_id(
        pool: &pool::Pool<sqlx::Postgres>,
        id: Uuid,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self>
    where
        Self: Sized;

    async fn get_by_ids(
        pool: &pool::Pool<sqlx::Postgres>,
        ids: Vec<Uuid>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>>
    where
        Self: Sized;
}
