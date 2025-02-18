use crate::{
    common::requests::FetchLevel,
    models::{
        classroom::db::DbClassroom,
        contact::Contact,
        student::Student,
        teacher::Teacher,
        traits::{FetchLevelVariant, TopLevelGetById as _},
    },
    permissions::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultClassroom {
    pub id: Uuid,
    pub number: i64,
    pub room: Option<String>,
    pub class_advisor: Vec<Teacher>,
    pub students: Vec<Student>,
    pub contacts: Vec<Contact>,
    pub year: i64,
}

#[async_trait]
impl FetchLevelVariant<DbClassroom> for DefaultClassroom {
    async fn from_table(
        pool: &PgPool,
        table: DbClassroom,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        authorizer
            .authorize_classroom(&table, pool, ActionType::ReadDefault)
            .await?;

        let student_ids = DbClassroom::get_classroom_students(pool, table.id).await?;
        let contact_ids = DbClassroom::get_classroom_contacts(pool, table.id).await?;
        let class_advisor_ids = DbClassroom::get_classroom_advisors(pool, table.id, None).await?;

        Ok(Self {
            id: table.id,
            number: table.number,
            room: table.main_room,
            students: Student::get_by_ids(
                pool,
                student_ids,
                descendant_fetch_level,
                Some(FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            contacts: Contact::get_by_ids(
                pool,
                contact_ids,
                descendant_fetch_level,
                Some(FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            year: table.year,
            class_advisor: Teacher::get_by_ids(
                pool,
                class_advisor_ids,
                descendant_fetch_level,
                Some(FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
        })
    }
}
