use mysk_lib_macros::{id_only_variant_boiler_plate, impl_fetch_level_variant_from};
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

id_only_variant_boiler_plate!(IdOnlyClassroom, DbClassroom);
