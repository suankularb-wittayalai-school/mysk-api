use crate::{
    common::requests::FetchLevel,
    models::{classroom::db::DbClassroom, traits::FetchLevelVariant},
    prelude::*,
};
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyClassroom {
    pub id: Uuid,
}

impl_id_only_variant_from!(classroom, IdOnlyClassroom, DbClassroom);
