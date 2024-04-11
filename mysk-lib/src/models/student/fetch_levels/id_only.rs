use crate::{
    common::requests::FetchLevel,
    models::{student::db::DbStudent, traits::FetchLevelVariant},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyStudent {
    pub id: Uuid,
}

impl_id_only_variant_from!(IdOnlyStudent, DbStudent);
