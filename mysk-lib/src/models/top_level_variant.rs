use crate::{
    common::requests::FetchLevel,
    models::traits::{FetchLevelVariant, GetById, TopLevelFromTable, TopLevelGetById},
    permissions::Authorizer,
    prelude::*,
};
use futures::future;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::{Encode, PgPool, Postgres, Type as SqlxType, postgres::PgHasArrayType};
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

impl<DbVariant, IdOnly, Compact, Default, Detailed> TopLevelFromTable<DbVariant>
    for TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>
where
    DbVariant: GetById,
    IdOnly: Serialize + FetchLevelVariant<DbVariant>,
    Compact: Serialize + FetchLevelVariant<DbVariant>,
    Default: Serialize + FetchLevelVariant<DbVariant>,
    Detailed: Serialize + FetchLevelVariant<DbVariant>,
{
    async fn from_table(
        pool: &PgPool,
        table: DbVariant,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &Authorizer,
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

impl<DbVariant, IdOnly, Compact, Default, Detailed> TopLevelGetById
    for TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>
where
    DbVariant: GetById,
    IdOnly: Serialize + FetchLevelVariant<DbVariant>,
    Compact: Serialize + FetchLevelVariant<DbVariant>,
    Default: Serialize + FetchLevelVariant<DbVariant>,
    Detailed: Serialize + FetchLevelVariant<DbVariant>,
{
    async fn get_by_id<T>(
        pool: &PgPool,
        id: T,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &Authorizer,
    ) -> Result<Self>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres>,
    {
        let variant = DbVariant::get_by_id(&mut *(pool.acquire().await?), id).await?;

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
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &Authorizer,
    ) -> Result<Vec<Self>>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres> + PgHasArrayType,
    {
        let mut conn = pool.acquire().await?;
        let variants = DbVariant::get_by_ids(&mut conn, ids).await?;
        let futures = variants
            .into_iter()
            .map(|variant| {
                Self::from_table(
                    pool,
                    variant,
                    fetch_level,
                    descendant_fetch_level,
                    authorizer,
                )
            })
            .collect::<Vec<_>>();
        let result = future::try_join_all(futures).await?;

        Ok(result)
    }
}
