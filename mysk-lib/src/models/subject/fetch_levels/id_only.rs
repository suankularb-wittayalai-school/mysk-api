use crate::prelude::*;
use mysk_lib_macros::{impl_fetch_level_variant_from, impl_id_only_variant_from};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::common::{requests::FetchLevel, traits::FetchLevelVariant};
use crate::models::subject::db::DbSubject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlySubject {
    pub id: Uuid,
}

impl_id_only_variant_from!(IdOnlySubject, DbSubject);
