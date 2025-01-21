use crate::{models::student::db::DbStudent, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactStudent {
    pub id: Uuid,
    pub student_id: Option<String>,
    pub person_id: Uuid,
    pub user_id: Option<Uuid>,
}

impl From<DbStudent> for CompactStudent {
    fn from(student: DbStudent) -> Self {
        Self {
            id: student.id,
            student_id: student.student_id,
            person_id: student.person_id,
            user_id: student.user_id,
        }
    }
}

impl_fetch_level_variant_from!(student, Compact, CompactStudent, DbStudent);
