use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::student::db::DbStudent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyStudent {
    pub id: Uuid,
}

impl From<DbStudent> for IdOnlyStudent {
    fn from(student: DbStudent) -> Self {
        Self { id: student.id }
    }
}
