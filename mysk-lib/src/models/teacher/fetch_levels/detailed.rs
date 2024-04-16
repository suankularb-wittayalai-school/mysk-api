use crate::{
    common::{requests::FetchLevel, string::MultiLangString},
    models::{
        classroom::Classroom,
        contact::Contact,
        enums::{BloodGroup, Sex},
        subject::Subject,
        subject_group::SubjectGroup,
        teacher::db::DbTeacher,
        traits::{FetchLevelVariant, TopLevelGetById},
        user::User,
    },
    prelude::*,
};
use chrono::NaiveDate;
use mysk_lib_macros::traits::db::GetById;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DetailedTeacher {
    pub id: Uuid,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub middle_name: Option<MultiLangString>,
    pub last_name: MultiLangString,
    pub nickname: Option<MultiLangString>,
    pub teacher_id: Option<String>,
    pub profile_url: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub sex: Sex,
    pub contacts: Vec<Contact>,
    pub class_advisor_at: Option<Classroom>,
    pub user: Option<User>,
    pub subject_group: SubjectGroup,
    pub subjects_in_charge: Vec<Subject>,
    pub citizen_id: Option<String>,
    pub blood_group: Option<BloodGroup>,
}

impl FetchLevelVariant<DbTeacher> for DetailedTeacher {
    async fn from_table(
        pool: &PgPool,
        table: DbTeacher,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        let contact_ids = DbTeacher::get_teacher_contacts(pool, table.id).await?;
        let classroom_id = DbTeacher::get_teacher_advisor_at(pool, table.id, None).await?;
        let subject_ids = DbTeacher::get_subject_in_charge(pool, table.id, None).await?;

        let subject_group = SubjectGroup::get_by_id(
            pool,
            table.subject_group_id,
            descendant_fetch_level,
            Some(&FetchLevel::IdOnly),
        )
        .await?;

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
            teacher_id: table.teacher_id,
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
            class_advisor_at: match classroom_id {
                Some(classroom_id) => Some(
                    Classroom::get_by_id(
                        pool,
                        classroom_id,
                        descendant_fetch_level,
                        Some(&FetchLevel::IdOnly),
                    )
                    .await?,
                ),
                None => None,
            },
            user,
            subject_group,
            subjects_in_charge: Subject::get_by_ids(
                pool,
                subject_ids,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
            )
            .await?,
            citizen_id: table.citizen_id,
            blood_group: table.blood_group,
        })
    }
}
