use crate::{
    common::string::MultiLangString, models::subject_group::db::DbSubjectGroup, prelude::*,
};
use chrono::{DateTime, Utc};
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

impl_fetch_level_variant_from!(subject_group, Default, DefaultSubjectGroup, DbSubjectGroup);
