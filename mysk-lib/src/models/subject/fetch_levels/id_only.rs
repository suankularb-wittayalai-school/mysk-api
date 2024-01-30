use crate::prelude::*;

use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::common::{requests::FetchLevel, traits::FetchLevelVariant};
use crate::models::subject::db::DbSubject;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct IdOnlySubject {
    pub id: Uuid,
}

impl_id_only_variant_from!(IdOnlySubject, DbSubject);
