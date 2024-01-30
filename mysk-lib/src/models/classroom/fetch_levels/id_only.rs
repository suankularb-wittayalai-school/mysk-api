use crate::prelude::*;

use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    classroom::db::DbClassroom,
    common::{requests::FetchLevel, traits::FetchLevelVariant},
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct IdOnlyClassroom {
    pub id: Uuid,
}

impl_id_only_variant_from!(IdOnlyClassroom, DbClassroom);
