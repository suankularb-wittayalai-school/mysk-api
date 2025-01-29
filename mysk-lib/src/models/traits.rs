use crate::{
    common::{
        pagination::{PaginationConfig, PaginationType},
        requests::{FetchLevel, FilterConfig, SortingConfig},
    },
    permissions::Authorizer,
    prelude::*,
    query::Queryable,
};
use async_trait::async_trait;
use sqlx::{
    postgres::{PgHasArrayType, PgRow},
    Acquire, Encode, Error as SqlxError, FromRow, PgPool, Postgres, QueryBuilder, Row as _,
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
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self>;
}

/// A trait for the actual database entity with ability to convert to be converted from DB variant.
#[async_trait]
pub trait TopLevelFromTable<DbVariant>: Sized {
    async fn from_table(
        pool: &PgPool,
        table: DbVariant,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self>;
}

#[async_trait]
pub trait TopLevelGetById: Sized {
    async fn get_by_id<T>(
        pool: &PgPool,
        id: T,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres> + Send;

    async fn get_by_ids<T>(
        pool: &PgPool,
        ids: Vec<T>,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Vec<Self>>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres> + PgHasArrayType + Send;
}

#[async_trait]
pub trait TopLevelQuery<DbVariant, Q, S>
where
    Self: TopLevelFromTable<DbVariant> + Sized + 'static,
    DbVariant: BaseQuery + QueryDb<Q, S> + Sized + Send + 'static,
    Q: Clone + Queryable + Send,
    S: Display + Send,
{
    async fn query(
        pool: &PgPool,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        filter: Option<FilterConfig<Q>>,
        sort: Option<SortingConfig<S>>,
        pagination: Option<PaginationConfig>,
        authorizer: &dyn Authorizer,
    ) -> Result<(Vec<Self>, PaginationType)>
    where
        Q: 'async_trait,
        S: 'async_trait,
    {
        let (models, pagination) = DbVariant::query(pool, filter, sort, pagination).await?;
        let futures: Vec<_> = models
            .into_iter()
            .map(|model| {
                let pool = pool.clone();
                let shared_authorizer = authorizer.clone_to_arc();

                tokio::spawn(async move {
                    Self::from_table(
                        &pool,
                        model,
                        fetch_level,
                        descendant_fetch_level,
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

        Ok((result, pagination))
    }
}

/// A trait for DB variant to allow querying and creating pagination response.
#[async_trait]
pub trait QueryDb<Q, S>
where
    Self: BaseQuery + for<'q> FromRow<'q, PgRow> + Sized + Unpin,
    Q: Clone + Queryable + Send,
    S: Display + Send,
{
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<Q>>,
    );

    /// Queries the database with optional filters, sorting, and pagination. If pagination is not
    /// provided, a default configuration is used.
    async fn query(
        pool: &PgPool,
        filter: Option<FilterConfig<Q>>,
        sort: Option<SortingConfig<S>>,
        pagination: Option<PaginationConfig>,
    ) -> Result<(Vec<Self>, PaginationType)>
    where
        Q: 'async_trait,
        S: 'async_trait,
    {
        let mut query = QueryBuilder::new(<Self as BaseQuery>::base_query());
        Self::build_shared_query(&mut query, filter.clone());

        if let Some(sorting) = sort {
            sorting.append_into_query_builder(&mut query);
        }

        let pagination = pagination.unwrap_or_default();
        pagination.append_into_query_builder(&mut query)?;

        let mut count_query = QueryBuilder::new(<Self as BaseQuery>::count_query());
        Self::build_shared_query(&mut count_query, filter);
        let count = u32::try_from(
            count_query
                .build()
                .fetch_one(pool)
                .await?
                .get::<i64, _>("count"),
        )
        .map_err(|_| {
            Error::InvalidRequest(
                "Page number is out of bounds".to_string(),
                "QueryDb::query".to_string(),
            )
        })?;

        Ok((
            query.build_query_as::<Self>().fetch_all(pool).await?,
            PaginationType::new(pagination.p, pagination.size.unwrap_or(50), count),
        ))
    }
}
