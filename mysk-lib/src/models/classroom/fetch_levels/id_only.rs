use crate::{
    models::{classroom::db::DbClassroom, traits::FetchLevelVariant},
    permissions::ActionType,
    prelude::*,
};
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyClassroom {
    pub id: Uuid,
}

impl_id_only_variant_from!(classroom, IdOnlyClassroom, DbClassroom);
