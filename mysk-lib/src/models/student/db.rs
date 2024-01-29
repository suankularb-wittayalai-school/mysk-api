use chrono::{DateTime, NaiveDate, Utc};
use sqlx::query;
use uuid::Uuid;

use crate::{
    helpers::date::get_current_academic_year,
    models::{
        classroom::ClassroomWClassNo,
        // common::traits::{BaseQuery, GetById},
        person::enums::{blood_group::BloodGroup, sex::Sex, shirt_size::ShirtSize},
    },
};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT students.id, students.created_at, prefix_th, prefix_en, first_name_th, first_name_en, last_name_th, last_name_en, middle_name_th, middle_name_en, nickname_th, nickname_en, birthdate, citizen_id, profile, pants_size, shirt_size, blood_group, sex, student_id, user_id FROM students INNER JOIN people ON students.person_id = people.id"
)]
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
    pub student_id: String,
    pub user_id: Option<Uuid>,
}

impl DbStudent {
    pub async fn get_student_contacts(
        pool: &sqlx::PgPool,
        student_id: Uuid,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        let res = query!(
            r#"SELECT contacts.id FROM contacts INNER JOIN person_contacts ON contacts.id = person_contacts.contact_id INNER JOIN people ON person_contacts.person_id = people.id INNER JOIN students ON people.id = students.person_id WHERE students.id = $1"#,
            student_id
        ).fetch_all(pool).await?;

        Ok(res.iter().map(|r| r.id).collect())
    }

    pub async fn get_student_classroom(
        pool: &sqlx::PgPool,
        student_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Option<ClassroomWClassNo>, sqlx::Error> {
        let res = query!(
            r#"SELECT classroom_id, class_no FROM classroom_students INNER JOIN classrooms ON classrooms.id = classroom_id WHERE student_id = $1 AND year = $2"#,
            student_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            }
        )
        .fetch_optional(pool)
        .await?;

        match res {
            None => Ok(None),
            Some(res) => Ok(Some(ClassroomWClassNo {
                id: res.classroom_id,
                class_no: res.class_no,
            })),
        }
    }
}
