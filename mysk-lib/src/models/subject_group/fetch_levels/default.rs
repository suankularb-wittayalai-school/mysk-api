use crate::{common::string::MultiLangString, models::subject_group::db::DbSubjectGroup};
use chrono::{DateTime, Utc};
use mysk_lib_macros::impl_fetch_variant_from;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultSubjectGroup {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub name: MultiLangString,
}

impl From<DbSubjectGroup> for DefaultSubjectGroup {
    fn from(subject_group: DbSubjectGroup) -> Self {
        Self {
            id: subject_group.id,
            created_at: subject_group.created_at,
            name: MultiLangString::new(subject_group.name_th, Some(subject_group.name_en)),
        }
    }
}

impl_fetch_variant_from!(subject_group, Default, DefaultSubjectGroup, DbSubjectGroup);
