use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    classroom::db::DbClassroom,
    common::{
        requests::FetchLevel,
        traits::{FetchLevelVariant, TopLevelGetById},
    },
    contact::Contact,
    student::Student,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultClassroom {
    pub id: Uuid,
    pub number: i64,
    pub room: String,
    // pub class_advisor: Vec<Teacher>, // TODO: Change to Teacher
    pub students: Vec<Student>,
    pub contacts: Vec<Contact>,
    pub year: i64,
}

impl FetchLevelVariant<DbClassroom> for DefaultClassroom {
    async fn from_table(
        pool: &PgPool,
        table: DbClassroom,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let student_ids = DbClassroom::get_classroom_students(pool, table.id).await?;
        let contact_ids = DbClassroom::get_classroom_contacts(pool, table.id).await?;
        // TODO: Add class_advisor model
        // let class_advisor_ids = DbClassroom::get_classroom_class_advisors(pool, table.id).await?;

        Ok(Self {
            id: table.id,
            number: table.number,
            room: table.main_room,
            students: Student::get_by_ids(pool, student_ids, descendant_fetch_level, None).await?,
            contacts: Contact::get_by_ids(pool, contact_ids, None, None).await?,
            year: table.year,
        })
    }
}