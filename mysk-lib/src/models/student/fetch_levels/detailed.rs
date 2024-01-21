use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    common::{
        requests::FetchLevel,
        string::MultiLangString,
        traits::{FetchLevelVariant, TopLevelGetById},
    },
    contact::Contact,
    person::enums::{blood_group::BloodGroup, sex::Sex},
    student::db::DbStudent,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedStudent {
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
    pub contacts: Vec<Contact>,
    pub classroom: Option<String>, // TODO: Add classroom model
    pub class_no: Option<i64>,
    pub user: Option<String>, // TODO: Add user model

    pub citizen_id: Option<String>,
    // pub passport_id: Option<String>,
    pub blood_group: Option<BloodGroup>,
}

#[async_trait]
impl FetchLevelVariant<DbStudent> for DetailedStudent {
    async fn from_table(
        pool: &PgPool,
        table: DbStudent,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let contact_ids = DbStudent::get_student_contacts(pool, table.id).await?;
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
            contacts: Contact::get_by_ids(
                pool,
                contact_ids,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
            )
            .await?,
            classroom: None, // TODO: Add classroom model
            class_no: None,  // TODO: Add class_no model
            user: None,      // TODO: Add user model

            citizen_id: table.citizen_id,
            blood_group: table.blood_group,
        })
    }
}