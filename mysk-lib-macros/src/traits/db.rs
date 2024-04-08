use sqlx::PgPool;
use uuid::Uuid;

pub trait BaseQuery {
    fn base_query() -> &'static str;

    fn count_query() -> &'static str;
}

// only for struct with id: Uuid and implements BaseQuery
pub trait GetById: BaseQuery {
    async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Self, sqlx::Error>
    where
        Self: Sized;

    async fn get_by_ids(pool: &PgPool, ids: Vec<Uuid>) -> Result<Vec<Self>, sqlx::Error>
    where
        Self: Sized;
}
