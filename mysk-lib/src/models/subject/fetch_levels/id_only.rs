use crate::{
    common::requests::FetchLevel,
    models::{subject::db::DbSubject, traits::FetchLevelVariant},
    prelude::*,
};
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlySubject {
    pub id: Uuid,
}

impl_id_only_variant_from!(subject, IdOnlySubject, DbSubject);
