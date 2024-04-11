use crate::{
    models::{
        common::{requests::FetchLevel, traits::FetchLevelVariant},
        teacher::db::DbTeacher,
    },
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyTeacher {
    pub id: Uuid,
}

impl_id_only_variant_from!(IdOnlyTeacher, DbTeacher);
