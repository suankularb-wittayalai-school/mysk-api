use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::models::{
    common::{requests::FetchLevel, string::MultiLangString, traits::FetchLevelVariant},
    student::db::DbStudent,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactStudent {
    pub id: Uuid,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub last_name: MultiLangString,
    pub nickname: Option<MultiLangString>,
    pub student_id: String,
    pub profile_url: Option<String>,
}

impl From<DbStudent> for CompactStudent {
    fn from(student: DbStudent) -> Self {
        Self {
            id: student.id,
            prefix: MultiLangString::new(student.prefix_th, student.prefix_en),
            first_name: MultiLangString::new(student.first_name_th, student.first_name_en),
            last_name: MultiLangString::new(student.last_name_th, student.last_name_en),
            nickname: student
                .nickname_th
                .map(|th| MultiLangString::new(th, student.nickname_en)),
            student_id: student.student_id,
            profile_url: student.profile,
        }
    }
}

impl FetchLevelVariant<DbStudent> for CompactStudent {
    async fn from_table(
        _pool: &PgPool,
        table: DbStudent,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, Error> {
        Ok(Self::from(table))
    }
}
