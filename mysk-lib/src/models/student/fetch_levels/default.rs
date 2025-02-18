use crate::{
    common::requests::FetchLevel,
    models::{
        classroom::Classroom,
        contact::Contact,
        person::Person,
        student::db::DbStudent,
        traits::{FetchLevelVariant, TopLevelGetById as _},
        user::User,
    },
    permissions::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefaultStudent {
    pub id: Uuid,
    pub student_id: Option<String>,
    pub contacts: Vec<Contact>,
    pub classroom: Option<Classroom>,
    pub class_no: Option<i64>,
    pub user: Option<User>,
    pub person: Person,
}

#[async_trait]
impl FetchLevelVariant<DbStudent> for DefaultStudent {
    async fn from_table(
        pool: &PgPool,
        table: DbStudent,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        authorizer
            .authorize_student(&table, pool, ActionType::ReadDefault)
            .await?;

        let contact_ids = DbStudent::get_student_contacts(pool, table.id).await?;

        let classroom = DbStudent::get_student_classroom(pool, table.id, None).await?;
        let user = match table.user_id {
            Some(user_id) => Some(User::get_by_id(pool, user_id).await?),
            None => None,
        };

        Ok(Self {
            id: table.id,
            student_id: table.student_id,
            contacts: Contact::get_by_ids(
                pool,
                contact_ids,
                descendant_fetch_level,
                Some(FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            classroom: match &classroom {
                Some(classroom) => Some(
                    Classroom::get_by_id(
                        pool,
                        classroom.id,
                        descendant_fetch_level,
                        Some(FetchLevel::IdOnly),
                        authorizer,
                    )
                    .await?,
                ),
                None => None,
            },
            class_no: classroom.map(|classroom| classroom.class_no),
            user,
            person: Person::get_by_id(pool, table.person_id).await?,
        })
    }
}
