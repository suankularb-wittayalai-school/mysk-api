#![allow(async_fn_in_trait)]

use crate::{
    common::{
        requests::{FetchLevel, FilterConfig, PaginationConfig, SortingConfig, SqlSection},
        response::PaginationType,
    },
    prelude::*,
};
use mysk_lib_macros::traits::db::BaseQuery;
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::fmt::Display;
use uuid::Uuid;

/// A trait for Fetch Level Variants of a database entity with ability to convert to be converted
/// from DB variant.
pub trait FetchLevelVariant<T> {
    async fn from_table(
        pool: &PgPool,
        table: T,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self>
    where
        Self: Sized;
}

/// A trait for the actual database entity with ability to convert to be converted from DB variant.
pub trait TopLevelFromTable<T> {
    async fn from_table(
        pool: &PgPool,
        table: T,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self>
    where
        Self: Sized;
}

pub trait TopLevelGetById {
    async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self>
    where
        Self: Sized;

    async fn get_by_ids(
        pool: &PgPool,
        ids: Vec<Uuid>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>>
    where
        Self: Sized;
}

pub trait TopLevelQuery<
    DbVariant: QueryDb<QueryableObject, SortableObject> + BaseQuery,
    QueryableObject: Queryable,
    SortableObject: Display,
>
{
    async fn query(
        pool: &PgPool,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
        filter: Option<&FilterConfig<QueryableObject>>,
        sort: Option<&SortingConfig<SortableObject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>>
    where
        Self: TopLevelFromTable<DbVariant> + Sized,
    {
        let models = DbVariant::query(pool, filter, sort, pagination).await?;

        let mut result = vec![];

        for variant in models {
            result
                .push(Self::from_table(pool, variant, fetch_level, descendant_fetch_level).await?);
        }

        Ok(result)
    }

    async fn response_pagination(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableObject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType>
    where
        Self: Sized,
    {
        DbVariant::response_pagination(pool, filter, pagination).await
    }
}

/// A trait for Queryable objects with ability to convert to query string conditions.
pub trait Queryable {
    // Convert to query string conditions
    fn to_query_string(&self) -> Vec<SqlSection>;
}

/// A trait for DB variant to allow querying and creating pagination response.
pub trait QueryDb<QueryableObject: Queryable, SortableObject: Display> {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<&FilterConfig<QueryableObject>>,
    ) where
        Self: Sized;

    async fn query(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableObject>>,
        sort: Option<&SortingConfig<SortableObject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>>
    where
        Self: BaseQuery + Sized;

    async fn response_pagination(
        pool: &sqlx::PgPool,
        filter: Option<&FilterConfig<QueryableObject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType>
    where
        Self: Sized;
}
