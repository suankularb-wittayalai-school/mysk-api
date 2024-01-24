use chrono::{DateTime, NaiveDate, Utc};
use sqlx::query;
use uuid::Uuid;

use crate::models::{
    classroom::ClassroomWClassNo,
    common::traits::{BaseQuery, GetById},
    person::enums::{blood_group::BloodGroup, sex::Sex, shirt_size::ShirtSize},
};

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
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

impl BaseQuery for DbTeacher {
    fn base_query() -> &'static str {
        r#"SELECT teachers.id, teachers.created_at, prefix_th, prefix_en, first_name_th, first_name_en, last_name_th, last_name_en, middle_name_th, middle_name_en, nickname_th, nickname_en, birthdate, citizen_id, profile, pants_size, shirt_size, blood_group, sex, teacher_id, user_id, subject_group_id FROM teachers INNER JOIN people ON teachers.person_id = people.id"#
    }
}

impl GetById for DbTeacher {
    async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
        // sqlx::query_as!(DbStudent, r#"SELECT students.id, students.created_at, prefix_th, prefix_en, first_name_th, first_name_en, last_name_th, last_name_en, middle_name_th, middle_name_en, nickname_th, nickname_en, birthdate, citizen_id, profile, pants_size, shirt_size AS "shirt_size: _", blood_group AS "blood_group: _", sex AS "sex: _", student_id, user_id FROM students INNER JOIN people ON students.person_id = people.id WHERE students.id = $1"#, id)
        //     .fetch_one(pool)
        //     .await

        sqlx::query_as::<_, DbTeacher>(
            format!("{} WHERE teachers.id = $1", Self::base_query()).as_str(),
        )
        .bind(id)
        .fetch_one(pool)
        .await
    }

    async fn get_by_ids(pool: &sqlx::PgPool, ids: Vec<Uuid>) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, DbTeacher>(
            format!("{} WHERE teachers.id = ANY($1)", Self::base_query()).as_str(),
        )
        .bind(ids)
        .fetch_all(pool)
        .await
    }
}

impl DbTeacher {
    pub async fn get_teacher_contacts(
        pool: &sqlx::PgPool,
        student_id: Uuid,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        let res = query!(
            r#"SELECT contacts.id FROM contacts INNER JOIN person_contacts ON contacts.id = person_contacts.contact_id INNER JOIN people ON person_contacts.person_id = people.id INNER JOIN teachers ON people.id = teachers.person_id WHERE teachers.id = $1"#,
            student_id
        ).fetch_all(pool).await?;

        Ok(res.iter().map(|r| r.id).collect())
    }
}
