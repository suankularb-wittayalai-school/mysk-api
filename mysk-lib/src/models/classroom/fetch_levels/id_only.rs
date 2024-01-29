use crate::prelude::*;
use mysk_lib_macros::{impl_fetch_level_variant_from, impl_id_only_variant_from};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    classroom::db::DbClassroom,
    common::{requests::FetchLevel, traits::FetchLevelVariant},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyClassroom {
    pub id: Uuid,
}

impl_id_only_variant_from!(IdOnlyClassroom, DbClassroom);
