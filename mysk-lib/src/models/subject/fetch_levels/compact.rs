use crate::{common::string::MultiLangString, models::subject::db::DbSubject};
use mysk_lib_macros::impl_fetch_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
            short_name: MultiLangString::new(
                subject.short_name_th.unwrap_or_default(),
                subject.short_name_en,
            ),
        }
    }
}

impl_fetch_variant_from!(subject, Compact, CompactSubject, DbSubject);
