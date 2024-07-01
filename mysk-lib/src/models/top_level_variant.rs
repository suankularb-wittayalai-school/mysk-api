use crate::{
    common::{requests::FetchLevel, response::ResponseType},
    models::traits::{FetchLevelVariant, TopLevelFromTable, TopLevelGetById},
    prelude::*,
};
use actix_web::{HttpResponse, Responder};
use async_trait::async_trait;
use mysk_lib_macros::traits::db::GetById;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::{Error as SqlxError, PgPool};
use std::marker::PhantomData;
use uuid::Uuid;

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
    ) -> Result<Self> {
        match fetch_level {
            Some(FetchLevel::IdOnly) | None => Ok(Self::IdOnly(
                // We don't need to return a pinned box because IdOnly is never recursive
                Box::new(IdOnly::from_table(pool, table, descendant_fetch_level).await?),
                PhantomData,
            )),
            Some(FetchLevel::Compact) => Ok(Self::Compact(
                Box::new(Box::pin(Compact::from_table(pool, table, descendant_fetch_level)).await?),
                PhantomData,
            )),
            Some(FetchLevel::Default) => Ok(Self::Default(
                Box::new(Box::pin(Default::from_table(pool, table, descendant_fetch_level)).await?),
                PhantomData,
            )),
            Some(FetchLevel::Detailed) => Ok(Self::Detailed(
                Box::new(
                    Box::pin(Detailed::from_table(pool, table, descendant_fetch_level)).await?,
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
    fn serialize<S: Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error> {
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
    async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        let variant = match DbVariant::get_by_id(pool, id).await {
            Ok(variant) => variant,
            Err(e) => {
                return Err(match e {
                    SqlxError::Database(source) => Error::InvalidRequest(
                        source.message().to_string(),
                        "TopLevelGetById::get_by_id".to_string(),
                    ),
                    SqlxError::RowNotFound => Error::EntityNotFound(
                        "Entity not found".to_string(),
                        "TopLevelGetById::get_by_id".to_string(),
                    ),
                    _ => Error::InternalSeverError(
                        "Internal server error".to_string(),
                        "TopLevelGetById::get_by_id".to_string(),
                    ),
                });
            }
        };

        Self::from_table(pool, variant, fetch_level, descendant_fetch_level).await
    }

    async fn get_by_ids(
        pool: &PgPool,
        ids: Vec<Uuid>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>> {
        let variants = match DbVariant::get_by_ids(pool, ids).await {
            Ok(variants) => variants,
            Err(e) => {
                return Err(match e {
                    SqlxError::Database(source) => Error::InvalidRequest(
                        source.message().to_string(),
                        "TopLevelGetById::get_by_ids".to_string(),
                    ),
                    SqlxError::RowNotFound => Error::EntityNotFound(
                        "Entity not found".to_string(),
                        "TopLevelGetById::get_by_ids".to_string(),
                    ),
                    _ => Error::InternalSeverError(
                        "Internal server error".to_string(),
                        "TopLevelGetById::get_by_ids".to_string(),
                    ),
                });
            }
        };
        let fetch_level = fetch_level.copied();
        let descendant_fetch_level = descendant_fetch_level.copied();
        let futures: Vec<_> = variants
            .into_iter()
            .map(|variant| {
                let pool = pool.clone();

                tokio::spawn(async move {
                    Self::from_table(
                        &pool,
                        variant,
                        fetch_level.as_ref(),
                        descendant_fetch_level.as_ref(),
                    )
                    .await
                })
            })
            .collect();

        let mut result = Vec::with_capacity(futures.len());
        for future in futures {
            result.push(future.await.unwrap()?);
        }

        Ok(result)
    }
}

impl<DbVariant, IdOnly, Compact, Default, Detailed>
    From<TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>>
    for ResponseType<TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>>
where
    DbVariant: GetById,
    IdOnly: Serialize + FetchLevelVariant<DbVariant>,
    Compact: Serialize + FetchLevelVariant<DbVariant>,
    Default: Serialize + FetchLevelVariant<DbVariant>,
    Detailed: Serialize + FetchLevelVariant<DbVariant>,
{
    fn from(variant: TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>) -> Self {
        ResponseType::new(variant, None)
    }
}

impl<DbVariant, IdOnly, Compact, Default, Detailed>
    From<TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>> for HttpResponse
where
    DbVariant: GetById,
    IdOnly: Serialize + FetchLevelVariant<DbVariant>,
    Compact: Serialize + FetchLevelVariant<DbVariant>,
    Default: Serialize + FetchLevelVariant<DbVariant>,
    Detailed: Serialize + FetchLevelVariant<DbVariant>,
{
    fn from(variant: TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>) -> Self {
        let response_type: ResponseType<
            TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>,
        > = variant.into();

        HttpResponse::Ok().json(response_type)
    }
}

impl<DbVariant, IdOnly, Compact, Default, Detailed>
    From<TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>> for Result<HttpResponse>
where
    DbVariant: GetById,
    IdOnly: Serialize + FetchLevelVariant<DbVariant>,
    Compact: Serialize + FetchLevelVariant<DbVariant>,
    Default: Serialize + FetchLevelVariant<DbVariant>,
    Detailed: Serialize + FetchLevelVariant<DbVariant>,
{
    fn from(variant: TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>) -> Self {
        let response_type: ResponseType<
            TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>,
        > = variant.into();

        Ok(HttpResponse::Ok().json(response_type))
    }
}

impl<DbVariant, IdOnly, Compact, Default, Detailed> Responder
    for TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>
where
    DbVariant: GetById,
    IdOnly: Serialize + FetchLevelVariant<DbVariant>,
    Compact: Serialize + FetchLevelVariant<DbVariant>,
    Default: Serialize + FetchLevelVariant<DbVariant>,
    Detailed: Serialize + FetchLevelVariant<DbVariant>,
{
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        let response_type: ResponseType<
            TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>,
        > = self.into();

        HttpResponse::Ok().json(response_type)
    }
}
