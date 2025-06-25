use crate::{
    common::{
        pagination::{PaginationConfig, PaginationType},
        requests::{FetchLevel, FilterConfig, SortingConfig},
    },
    permissions::Authorizer,
    prelude::*,
    query::Queryable,
};
use futures::future;
use sqlx::{
    Encode, Error as SqlxError, FromRow, PgConnection, PgPool, Postgres, QueryBuilder, Row as _,
    Type as SqlxType,
    postgres::{PgHasArrayType, PgRow},
};
use std::fmt::Display;

pub trait BaseQuery {
    #[must_use]
    fn base_query() -> &'static str;

    #[must_use]
    fn count_query() -> &'static str;
}

pub trait GetById: BaseQuery + Sized {
    fn get_by_id<T>(
        conn: &mut PgConnection,
        id: T,
    ) -> impl Future<Output = Result<Self, SqlxError>>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres>;

    fn get_by_ids<T>(
        conn: &mut PgConnection,
        ids: Vec<T>,
    ) -> impl Future<Output = Result<Vec<Self>, SqlxError>>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres> + PgHasArrayType;
}

pub trait TopLevelGetById: Sized {
    fn get_by_id<T>(
        pool: &PgPool,
        id: T,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &Authorizer,
    ) -> impl Future<Output = Result<Self>>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres>;

    fn get_by_ids<T>(
        pool: &PgPool,
        ids: Vec<T>,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &Authorizer,
    ) -> impl Future<Output = Result<Vec<Self>>>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres> + PgHasArrayType;
}

/// A trait for Fetch Level Variants of a database entity with ability to convert to be converted
/// from DB variant.
pub trait FetchLevelVariant<DbVariant>: Sized {
    fn from_table(
        pool: &PgPool,
        table: DbVariant,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &Authorizer,
    ) -> impl Future<Output = Result<Self>>;
}

/// A trait for the actual database entity with ability to convert to be converted from DB variant.
pub trait TopLevelFromTable<DbVariant>: Sized {
    fn from_table(
        pool: &PgPool,
        table: DbVariant,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &Authorizer,
    ) -> impl Future<Output = Result<Self>>;
}

/// A trait for DB variant to allow querying and creating pagination response.
pub trait QueryDb<Q, S>
where
    Self: BaseQuery + for<'q> FromRow<'q, PgRow> + Send + Unpin,
    Q: Clone + Queryable,
    S: Display,
{
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<Q>>,
    );

    /// Queries the database with optional filters, sorting, and pagination. If pagination is not
    /// provided, a default configuration is used.
    fn query(
        conn: &mut PgConnection,
        filter: Option<FilterConfig<Q>>,
        sort: Option<SortingConfig<S>>,
        pagination: Option<PaginationConfig>,
    ) -> impl Future<Output = Result<(Vec<Self>, PaginationType)>> {
        async move {
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
                    .fetch_one(&mut *conn)
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
                query.build_query_as::<Self>().fetch_all(conn).await?,
                PaginationType::new(pagination.p, pagination.size.unwrap_or(50), count),
            ))
        }
    }
}

pub trait TopLevelQuery<DbVariant, Q, S>
where
    Self: TopLevelFromTable<DbVariant>,
    DbVariant: BaseQuery + QueryDb<Q, S>,
    Q: Clone + Queryable,
    S: Display,
{
    fn query(
        pool: &PgPool,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        filter: Option<FilterConfig<Q>>,
        sort: Option<SortingConfig<S>>,
        pagination: Option<PaginationConfig>,
        authorizer: &Authorizer,
    ) -> impl Future<Output = Result<(Vec<Self>, PaginationType)>> {
        async move {
            let mut conn = pool.acquire().await?;
            let (models, pagination) =
                DbVariant::query(&mut conn, filter, sort, pagination).await?;
            let futures = models
                .into_iter()
                .map(|model| {
                    Self::from_table(pool, model, fetch_level, descendant_fetch_level, authorizer)
                })
                .collect::<Vec<_>>();
            let result = future::try_join_all(futures).await?;

            Ok((result, pagination))
        }
    }
}
