use async_trait::async_trait;
use sqlx::{
    postgres::PgHasArrayType, Acquire, Encode, Error as SqlxError, Postgres, Type as SqlxType,
};
// use uuid::Uuid;

pub trait BaseQuery {
    fn base_query() -> &'static str;

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
