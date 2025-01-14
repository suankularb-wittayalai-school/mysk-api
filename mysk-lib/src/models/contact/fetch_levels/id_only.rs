use crate::{
    common::requests::FetchLevel,
    models::{contact::db::DbContact, traits::FetchLevelVariant},
    permissions::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyContact {
    pub id: Uuid,
}

mysk_lib_macros::impl_id_only_variant_from!(contact, IdOnlyContact, DbContact);
