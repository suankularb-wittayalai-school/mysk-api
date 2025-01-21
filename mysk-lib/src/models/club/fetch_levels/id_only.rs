use crate::{models::club::db::DbClub, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyClub {
    pub id: Uuid,
}

impl_id_only_variant_from!(club, IdOnlyClub, DbClub);
