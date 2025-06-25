use crate::{
    common::{
        PaginationConfig, PaginationType,
        requests::{FetchLevel, FilterConfig, SortablePlaceholder, SortingConfig},
    },
    models::traits::{FetchLevelVariant, GetById, QueryDb},
    permissions::Authorizer,
    prelude::*,
    query::{Queryable, QueryablePlaceholder},
};
use futures::future;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::{Encode, PgPool, Postgres, Type as SqlxType, postgres::PgHasArrayType};
use std::{fmt::Display, marker::PhantomData};

#[derive(Clone, Debug, Deserialize)]
pub enum TopLevelVariant<
    Table,
    IdOnly,
    Compact,
    Default,
    Detailed,
    Q = QueryablePlaceholder,
    S = SortablePlaceholder,
> {
    IdOnly(Box<IdOnly>, PhantomData<(Table, Q, S)>),
    Compact(Box<Compact>, PhantomData<(Table, Q, S)>),
    Default(Box<Default>, PhantomData<(Table, Q, S)>),
    Detailed(Box<Detailed>, PhantomData<(Table, Q, S)>),
}

impl<Table, IdOnly, Compact, Default, Detailed, Q, S>
    TopLevelVariant<Table, IdOnly, Compact, Default, Detailed, Q, S>
where
    Table: GetById,
    IdOnly: FetchLevelVariant<Table>,
    Compact: FetchLevelVariant<Table>,
    Default: FetchLevelVariant<Table>,
    Detailed: FetchLevelVariant<Table>,
{
    pub async fn from_table(
        pool: &PgPool,
        table: Table,
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

    pub async fn get_by_id<T>(
        pool: &PgPool,
        id: T,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &Authorizer,
    ) -> Result<Self>
    where
        T: for<'q> Encode<'q, Postgres> + SqlxType<Postgres>,
    {
        let variant = Table::get_by_id(&mut *(pool.acquire().await?), id).await?;

        Self::from_table(
            pool,
            variant,
            fetch_level,
            descendant_fetch_level,
            authorizer,
        )
        .await
    }

    pub async fn get_by_ids<T>(
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
        let variants = Table::get_by_ids(&mut conn, ids).await?;
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

impl<Table, IdOnly, Compact, Default, Detailed, Q, S>
    TopLevelVariant<Table, IdOnly, Compact, Default, Detailed, Q, S>
where
    Table: GetById + QueryDb<Q, S>,
    IdOnly: FetchLevelVariant<Table>,
    Compact: FetchLevelVariant<Table>,
    Default: FetchLevelVariant<Table>,
    Detailed: FetchLevelVariant<Table>,
    Q: Clone + Queryable,
    S: Display,
{
    pub async fn query(
        pool: &PgPool,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
        filter: Option<FilterConfig<Q>>,
        sort: Option<SortingConfig<S>>,
        pagination: Option<PaginationConfig>,
        authorizer: &Authorizer,
    ) -> Result<(Vec<Self>, PaginationType)> {
        let mut conn = pool.acquire().await?;
        let (models, pagination) = Table::query(&mut conn, filter, sort, pagination).await?;
        let futures = models
            .into_iter()
            .map(|model| {
                Self::from_table(pool, model, fetch_level, descendant_fetch_level, authorizer)
            })
            .collect::<Vec<_>>();
        let result = future::try_join_all(futures).await?;

        Ok((result, pagination))
    }
}

impl<Table, IdOnly, Compact, Default, Detailed, Q, S> Serialize
    for TopLevelVariant<Table, IdOnly, Compact, Default, Detailed, Q, S>
where
    IdOnly: Serialize,
    Compact: Serialize,
    Default: Serialize,
    Detailed: Serialize,
{
    fn serialize<Ser: Serializer>(
        &self,
        serializer: Ser,
    ) -> std::result::Result<Ser::Ok, Ser::Error> {
        match self {
            TopLevelVariant::IdOnly(variant, _) => variant.serialize(serializer),
            TopLevelVariant::Compact(variant, _) => variant.serialize(serializer),
            TopLevelVariant::Default(variant, _) => variant.serialize(serializer),
            TopLevelVariant::Detailed(variant, _) => variant.serialize(serializer),
        }
    }
}
