use crate::{
    helpers::date::get_current_academic_year,
    models::person::enums::{blood_group::BloodGroup, sex::Sex, shirt_size::ShirtSize},
    prelude::*,
};
use chrono::{DateTime, NaiveDate, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::{query, FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(query = r#"
    SELECT
        teachers.id, teachers.created_at, prefix_th, prefix_en, first_name_th, first_name_en,
        last_name_th, last_name_en, middle_name_th, middle_name_en, nickname_th, nickname_en,
        birthdate, citizen_id, profile, pants_size, shirt_size, blood_group, sex, teacher_id,
        user_id, subject_group_id
    FROM teachers INNER JOIN people ON teachers.person_id = people.id
    "#)]
#[get_by_id(table = "teachers")]
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
    pub async fn get_teacher_contacts(pool: &PgPool, teacher_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query!(
            r#"
            SELECT contacts.id FROM contacts
            INNER JOIN person_contacts ON contacts.id = person_contacts.contact_id
            INNER JOIN people ON person_contacts.person_id = people.id
            INNER JOIN teachers ON people.id = teachers.person_id
            WHERE teachers.id = $1
            "#,
            teacher_id,
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbTeacher::get_teacher_contacts".to_string(),
            )),
        }
    }

    pub async fn get_teacher_advisor_at(
        pool: &PgPool,
        teacher_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Option<Uuid>> {
        let res = query!(
            r#"
            SELECT classroom_id FROM classroom_advisors
            INNER JOIN classrooms ON classrooms.id = classroom_id
            WHERE teacher_id = $1 AND classrooms.year = $2
            "#,
            teacher_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            },
        )
        .fetch_optional(pool)
        .await;

        match res {
            Ok(res) => Ok(res.map(|r| r.classroom_id)),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbTeacher::get_teacher_advisor_at".to_string(),
            )),
        }
    }

    pub async fn get_subject_in_charge(
        pool: &PgPool,
        teacher_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            r#"SELECT subject_id FROM subject_teachers WHERE teacher_id = $1 AND year = $2"#,
            teacher_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            },
        )
        .fetch_all(pool)
        .await;

        let res2 = query!(
            r#"SELECT subject_id FROM subject_co_teachers WHERE teacher_id = $1 AND year = $2"#,
            teacher_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            },
        )
        .fetch_all(pool)
        .await;

        let res = match res {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::InternalSeverError(
                    e.to_string(),
                    "DbTeacher::get_subject_in_charge".to_string(),
                ));
            }
        };
        let res2 = match res2 {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::InternalSeverError(
                    e.to_string(),
                    "DbTeacher::get_subject_in_charge".to_string(),
                ));
            }
        };
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
