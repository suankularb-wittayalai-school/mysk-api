use async_trait::async_trait;
use sqlx::{Acquire, Error as SqlxError, Postgres};
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
    async fn get_by_id<'a, A: Send>(conn: A, id: Uuid) -> Result<Self, SqlxError>
    where
        A: Acquire<'a, Database = Postgres>;

    async fn get_by_ids<'a, A: Send>(conn: A, ids: Vec<Uuid>) -> Result<Vec<Self>, SqlxError>
    where
        A: Acquire<'a, Database = Postgres>;
}
