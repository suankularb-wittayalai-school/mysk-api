use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use super::requests::FetchLevel;

#[async_trait]
pub trait FetchLevelVariant<T> {
    async fn from_table(
        table: T,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait BaseQuery {
    fn base_query() -> &'static str;
}

// only for struct with id: Uuid and implements BaseQuery
#[async_trait]
pub trait GetById {
    async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, Error>
    where
        Self: Sized;

    async fn get_by_ids(
        pool: &sqlx::PgPool,
        ids: Vec<sqlx::types::Uuid>,
    ) -> Result<Vec<Self>, Error>
    where
        Self: Sized;
}
