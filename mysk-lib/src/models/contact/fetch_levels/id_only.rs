use crate::{models::contact::db::DbContact, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyContact {
    pub id: Uuid,
}

impl_id_only_variant_from!(contact, IdOnlyContact, DbContact);
