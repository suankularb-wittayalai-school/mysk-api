use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use super::requests::{FetchLevel, FilterConfig, PaginationConfig, SortingConfig};

#[async_trait]
pub trait DBEntity {
    async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Self, Error>
    where
        Self: Sized;

    fn get_default_query() -> String;
    fn get_count_query() -> String;

    // T is the queryable variant of the entity
    // return the number of params counted in the query
    fn append_where_clause<T>(
        query: &mut String,
        filter: &FilterConfig<T>,
        param_counted: u64,
    ) -> u64;

    fn append_order_by_clause<T>(
        query: &mut String,
        sorting: &SortingConfig<T>,
        param_counted: u64,
    ) -> u64; // T is the sortable enum of the entity

    fn append_pagination_clause(
        query: &mut String,
        pagination: &Option<PaginationConfig>,
        param_counted: u64,
    ) -> u64; // default is 50

    fn construct_query<Queryable, Sortable>(
        filter: &Option<FilterConfig<Queryable>>,
        sorting: &Option<SortingConfig<Sortable>>,
        pagination: &Option<PaginationConfig>,
    ) -> String
    where
        Queryable: std::fmt::Display,
        Sortable: std::fmt::Display,
    {
        let mut query = Self::get_default_query();
        let mut param_counted = 0;

        if let Some(filter) = filter {
            param_counted = Self::append_where_clause(&mut query, filter, param_counted);
        }
        if let Some(sorting) = sorting {
            param_counted = Self::append_order_by_clause(&mut query, sorting, param_counted);
        }

        Self::append_pagination_clause(&mut query, pagination, param_counted);

        query
    }

    async fn query<Queryable, Sortable>(
        pool: &PgPool,
        filter: &FilterConfig<Queryable>,
        sorting: &SortingConfig<Sortable>,
        pagination: &Option<PaginationConfig>,
    ) -> Result<Vec<Self>, Error>
    where
        Self: Sized;

    async fn get_pagination_meta<Queryable>(
        pool: &PgPool,
        filter: &FilterConfig<Queryable>,
        pagination: &Option<PaginationConfig>,
    ) -> Result<u64, Error>;

    async fn update_by_id<T>(pool: &PgPool, id: Uuid, entity: &T) -> Result<Self, Error>
    where
        Self: Sized;

    async fn delete_by_id(pool: &PgPool, id: Uuid) -> Result<Self, Error>
    where
        Self: Sized;

    async fn create<T>(pool: &PgPool, entity: &T) -> Result<Self, Error>
    // T is the creatable variant of the entity
    where
        Self: Sized;
}

#[async_trait]
pub trait FetchLevelVariant {
    async fn from_table(
        table: impl DBEntity,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

#[async_trait]
pub trait CompositeEntity {
    async fn from_table(
        pool: &PgPool,
        table: impl DBEntity,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, Error>
    where
        Self: Sized;

    async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, Error>
    where
        Self: Sized;

    async fn query<Queryable, Sortable>(
        pool: &PgPool,
        filter: &FilterConfig<Queryable>,
        sorting: &SortingConfig<Sortable>,
        pagination: &Option<PaginationConfig>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>, Error>
    where
        Self: Sized;

    async fn get_pagination_meta<Queryable>(
        pool: &PgPool,
        filter: &FilterConfig<Queryable>,
        pagination: &Option<PaginationConfig>,
    ) -> Result<u64, Error>;

    async fn update_by_id<T>(
        pool: &PgPool,
        id: Uuid,
        entity: &T,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, Error>
    where
        Self: Sized;

    async fn delete_by_id(
        pool: &PgPool,
        id: Uuid,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, Error>
    where
        Self: Sized;

    async fn create<T>(
        pool: &PgPool,
        entity: &T,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}
