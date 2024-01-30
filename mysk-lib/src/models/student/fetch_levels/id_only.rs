use crate::prelude::*;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    common::{requests::FetchLevel, traits::FetchLevelVariant},
    student::db::DbStudent,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyStudent {
    pub id: Uuid,
}

impl_id_only_variant_from!(IdOnlyStudent, DbStudent);
