use sqlx::{pool, PgPool};
use uuid::Uuid;

use super::requests::{
    FetchLevel, FilterConfig, PaginationConfig, QueryParam, SortingConfig, SqlSection,
};
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

pub trait TopLevelQuery<QueryableObject: Queryable, SortableObject> {
    async fn query(
        pool: &sqlx::PgPool,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
        filter: Option<&FilterConfig<QueryableObject>>,
        sort: Option<&SortingConfig<SortableObject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>>
    where
        Self: Sized;
}

/// A trait for Queryable objects with ability to convert to query string conditions
pub trait Queryable {
    // Convert to query string conditions
    fn to_query_string(&self) -> Vec<SqlSection>;
}

/// A trait for DB objects with ability to query from DB
pub trait QueryDb<QueryableObject: Queryable, SortableObject> {
    /// Query from DB
    async fn query(
        pool: &sqlx::PgPool,
        filter: Option<&FilterConfig<QueryableObject>>,
        sort: Option<&SortingConfig<SortableObject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>>
    where
        Self: Sized;
}
