use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::common::{requests::FetchLevel, traits::FetchLevelVariant};
use crate::models::elective_subject::db::DbElectiveSubject;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyElectiveSubject {
    pub id: Uuid,
}

impl_id_only_variant_from!(IdOnlyElectiveSubject, DbElectiveSubject);
