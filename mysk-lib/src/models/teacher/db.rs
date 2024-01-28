use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{query, Error, PgPool};
use uuid::Uuid;

use crate::{
    helpers::date::get_current_academic_year,
    models::{
        // common::traits::{BaseQuery, GetById},
        person::enums::{blood_group::BloodGroup, sex::Sex, shirt_size::ShirtSize},
    },
};

use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow, BaseQuery, GetById)]
#[base_query(
    query = r#"SELECT teachers.id, teachers.created_at, prefix_th, prefix_en, first_name_th, first_name_en, last_name_th, last_name_en, middle_name_th, middle_name_en, nickname_th, nickname_en, birthdate, citizen_id, profile, pants_size, shirt_size, blood_group, sex, teacher_id, user_id, subject_group_id FROM teachers INNER JOIN people ON teachers.person_id = people.id"#
)]
pub struct DbTeacher {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub prefix_th: String,
    pub prefix_en: Option<String>,
    pub first_name_th: String,
    pub first_name_en: Option<String>,
    pub last_name_th: String,
    pub last_name_en: Option<String>,
    pub middle_name_th: Option<String>,
    pub middle_name_en: Option<String>,
    pub nickname_th: Option<String>,
    pub nickname_en: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub citizen_id: Option<String>,
    pub profile: Option<String>,
    pub pants_size: Option<String>,
    pub shirt_size: Option<ShirtSize>,
    pub blood_group: Option<BloodGroup>,
    pub sex: Sex,
    pub teacher_id: Option<String>,
    pub subject_group_id: i64,
    pub user_id: Option<Uuid>,
}

impl DbTeacher {
    pub async fn get_teacher_contacts(
        pool: &sqlx::PgPool,
        teacher_id: Uuid,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        let res = query!(
            r#"SELECT contacts.id FROM contacts INNER JOIN person_contacts ON contacts.id = person_contacts.contact_id INNER JOIN people ON person_contacts.person_id = people.id INNER JOIN teachers ON people.id = teachers.person_id WHERE teachers.id = $1"#,
            teacher_id
        ).fetch_all(pool).await?;

        Ok(res.iter().map(|r| r.id).collect())
    }

    pub async fn get_teacher_advisor_at(
        pool: &sqlx::PgPool,
        teacher_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        let res = query!(
            r#"SELECT classroom_id FROM classroom_advisors INNER JOIN classrooms ON classrooms.id = classroom_id WHERE teacher_id = $1 AND classrooms.year = $2"#,
            teacher_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            }
        )
        .fetch_optional(pool)
        .await?;

        Ok(res.map(|r| r.classroom_id))
    }

    pub async fn get_subject_in_charge(
        pool: &sqlx::PgPool,
        teacher_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        let res = query!(
            r#"SELECT subject_id FROM subject_teachers WHERE teacher_id = $1 AND year = $2"#,
            teacher_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            }
        )
        .fetch_all(pool)
        .await?;

        let res2 = query!(
            r#"SELECT subject_id FROM subject_co_teachers WHERE teacher_id = $1 AND year = $2"#,
            teacher_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            }
        )
        .fetch_all(pool)
        .await?;

        let mut result = vec![];

        for r in res {
            result.push(r.subject_id);
        }

        for r in res2 {
            result.push(r.subject_id);
        }

        Ok(result)
    }
}
