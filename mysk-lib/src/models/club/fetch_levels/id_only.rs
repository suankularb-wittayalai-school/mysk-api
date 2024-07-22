use crate::{
    common::requests::FetchLevel,
    models::{club::db::DbClub, traits::FetchLevelVariant},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyClub {
    pub id: Uuid,
}

mysk_lib_macros::impl_id_only_variant_from!(club, IdOnlyClub, DbClub);
