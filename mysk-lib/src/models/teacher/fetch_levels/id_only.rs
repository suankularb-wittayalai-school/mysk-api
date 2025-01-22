use crate::models::teacher::db::DbTeacher;
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyTeacher {
    pub id: Uuid,
}

impl_id_only_variant_from!(teacher, IdOnlyTeacher, DbTeacher);
