use crate::{
    common::requests::FetchLevel,
    models::{
        classroom::Classroom,
        contact::Contact,
        person::Person,
        subject::Subject,
        subject_group::SubjectGroup,
        teacher::db::DbTeacher,
        traits::{FetchLevelVariant, TopLevelGetById},
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
pub struct DetailedTeacher {
    pub id: Uuid,
    pub teacher_id: Option<String>,
    pub contacts: Vec<Contact>,
    pub class_advisor_at: Option<Classroom>,
    pub user: Option<User>,
    pub person: Option<Person>,
    pub subject_group: SubjectGroup,
    pub subjects_in_charge: Vec<Subject>,
}

#[async_trait]
impl FetchLevelVariant<DbTeacher> for DetailedTeacher {
    async fn from_table(
        pool: &PgPool,
        table: DbTeacher,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        authorizer
            .authorize_teacher(&table, pool, ActionType::ReadDetailed)
            .await?;

        let contact_ids = DbTeacher::get_teacher_contacts(pool, table.id).await?;
        let classroom_id = DbTeacher::get_teacher_advisor_at(pool, table.id, None).await?;
        let subject_ids = DbTeacher::get_subject_in_charge(pool, table.id, None).await?;

        let subject_group = SubjectGroup::get_by_id(
            pool,
            table.subject_group_id,
            descendant_fetch_level,
            Some(&FetchLevel::IdOnly),
            authorizer,
        )
        .await?;

        let user = match table.user_id {
            Some(user_id) => Some(User::get_by_id(pool, user_id).await?),
            None => None,
        };

        let person = match table.person_id {
            Some(person_id) => Some(Person::get_by_id(pool, person_id).await?),
            None => None,
        };

        Ok(Self {
            id: table.id,
            teacher_id: table.teacher_id,
            contacts: Contact::get_by_ids(
                pool,
                contact_ids,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            class_advisor_at: match classroom_id {
                Some(classroom_id) => Some(
                    Classroom::get_by_id(
                        pool,
                        classroom_id,
                        descendant_fetch_level,
                        Some(&FetchLevel::IdOnly),
                        authorizer,
                    )
                    .await?,
                ),
                None => None,
            },
            user,
            person,
            subject_group,
            subjects_in_charge: Subject::get_by_ids(
                pool,
                subject_ids,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
        })
    }
}
