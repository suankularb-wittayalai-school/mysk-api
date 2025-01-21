use crate::{
    common::{
        requests::{FetchLevel, FilterConfig, PaginationConfig, SortingConfig, SqlSection},
        response::PaginationType,
    },
    permissions::Authorizer,
    prelude::*,
};
use async_trait::async_trait;
use sqlx::{
    postgres::PgHasArrayType, Acquire, Encode, Error as SqlxError, PgPool, Postgres, QueryBuilder,
    Type as SqlxType,
};
use std::fmt::Display;

pub trait BaseQuery {
    #[must_use]
    fn base_query() -> &'static str;

    #[must_use]
    fn count_query() -> &'static str;
}

#[async_trait]
pub trait GetById: BaseQuery + Sized {
    async fn get_by_id<'c, A, T>(conn: A, id: T) -> Result<Self, SqlxError>
    where
        A: Acquire<'c, Database = Postgres> + Send,
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres> + Send;

    async fn get_by_ids<'c, A, T>(conn: A, ids: Vec<T>) -> Result<Vec<Self>, SqlxError>
    where
        A: Acquire<'c, Database = Postgres> + Send,
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres> + PgHasArrayType + Send;
}

/// A trait for Fetch Level Variants of a database entity with ability to convert to be converted
/// from DB variant.
#[async_trait]
pub trait FetchLevelVariant<DbVariant>: Sized {
    async fn from_table(
        pool: &PgPool,
        table: DbVariant,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self>;
}

/// A trait for the actual database entity with ability to convert to be converted from DB variant.
#[async_trait]
pub trait TopLevelFromTable<DbVariant>: Sized {
    async fn from_table(
        pool: &PgPool,
        table: DbVariant,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self>;
}

#[async_trait]
pub trait TopLevelGetById: Sized {
    async fn get_by_id<T>(
        pool: &PgPool,
        id: T,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres> + Send;

    async fn get_by_ids<T>(
        pool: &PgPool,
        ids: Vec<T>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Vec<Self>>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres> + PgHasArrayType + Send;
}

#[async_trait]
pub trait TopLevelQuery<DbVariant, QueryableObject, SortableObject>
where
    Self: TopLevelFromTable<DbVariant> + Sized + 'static,
    DbVariant: BaseQuery + QueryDb<QueryableObject, SortableObject> + Sized + Send + 'static,
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
        authorizer: &dyn Authorizer,
    ) -> Result<Vec<Self>> {
        let models = DbVariant::query(pool, filter, sort, pagination).await?;
        let fetch_level = fetch_level.copied();
        let descendant_fetch_level = descendant_fetch_level.copied();
        let futures: Vec<_> = models
            .into_iter()
            .map(|model| {
                let pool = pool.clone();
                let shared_authorizer = authorizer.clone_to_arc();

                tokio::spawn(async move {
                    Self::from_table(
                        &pool,
                        model,
                        fetch_level.as_ref(),
                        descendant_fetch_level.as_ref(),
                        &*shared_authorizer,
                    )
                    .await
                })
            })
            .collect();

        let mut result = Vec::with_capacity(futures.len());
        for future in futures {
            result.push(future.await??);
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
pub trait QueryDb<QueryableObject: Queryable, SortableObject: Display>: BaseQuery + Sized {
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
