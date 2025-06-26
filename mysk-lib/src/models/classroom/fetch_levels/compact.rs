use crate::models::classroom::db::DbClassroom;
use mysk_lib_macros::impl_fetch_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactClassroom {
    pub id: Uuid,
    pub number: i64,
    pub room: Option<String>,
}

impl From<DbClassroom> for CompactClassroom {
    fn from(classroom: DbClassroom) -> Self {
        Self {
            id: classroom.id,
            number: classroom.number,
            room: classroom.main_room,
        }
    }
}

impl_fetch_variant_from!(classroom, Compact, CompactClassroom, DbClassroom);
