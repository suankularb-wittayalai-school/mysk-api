use crate::models::cheer_practice_attendance::db::DbCheerPracticeAttendance;
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyCheerPracticeAttendance {
    pub id: Uuid,
}

impl_id_only_variant_from!(
    cheer_practice_attendance,
    IdOnlyCheerPracticeAttendance,
    DbCheerPracticeAttendance,
);
