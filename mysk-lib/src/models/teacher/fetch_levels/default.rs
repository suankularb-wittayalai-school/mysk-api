use crate::{
    common::requests::FetchLevel,
    models::{
        classroom::Classroom, contact::Contact, person::Person, subject::Subject,
        subject_group::SubjectGroup, teacher::db::DbTeacher, traits::FetchVariant, user::User,
    },
    permissions::{ActionType, Authorizable as _, Authorizer},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefaultTeacher {
    pub id: Uuid,
    pub teacher_id: Option<String>,
    pub contacts: Vec<Contact>,
    pub class_advisor_at: Option<Classroom>,
    pub user: Option<User>,
    pub person: Option<Person>,
    pub subject_group: SubjectGroup,
    pub subjects_in_charge: Vec<Subject>,
}

impl FetchVariant for DefaultTeacher {
    type Relation = DbTeacher;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let mut conn = pool.acquire().await?;
        authorizer
            .authorize_teacher(&relation, &mut conn, ActionType::ReadDefault)
            .await?;

        let contact_ids = DbTeacher::get_teacher_contacts(&mut conn, relation.id).await?;
        let classroom_id = DbTeacher::get_teacher_advisor_at(&mut conn, relation.id, None).await?;
        let subject_id = DbTeacher::get_subject_in_charge(&mut conn, relation.id, None).await?;

        let subject_group = SubjectGroup::get_by_id(
            pool,
            relation.subject_group_id,
            descendant_fetch_level,
            FetchLevel::IdOnly,
            authorizer,
        )
        .await?;

        let user = match relation.user_id {
            Some(user_id) => Some(User::get_by_id(&mut conn, user_id).await?),
            None => None,
        };

        let person = match relation.person_id {
            Some(person_id) => Some(Person::get_by_id(&mut conn, person_id).await?),
            None => None,
        };
        drop(conn);

        Ok(Self {
            id: relation.id,
            teacher_id: relation.teacher_id,
            contacts: Contact::get_by_ids(
                pool,
                &contact_ids,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            class_advisor_at: match classroom_id {
                Some(classroom_id) => Some(
                    Classroom::get_by_id(
                        pool,
                        classroom_id,
                        descendant_fetch_level,
                        FetchLevel::IdOnly,
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
                &subject_id,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
        })
    }
}
