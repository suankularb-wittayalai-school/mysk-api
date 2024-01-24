use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{common::string::MultiLangString, teacher::db::DbTeacher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactTeacher {
    pub id: Uuid,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub last_name: MultiLangString,
    pub nickname: Option<MultiLangString>,
    pub teacher_id: Option<String>,
    pub profile: Option<String>,
    pub subject_group: String, // TODO: Change to SubjectGroup
}

impl From<DbTeacher> for CompactTeacher {
    fn from(teacher: DbTeacher) -> Self {
        Self {
            id: teacher.id,
            prefix: MultiLangString::new(teacher.prefix_th, teacher.prefix_en),
            first_name: MultiLangString::new(teacher.first_name_th, teacher.first_name_en),
            last_name: MultiLangString::new(teacher.last_name_th, teacher.last_name_en),
            nickname: teacher
                .nickname_th
                .map(|th| MultiLangString::new(th, teacher.nickname_en)),
            teacher_id: teacher.teacher_id,
            profile: teacher.profile,
            subject_group: "TODO".to_string(),
        }
    }
}
