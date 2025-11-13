use crate::{
    common::requests::FetchLevel,
    models::{
        classroom::Classroom, contact::Contact, person::Person, student::db::DbStudent,
        traits::FetchVariant, user::User,
    },
    permissions::{ActionType, Authorizable as _, Authorizer},
    prelude::*,
};
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

impl FetchVariant for DefaultStudent {
    type Relation = DbStudent;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let mut conn = pool.acquire().await?;
        authorizer
            .authorize_student(&relation, &mut conn, ActionType::ReadDefault)
            .await?;

        let contact_ids = DbStudent::get_student_contacts(&mut conn, relation.id).await?;

        let classroom = DbStudent::get_student_classroom(&mut conn, relation.id, None).await?;
        let user = match relation.user_id {
            Some(user_id) => Some(User::get_by_id(&mut conn, user_id, None).await?),
            None => None,
        };
        let person = Person::get_by_id(&mut conn, relation.person_id).await?;
        drop(conn);

        Ok(Self {
            id: relation.id,
            student_id: relation.student_id,
            contacts: Contact::get_by_ids(
                pool,
                &contact_ids,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            classroom: match &classroom {
                Some(classroom) => Some(
                    Classroom::get_by_id(
                        pool,
                        classroom.id,
                        descendant_fetch_level,
                        FetchLevel::IdOnly,
                        authorizer,
                    )
                    .await?,
                ),
                None => None,
            },
            class_no: classroom.map(|classroom| classroom.class_no),
            user,
            person,
        })
    }
}
