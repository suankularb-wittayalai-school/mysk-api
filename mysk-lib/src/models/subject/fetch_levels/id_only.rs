use crate::{
    models::{
        common::{requests::FetchLevel, traits::FetchLevelVariant},
        subject::db::DbSubject,
    },
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlySubject {
    pub id: Uuid,
}

impl_id_only_variant_from!(IdOnlySubject, DbSubject);
