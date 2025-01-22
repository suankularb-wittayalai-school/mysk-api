use crate::models::elective_subject::db::DbElectiveSubject;
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyElectiveSubject {
    pub id: Uuid,
}

impl_id_only_variant_from!(elective_subject, IdOnlyElectiveSubject, DbElectiveSubject);
