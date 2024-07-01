use crate::{
    common::{
        requests::{FetchLevel, FilterConfig, PaginationConfig, SortingConfig, SqlSection},
        response::PaginationType,
    },
    prelude::*,
};
use async_trait::async_trait;
use mysk_lib_macros::traits::db::BaseQuery;
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::fmt::Display;
use uuid::Uuid;

/// A trait for Fetch Level Variants of a database entity with ability to convert to be converted
/// from DB variant.
#[async_trait]
pub trait FetchLevelVariant<T>
where
    Self: Sized,
{
    async fn from_table(
        pool: &PgPool,
        table: T,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self>;
}

/// A trait for the actual database entity with ability to convert to be converted from DB variant.
#[async_trait]
pub trait TopLevelFromTable<T>
where
    Self: Sized,
{
    async fn from_table(
        pool: &PgPool,
        table: T,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self>;
}

#[async_trait]
pub trait TopLevelGetById
where
    Self: Sized,
{
    async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self>;

    async fn get_by_ids(
        pool: &PgPool,
        ids: Vec<Uuid>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>>;
}

#[async_trait]
pub trait TopLevelQuery<DbVariant, QueryableObject, SortableObject>
where
    Self: TopLevelFromTable<DbVariant> + Sized + 'static,
    DbVariant: QueryDb<QueryableObject, SortableObject> + BaseQuery + Send + 'static,
    QueryableObject: Queryable + Sync,
    SortableObject: Display + Sync,
{
    async fn query(
        pool: &PgPool,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
        filter: Option<&FilterConfig<QueryableObject>>,
        sort: Option<&SortingConfig<SortableObject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>> {
        let models = DbVariant::query(pool, filter, sort, pagination).await?;
        let fetch_level = fetch_level.copied();
        let descendant_fetch_level = descendant_fetch_level.copied();
        let futures: Vec<_> = models
            .into_iter()
            .map(|model| {
                let pool = pool.clone();

                tokio::spawn(async move {
                    Self::from_table(
                        &pool,
                        model,
                        fetch_level.as_ref(),
                        descendant_fetch_level.as_ref(),
                    )
                    .await
                })
            })
            .collect();

        let mut result = Vec::with_capacity(futures.len());
        for future in futures {
            result.push(future.await.unwrap()?);
        }

        Ok(result)
    }

    async fn response_pagination(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableObject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType> {
        DbVariant::response_pagination(pool, filter, pagination).await
    }
}

/// A trait for Queryable objects with ability to convert to query string conditions.
pub trait Queryable {
    // Convert to query string conditions
    fn to_query_string(&self) -> Vec<SqlSection>;
}

/// A trait for DB variant to allow querying and creating pagination response.
#[async_trait]
pub trait QueryDb<QueryableObject, SortableObject>
where
    Self: Sized + BaseQuery,
    QueryableObject: Queryable,
    SortableObject: Display,
{
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<&FilterConfig<QueryableObject>>,
    );

    async fn query(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableObject>>,
        sort: Option<&SortingConfig<SortableObject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>>;

    async fn response_pagination(
        pool: &sqlx::PgPool,
        filter: Option<&FilterConfig<QueryableObject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType>;
}
