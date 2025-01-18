use crate::{
    common::{requests::FetchLevel, string::MultiLangString},
    models::{elective_subject::db::DbElectiveSubject, traits::FetchLevelVariant},
    permissions::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use mysk_lib_macros::impl_fetch_level_variant_from;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactElectiveSubject {
    pub id: Uuid,
    pub name: MultiLangString,
    pub code: MultiLangString,
    pub short_name: MultiLangString,
    pub class_size: i64,
    pub cap_size: i64,
    pub session_code: String,
}

impl From<DbElectiveSubject> for CompactElectiveSubject {
    fn from(subject: DbElectiveSubject) -> Self {
        Self {
            id: subject.id,
            name: MultiLangString::new(subject.name_th, Some(subject.name_en)),
            code: MultiLangString::new(subject.code_th, Some(subject.code_en)),
            short_name: MultiLangString::new(
                subject.short_name_th.unwrap_or_default(),
                subject.short_name_en,
            ),
            class_size: subject.class_size,
            cap_size: subject.cap_size,
            session_code: subject.session_code,
        }
    }
}

impl_fetch_level_variant_from!(
    elective_subject,
    Compact,
    CompactElectiveSubject,
    DbElectiveSubject
);
