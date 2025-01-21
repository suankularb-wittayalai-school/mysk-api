use crate::{
    common::requests::FetchLevel,
    models::traits::{FetchLevelVariant, TopLevelFromTable, TopLevelGetById},
    permissions::Authorizer,
    prelude::*,
};
use async_trait::async_trait;
use mysk_lib_macros::traits::db::GetById;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::{postgres::PgHasArrayType, Encode, PgPool, Postgres, Type as SqlxType};
use std::marker::PhantomData;

#[derive(Clone, Debug, Deserialize)]
pub enum TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>
where
    DbVariant: GetById,
    IdOnly: Serialize + FetchLevelVariant<DbVariant>,
    Compact: Serialize + FetchLevelVariant<DbVariant>,
    Default: Serialize + FetchLevelVariant<DbVariant>,
    Detailed: Serialize + FetchLevelVariant<DbVariant>,
{
    IdOnly(Box<IdOnly>, PhantomData<DbVariant>),
    Compact(Box<Compact>, PhantomData<DbVariant>),
    Default(Box<Default>, PhantomData<DbVariant>),
    Detailed(Box<Detailed>, PhantomData<DbVariant>),
}

#[async_trait]
impl<DbVariant, IdOnly, Compact, Default, Detailed> TopLevelFromTable<DbVariant>
    for TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>
where
    DbVariant: GetById + Send,
    IdOnly: Serialize + FetchLevelVariant<DbVariant> + Send,
    Compact: Serialize + FetchLevelVariant<DbVariant> + Send,
    Default: Serialize + FetchLevelVariant<DbVariant> + Send,
    Detailed: Serialize + FetchLevelVariant<DbVariant> + Send,
{
    async fn from_table(
        pool: &PgPool,
        table: DbVariant,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        match fetch_level {
            Some(FetchLevel::IdOnly) | None => Ok(Self::IdOnly(
                // We don't need to return a pinned box because IdOnly is never recursive
                Box::new(
                    IdOnly::from_table(pool, table, descendant_fetch_level, authorizer).await?,
                ),
                PhantomData,
            )),
            Some(FetchLevel::Compact) => Ok(Self::Compact(
                Box::new(
                    Box::pin(Compact::from_table(
                        pool,
                        table,
                        descendant_fetch_level,
                        authorizer,
                    ))
                    .await?,
                ),
                PhantomData,
            )),
            Some(FetchLevel::Default) => Ok(Self::Default(
                Box::new(
                    Box::pin(Default::from_table(
                        pool,
                        table,
                        descendant_fetch_level,
                        authorizer,
                    ))
                    .await?,
                ),
                PhantomData,
            )),
            Some(FetchLevel::Detailed) => Ok(Self::Detailed(
                Box::new(
                    Box::pin(Detailed::from_table(
                        pool,
                        table,
                        descendant_fetch_level,
                        authorizer,
                    ))
                    .await?,
                ),
                PhantomData,
            )),
        }
    }
}

#[async_trait]
impl<DbVariant, IdOnly, Compact, Default, Detailed> Serialize
    for TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>
where
    DbVariant: GetById,
    IdOnly: Serialize + FetchLevelVariant<DbVariant>,
    Compact: Serialize + FetchLevelVariant<DbVariant>,
    Default: Serialize + FetchLevelVariant<DbVariant>,
    Detailed: Serialize + FetchLevelVariant<DbVariant>,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match self {
            TopLevelVariant::IdOnly(variant, _) => variant.serialize(serializer),
            TopLevelVariant::Compact(variant, _) => variant.serialize(serializer),
            TopLevelVariant::Default(variant, _) => variant.serialize(serializer),
            TopLevelVariant::Detailed(variant, _) => variant.serialize(serializer),
        }
    }
}

#[async_trait]
impl<DbVariant, IdOnly, Compact, Default, Detailed> TopLevelGetById
    for TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>
where
    DbVariant: GetById + Send + 'static,
    IdOnly: Serialize + FetchLevelVariant<DbVariant> + Send + 'static,
    Compact: Serialize + FetchLevelVariant<DbVariant> + Send + 'static,
    Default: Serialize + FetchLevelVariant<DbVariant> + Send + 'static,
    Detailed: Serialize + FetchLevelVariant<DbVariant> + Send + 'static,
{
    async fn get_by_id<T>(
        pool: &PgPool,
        id: T,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres> + Send,
    {
        let variant = DbVariant::get_by_id(pool, id).await?;

        Self::from_table(
            pool,
            variant,
            fetch_level,
            descendant_fetch_level,
            authorizer,
        )
        .await
    }

    async fn get_by_ids<T>(
        pool: &PgPool,
        ids: Vec<T>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Vec<Self>>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres> + PgHasArrayType + Send,
    {
        let variants = DbVariant::get_by_ids(pool, ids).await?;
        let fetch_level = fetch_level.copied();
        let descendant_fetch_level = descendant_fetch_level.copied();
        let futures: Vec<_> = variants
            .into_iter()
            .map(|variant| {
                let pool = pool.clone();
                let shared_authorizer = authorizer.clone_to_arc();

                tokio::spawn(async move {
                    Self::from_table(
                        &pool,
                        variant,
                        fetch_level.as_ref(),
                        descendant_fetch_level.as_ref(),
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

        Ok(result)
    }
}
