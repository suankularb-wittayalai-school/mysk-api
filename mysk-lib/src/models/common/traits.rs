use sqlx::{pool, Error, PgPool};
use uuid::Uuid;

use super::requests::FetchLevel;

pub trait FetchLevelVariant<T> {
    async fn from_table(
        pool: &PgPool,
        table: T,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

// pub trait BaseQuery {
//     fn base_query() -> &'static str;
// }

// // only for struct with id: Uuid and implements BaseQuery
// pub trait GetById {
//     async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, Error>
//     where
//         Self: Sized;

//     async fn get_by_ids(
//         pool: &sqlx::PgPool,
//         ids: Vec<sqlx::types::Uuid>,
//     ) -> Result<Vec<Self>, Error>
//     where
//         Self: Sized;
// }

pub trait TopLevelFromTable<T> {
    async fn from_table(
        pool: &pool::Pool<sqlx::Postgres>,
        table: T,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait TopLevelGetById {
    async fn get_by_id(
        pool: &pool::Pool<sqlx::Postgres>,
        id: Uuid,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, Error>
    where
        Self: Sized;

    async fn get_by_ids(
        pool: &pool::Pool<sqlx::Postgres>,
        ids: Vec<Uuid>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>, Error>
    where
        Self: Sized;
}
