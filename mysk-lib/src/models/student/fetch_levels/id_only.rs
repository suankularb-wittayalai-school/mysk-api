use mysk_lib_macros::fetch_level_variant::non_db_request_variant;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    common::{requests::FetchLevel, traits::FetchLevelVariant},
    student::db::DbStudent,
};

use mysk_lib_macros::non_db_request_variant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyStudent {
    pub id: Uuid,
}

non_db_request_variant!(IdOnlyStudent, DbStudent);
