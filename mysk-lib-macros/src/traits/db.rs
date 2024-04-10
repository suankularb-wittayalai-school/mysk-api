use std::future::Future;

use sqlx::PgPool;
use uuid::Uuid;

pub trait BaseQuery {
    fn base_query() -> &'static str;

    fn count_query() -> &'static str;
}

// only for struct with id: Uuid and implements BaseQuery
pub trait GetById: BaseQuery {
    fn get_by_id(pool: &PgPool, id: Uuid) -> impl Future<Output = Result<Self, sqlx::Error>>
    where
        Self: Sized;

    fn get_by_ids(
        pool: &PgPool,
        ids: Vec<Uuid>,
    ) -> impl Future<Output = Result<Vec<Self>, sqlx::Error>>
    where
        Self: Sized;
}
