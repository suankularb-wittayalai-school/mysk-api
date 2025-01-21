use crate::{
    models::{contact::db::DbContact, traits::FetchLevelVariant},
    permissions::{ActionType},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyContact {
    pub id: Uuid,
}

mysk_lib_macros::impl_id_only_variant_from!(contact, IdOnlyContact, DbContact);
