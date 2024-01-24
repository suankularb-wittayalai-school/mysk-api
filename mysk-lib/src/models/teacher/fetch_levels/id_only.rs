use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::teacher::db::DbTeacher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyTeacher {
    pub id: Uuid,
}

impl From<DbTeacher> for IdOnlyTeacher {
    fn from(teacher: DbTeacher) -> Self {
        Self { id: teacher.id }
    }
}
