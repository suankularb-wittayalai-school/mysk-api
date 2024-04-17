use crate::{
    common::{requests::FetchLevel, string::MultiLangString},
    models::{subject::db::DbSubject, traits::FetchLevelVariant},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
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

impl_fetch_level_variant_from!(CompactSubject, DbSubject);
