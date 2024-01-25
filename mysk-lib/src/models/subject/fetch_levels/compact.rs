use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{common::string::MultiLangString, subject::db::DbSubject};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactSubject {
    pub id: Uuid,
    pub name: MultiLangString,
    pub code: MultiLangString,
    pub short_name: MultiLangString,
}

impl From<DbSubject> for CompactSubject {
    fn from(subject: DbSubject) -> Self {
        Self {
            id: subject.id,
            name: MultiLangString::new(subject.name_th, Some(subject.name_en)),
            code: MultiLangString::new(subject.code_th, Some(subject.code_en)),
            short_name: MultiLangString::new(subject.short_name_th, Some(subject.short_name_en)),
        }
    }
}