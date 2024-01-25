use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::classroom::db::DbClassroom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactClassroom {
    pub id: Uuid,
    pub number: i64,
    pub room: String,
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
