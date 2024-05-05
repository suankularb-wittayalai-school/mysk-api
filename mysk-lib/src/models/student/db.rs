use crate::{
    helpers::date::get_current_academic_year,
    models::{
        classroom::ClassroomWClassNo,
        enums::{BloodGroup, Sex, ShirtSize},
    },
    prelude::*,
};
use chrono::{DateTime, NaiveDate, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::{query, FromRow, PgPool};
use uuid::Uuid;

#[derive(BaseQuery, Clone, Debug, Deserialize, FromRow, GetById)]
#[base_query(query = r"
    SELECT
        students.id, students.created_at, prefix_th, prefix_en, first_name_th, first_name_en,
        last_name_th, last_name_en, middle_name_th, middle_name_en, nickname_th, nickname_en,
        birthdate, citizen_id, profile, pants_size, shirt_size, blood_group, sex, student_id,
        user_id
    FROM students INNER JOIN people ON students.person_id = people.id
")]
#[get_by_id(table = "students")]
pub struct DbStudent {
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
    pub student_id: Option<String>,
    pub user_id: Option<Uuid>,
}

impl DbStudent {
    pub async fn get_student_from_user_id(pool: &PgPool, user_id: Uuid) -> Result<Option<Uuid>> {
        query!(r#"SELECT id FROM students WHERE user_id = $1"#, user_id)
            .fetch_optional(pool)
            .await
            .map(|res| res.map(|r| r.id))
            .map_err(|e| {
                Error::InternalSeverError(
                    e.to_string(),
                    "DbStudent::get_student_from_user_id".to_string(),
                )
            })
    }

    pub async fn get_student_contacts(pool: &PgPool, student_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query!(
            r#"
            SELECT contacts.id FROM contacts
            INNER JOIN person_contacts ON contacts.id = person_contacts.contact_id
            INNER JOIN people ON person_contacts.person_id = people.id
            INNER JOIN students ON people.id = students.person_id
            WHERE students.id = $1
            "#,
            student_id,
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbStudent::get_student_contacts".to_string(),
            )),
        }
    }

    pub async fn get_student_classroom(
        pool: &PgPool,
        student_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Option<ClassroomWClassNo>> {
        let res = query!(
            r#"
            SELECT classroom_id, class_no FROM classroom_students
            INNER JOIN classrooms ON classrooms.id = classroom_id
            WHERE student_id = $1 AND year = $2
            "#,
            student_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            },
        )
        .fetch_optional(pool)
        .await;

        match res {
            Ok(res) => match res {
                Some(res) => Ok(Some(ClassroomWClassNo {
                    id: res.classroom_id,
                    class_no: res.class_no,
                })),
                None => Ok(None),
            },
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbStudent::get_student_classroom".to_string(),
            )),
        }
    }
}
