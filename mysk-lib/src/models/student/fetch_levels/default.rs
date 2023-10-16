use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    common::{requests::FetchLevel, string::MultiLangString, traits::FetchLevelVariant},
    person::enums::sex::Sex,
    student::db::DbStudent,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultStudent {
    pub id: Uuid,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub middle_name: Option<MultiLangString>,
    pub last_name: MultiLangString,
    pub nickname: Option<MultiLangString>,
    pub student_id: String,
    pub profile_url: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub sex: Sex,
    pub contact: Option<String>,   // TODO: Add contact model
    pub classroom: Option<String>, // TODO: Add classroom model
    pub class_no: Option<i64>,
    pub user: Option<String>, // TODO: Add user model
}

#[async_trait]
impl FetchLevelVariant<DbStudent> for DefaultStudent {
    async fn from_table(
        pool: &PgPool,
        table: DbStudent,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: table.id,
            prefix: MultiLangString::new(table.prefix_th, table.prefix_en),
            first_name: MultiLangString::new(table.first_name_th, table.first_name_en),
            last_name: MultiLangString::new(table.last_name_th, table.last_name_en),
            middle_name: table
                .middle_name_th
                .map(|th| MultiLangString::new(th, table.middle_name_en)),
            nickname: table
                .nickname_th
                .map(|th| MultiLangString::new(th, table.nickname_en)),
            student_id: table.student_id,
            profile_url: table.profile,
            birthdate: table.birthdate,
            sex: table.sex,
            contact: None,   // TODO: Add contact model
            classroom: None, // TODO: Add classroom model
            class_no: None,  // TODO: Add class_no model
            user: None,      // TODO: Add user model
        })
    }
}
