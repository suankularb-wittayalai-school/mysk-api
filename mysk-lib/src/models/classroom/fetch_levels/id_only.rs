use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::classroom::db::DbClassroom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyClassroom {
    pub id: Uuid,
}

impl From<DbClassroom> for IdOnlyClassroom {
    fn from(classroom: DbClassroom) -> Self {
        Self { id: classroom.id }
    }
}
