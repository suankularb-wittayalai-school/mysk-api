use std::marker::PhantomData;

use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use mysk_lib_macros::traits::db::GetById;

use super::{
    requests::FetchLevel,
    response::ResponseType,
    traits::{FetchLevelVariant, TopLevelFromTable, TopLevelGetById},
};

#[derive(Clone, Debug, Deserialize)]
pub enum TopLevelVariant<
    DbVariant: GetById,
    IdOnly: Serialize + FetchLevelVariant<DbVariant>,
    Compact: Serialize + FetchLevelVariant<DbVariant>,
    Default: Serialize + FetchLevelVariant<DbVariant>,
    Detailed: Serialize + FetchLevelVariant<DbVariant>,
> {
    IdOnly(Box<IdOnly>, PhantomData<DbVariant>),
    Compact(Box<Compact>, PhantomData<DbVariant>),
    Default(Box<Default>, PhantomData<DbVariant>),
    Detailed(Box<Detailed>, PhantomData<DbVariant>),
}

impl<
        DbVariant: GetById,
        IdOnly: Serialize + FetchLevelVariant<DbVariant>,
        Compact: Serialize + FetchLevelVariant<DbVariant>,
        Default: Serialize + FetchLevelVariant<DbVariant>,
        Detailed: Serialize + FetchLevelVariant<DbVariant>,
    > TopLevelFromTable<DbVariant>
    for TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>
where
    IdOnly: Serialize + FetchLevelVariant<DbVariant>,
    Compact: Serialize + FetchLevelVariant<DbVariant>,
    Default: Serialize + FetchLevelVariant<DbVariant>,
    Detailed: Serialize + FetchLevelVariant<DbVariant>,
{
    async fn from_table(
        pool: &sqlx::pool::Pool<sqlx::Postgres>,
        table: DbVariant,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        match fetch_level {
            Some(FetchLevel::IdOnly) => Ok(Self::IdOnly(
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
            None => Ok(Self::IdOnly(
                Box::new(IdOnly::from_table(pool, table, descendant_fetch_level).await?),
                PhantomData,
            )),
        }
    }
}

impl<
        DbVariant: GetById,
        IdOnly: Serialize + FetchLevelVariant<DbVariant>,
        Compact: Serialize + FetchLevelVariant<DbVariant>,
        Default: Serialize + FetchLevelVariant<DbVariant>,
        Detailed: Serialize + FetchLevelVariant<DbVariant>,
    > Serialize for TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>
where
    IdOnly: Serialize + FetchLevelVariant<DbVariant>,
    Compact: Serialize + FetchLevelVariant<DbVariant>,
    Default: Serialize + FetchLevelVariant<DbVariant>,
    Detailed: Serialize + FetchLevelVariant<DbVariant>,
{
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> {
        match self {
            TopLevelVariant::IdOnly(variant, _) => variant.serialize(serializer),
            TopLevelVariant::Compact(variant, _) => variant.serialize(serializer),
            TopLevelVariant::Default(variant, _) => variant.serialize(serializer),
            TopLevelVariant::Detailed(variant, _) => variant.serialize(serializer),
        }
    }
}

impl<
        DbVariant: GetById,
        IdOnly: Serialize + FetchLevelVariant<DbVariant>,
        Compact: Serialize + FetchLevelVariant<DbVariant>,
        Default: Serialize + FetchLevelVariant<DbVariant>,
        Detailed: Serialize + FetchLevelVariant<DbVariant>,
    > TopLevelGetById for TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>
where
    IdOnly: Serialize + FetchLevelVariant<DbVariant>,
    Compact: Serialize + FetchLevelVariant<DbVariant>,
    Default: Serialize + FetchLevelVariant<DbVariant>,
    Detailed: Serialize + FetchLevelVariant<DbVariant>,
{
    async fn get_by_id(
        pool: &sqlx::pool::Pool<sqlx::Postgres>,
        id: uuid::Uuid,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        let variant = DbVariant::get_by_id(pool, id).await;

        let variant = match variant {
            Ok(variant) => variant,
            Err(e) => {
                return Err(Error::InternalSeverError(
                    e.to_string(),
                    "TopLevelGetById::get_by_id".to_string(),
                ))
            }
        };

        Self::from_table(pool, variant, fetch_level, descendant_fetch_level).await
    }

    async fn get_by_ids(
        pool: &sqlx::pool::Pool<sqlx::Postgres>,
        ids: Vec<uuid::Uuid>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>> {
        let variants = DbVariant::get_by_ids(pool, ids).await;

        let variants = match variants {
            Ok(variants) => variants,
            Err(e) => {
                return Err(Error::InternalSeverError(
                    e.to_string(),
                    "TopLevelGetById::get_by_ids".to_string(),
                ))
            }
        };

        let mut result = vec![];

        for variant in variants {
            result
                .push(Self::from_table(pool, variant, fetch_level, descendant_fetch_level).await?);
        }

        Ok(result)
    }
}

impl<
        DbVariant: GetById,
        IdOnly: Serialize + FetchLevelVariant<DbVariant>,
        Compact: Serialize + FetchLevelVariant<DbVariant>,
        Default: Serialize + FetchLevelVariant<DbVariant>,
        Detailed: Serialize + FetchLevelVariant<DbVariant>,
    > From<TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>>
    for ResponseType<TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>>
{
    fn from(variant: TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>) -> Self {
        ResponseType::new(variant, None)
    }
}

impl<
        DbVariant: GetById,
        IdOnly: Serialize + FetchLevelVariant<DbVariant>,
        Compact: Serialize + FetchLevelVariant<DbVariant>,
        Default: Serialize + FetchLevelVariant<DbVariant>,
        Detailed: Serialize + FetchLevelVariant<DbVariant>,
    > From<TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>> for HttpResponse
{
    fn from(variant: TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>) -> Self {
        let response_type: ResponseType<
            TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>,
        > = variant.into();

        HttpResponse::Ok().json(response_type)
    }
}

impl<
        DbVariant: GetById,
        IdOnly: Serialize + FetchLevelVariant<DbVariant>,
        Compact: Serialize + FetchLevelVariant<DbVariant>,
        Default: Serialize + FetchLevelVariant<DbVariant>,
        Detailed: Serialize + FetchLevelVariant<DbVariant>,
    > From<TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>>
    for Result<HttpResponse>
{
    fn from(variant: TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>) -> Self {
        let response_type: ResponseType<
            TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>,
        > = variant.into();

        Ok(HttpResponse::Ok().json(response_type))
    }
}

impl<
        DbVariant: GetById,
        IdOnly: Serialize + FetchLevelVariant<DbVariant>,
        Compact: Serialize + FetchLevelVariant<DbVariant>,
        Default: Serialize + FetchLevelVariant<DbVariant>,
        Detailed: Serialize + FetchLevelVariant<DbVariant>,
    > Responder for TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>
{
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        let response_type: ResponseType<
            TopLevelVariant<DbVariant, IdOnly, Compact, Default, Detailed>,
        > = self.into();

        HttpResponse::Ok().json(response_type)
    }
}