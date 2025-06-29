use crate::{
    common::{
        PaginationConfig, PaginationType,
        requests::{FetchLevel, FilterConfig, SortingConfig},
    },
    models::traits::{FetchVariant, GetById, QueryRelation},
    permissions::Authorizer,
    prelude::*,
};
use futures::future;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::PgPool;
use std::marker::PhantomData;

/// Data model used for every user-facing API response. A data model for a relation can exist as one
/// of four variants.
#[derive(Clone, Debug, Deserialize)]
pub enum Model<R, Io, Co, Df, Dt> {
    IdOnly(Box<Io>, PhantomData<R>),
    Compact(Box<Co>, PhantomData<R>),
    Default(Box<Df>, PhantomData<R>),
    Detailed(Box<Dt>, PhantomData<R>),
}

impl<R: GetById, Io, Co, Df, Dt> Model<R, Io, Co, Df, Dt>
where
    Io: FetchVariant<Relation = R>,
    Co: FetchVariant<Relation = R>,
    Df: FetchVariant<Relation = R>,
    Dt: FetchVariant<Relation = R>,
{
    /// Gets a single row of the model by ID.
    pub async fn get_by_id(
        pool: &PgPool,
        id: <R as GetById>::Id,
        fetch_level: FetchLevel,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let variant = R::get_by_id(&mut *(pool.acquire().await?), id).await?;

        Self::from_variant(
            pool,
            variant,
            fetch_level,
            descendant_fetch_level,
            authorizer,
        )
        .await
    }

    /// Get multiple rows of the model by IDs.
    pub async fn get_by_ids(
        pool: &PgPool,
        ids: &[<R as GetById>::Id],
        fetch_level: FetchLevel,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Vec<Self>> {
        let variants = R::get_by_ids(&mut *(pool.acquire().await?), ids).await?;
        let futures = variants
            .into_iter()
            .map(|variant| {
                Self::from_variant(
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

    async fn from_variant(
        pool: &PgPool,
        relation: R,
        fetch_level: FetchLevel,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        // Implementation detail:
        // If the underlying type (fetch variant) is cyclic/recursive, then Rust requires that the
        // future must be pinned in-place. Since we won't know which types are cyclic/recursive, we
        // pin everything except for `IdOnly`.
        match fetch_level {
            FetchLevel::IdOnly => Ok(Self::IdOnly(
                Box::new(
                    Io::from_relation(pool, relation, descendant_fetch_level, authorizer).await?,
                ),
                PhantomData,
            )),
            FetchLevel::Compact => Ok(Self::Compact(
                Box::new(
                    Box::pin(Co::from_relation(
                        pool,
                        relation,
                        descendant_fetch_level,
                        authorizer,
                    ))
                    .await?,
                ),
                PhantomData,
            )),
            FetchLevel::Default => Ok(Self::Default(
                Box::new(
                    Box::pin(Df::from_relation(
                        pool,
                        relation,
                        descendant_fetch_level,
                        authorizer,
                    ))
                    .await?,
                ),
                PhantomData,
            )),
            FetchLevel::Detailed => Ok(Self::Detailed(
                Box::new(
                    Box::pin(Dt::from_relation(
                        pool,
                        relation,
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

impl<R: GetById + QueryRelation, Io, Co, Df, Dt> Model<R, Io, Co, Df, Dt>
where
    Io: FetchVariant<Relation = R>,
    Co: FetchVariant<Relation = R>,
    Df: FetchVariant<Relation = R>,
    Dt: FetchVariant<Relation = R>,
{
    /// Queries the database with optional filters, sorting, and pagination. If pagination is not
    /// provided, a default configuration is used.
    pub async fn query(
        pool: &PgPool,
        fetch_level: FetchLevel,
        descendant_fetch_level: FetchLevel,
        filter: Option<FilterConfig<<R as QueryRelation>::Q>>,
        sort: Option<SortingConfig<<R as QueryRelation>::S>>,
        pagination: Option<PaginationConfig>,
        authorizer: &Authorizer,
    ) -> Result<(Vec<Self>, PaginationType)> {
        let mut conn = pool.acquire().await?;
        let (models, pagination) = R::query(&mut conn, filter, sort, pagination).await?;
        let futures = models
            .into_iter()
            .map(|model| {
                Self::from_variant(pool, model, fetch_level, descendant_fetch_level, authorizer)
            })
            .collect::<Vec<_>>();
        let result = future::try_join_all(futures).await?;

        Ok((result, pagination))
    }
}

impl<R, Io, Co, Df, Dt> Serialize for Model<R, Io, Co, Df, Dt>
where
    Io: Serialize,
    Co: Serialize,
    Df: Serialize,
    Dt: Serialize,
{
    fn serialize<Ser: Serializer>(
        &self,
        serializer: Ser,
    ) -> std::result::Result<Ser::Ok, Ser::Error> {
        match self {
            Self::IdOnly(variant, _) => variant.serialize(serializer),
            Self::Compact(variant, _) => variant.serialize(serializer),
            Self::Default(variant, _) => variant.serialize(serializer),
            Self::Detailed(variant, _) => variant.serialize(serializer),
        }
    }
}
