use sqlx::{Error, PgPool};
use uuid::Uuid;

pub trait BaseQuery {
    fn base_query() -> &'static str;
}

// only for struct with id: Uuid and implements BaseQuery
pub trait GetById: BaseQuery {
    fn get_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<Self, Error>> + Send
    where
        Self: Sized;

    fn get_by_ids(
        pool: &PgPool,
        ids: Vec<Uuid>,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, Error>> + Send
    where
        Self: Sized;
}
