use crate::{
    common::requests::FetchLevel,
    models::{subject::db::DbSubject, traits::FetchLevelVariant},
    permissions::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlySubject {
    pub id: Uuid,
}

impl_id_only_variant_from!(subject, IdOnlySubject, DbSubject);
