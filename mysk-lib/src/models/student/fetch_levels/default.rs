use crate::{
    common::{requests::FetchLevel, string::MultiLangString},
    models::{
        classroom::Classroom,
        contact::Contact,
        enums::Sex,
        student::db::DbStudent,
        traits::{FetchLevelVariant, TopLevelGetById},
        user::User,
    },
    prelude::*,
};
use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefaultStudent {
    pub id: Uuid,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub middle_name: Option<MultiLangString>,
    pub last_name: MultiLangString,
    pub nickname: Option<MultiLangString>,
    pub student_id: Option<String>,
    pub profile_url: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub sex: Sex,
    pub contacts: Vec<Contact>,
    pub classroom: Option<Classroom>,
    pub class_no: Option<i64>,
    pub user: Option<User>,
}

#[async_trait]
impl FetchLevelVariant<DbStudent> for DefaultStudent {
    async fn from_table(
        pool: &PgPool,
        table: DbStudent,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        let contact_ids = DbStudent::get_student_contacts(pool, table.id).await?;

        let classroom = DbStudent::get_student_classroom(pool, table.id, None).await?;
        let user = match table.user_id {
            Some(user_id) => Some(User::get_by_id(pool, user_id).await?),
            None => None,
        };

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
            classroom: match &classroom {
                Some(classroom) => Some(
                    Classroom::get_by_id(
                        pool,
                        classroom.id,
                        descendant_fetch_level,
                        Some(&FetchLevel::IdOnly),
                    )
                    .await?,
                ),
                None => None,
            },
            class_no: classroom.map(|classroom| classroom.class_no),
            user,
        })
    }
}
