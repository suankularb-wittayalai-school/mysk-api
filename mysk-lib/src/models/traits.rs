use crate::{
    common::{
        pagination::{PaginationConfig, PaginationType},
        requests::{FetchLevel, FilterConfig, SortingConfig},
    },
    permissions::Authorizer,
    prelude::*,
    query::Queryable,
};
use sqlx::{
    Encode, Error as SqlxError, FromRow, PgConnection, PgPool, Postgres, QueryBuilder, Row as _,
    Type as SqlxType,
    postgres::{PgHasArrayType, PgRow},
};
use std::fmt::Display;

/// Get data from relations by its' ID via a base query.
pub trait GetById: for<'r> FromRow<'r, PgRow> + Sized {
    /// The base query of the relation.
    const BASE_QUERY: &'static str;

    /// The count query of the relation, it is used in the [`QueryRelation`] trait.
    const COUNT_QUERY: &'static str;

    /// The type corresponding to the ID or primary key of the relation.
    type Id: for<'q> Encode<'q, Postgres> + SqlxType<Postgres> + PgHasArrayType;

    /// Gets a single row of the relation by ID.
    fn get_by_id(
        conn: &mut PgConnection,
        id: Self::Id,
    ) -> impl Future<Output = Result<Self, SqlxError>>;

    /// Get multiple rows of the relation by IDs.
    fn get_by_ids(
        conn: &mut PgConnection,
        ids: &[Self::Id],
    ) -> impl Future<Output = Result<Vec<Self>, SqlxError>>;
}

/// A fetch variant is a data model that can be derived from a base relation.
pub trait FetchVariant: Sized {
    /// The base relation for this fetch variant.
    type Relation: GetById;

    /// Converts to this fetch variant from the base relation and any additional dependencies.
    fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> impl Future<Output = Result<Self>>;
}

/// Query data using complex conditions and predicates from relations.
pub trait QueryRelation: for<'r> FromRow<'r, PgRow> + GetById + Send + Unpin {
    /// The query configuration object.
    type Q: Clone + Queryable<Relation = Self>;

    /// The columns to sort by described as variants in an enum.
    type S: Display;

    /// Builds a shared query with applicable filters and sorting rules to be used for data fetching
    /// and count fetching.
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<Self::Q>>,
    );

    /// Queries the database with optional filters, sorting, and pagination. If pagination is not
    /// provided, a default configuration is used.
    fn query(
        conn: &mut PgConnection,
        filter: Option<FilterConfig<Self::Q>>,
        sort: Option<SortingConfig<Self::S>>,
        pagination: Option<PaginationConfig>,
    ) -> impl Future<Output = Result<(Vec<Self>, PaginationType)>> {
        async move {
            let mut query = QueryBuilder::new(<Self as GetById>::BASE_QUERY);
            Self::build_shared_query(&mut query, filter.clone());

            if let Some(sorting) = sort {
                sorting.append_into_query_builder(&mut query);
            }

            let pagination = pagination.unwrap_or_default();
            pagination.append_into_query_builder(&mut query)?;

            let mut count_query = QueryBuilder::new(<Self as GetById>::COUNT_QUERY);
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
                    "QueryRelation::query".to_string(),
                )
            })?;

            Ok((
                query.build_query_as::<Self>().fetch_all(conn).await?,
                PaginationType::new(pagination.p, pagination.size.unwrap_or(50), count),
            ))
        }
    }
}
