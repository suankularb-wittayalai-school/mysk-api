use crate::{
    models::{club::db::DbClub, traits::FetchLevelVariant},
    permissions::{ActionType},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyClub {
    pub id: Uuid,
}

mysk_lib_macros::impl_id_only_variant_from!(club, IdOnlyClub, DbClub);
