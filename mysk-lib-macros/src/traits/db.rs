use async_trait::async_trait;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

pub trait BaseQuery {
    fn base_query() -> &'static str;

    fn count_query() -> &'static str;
}

#[async_trait]
pub trait GetById: BaseQuery
where
    Self: Sized,
{
    async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Self, SqlxError>;

    async fn get_by_ids(pool: &PgPool, ids: Vec<Uuid>) -> Result<Vec<Self>, SqlxError>;
}
