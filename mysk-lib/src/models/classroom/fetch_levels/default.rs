use crate::{
    common::requests::FetchLevel,
    models::{
        classroom::db::DbClassroom, contact::Contact, teacher::Teacher, traits::FetchVariant,
    },
    permissions::{ActionType, Authorizable as _, Authorizer},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultClassroom {
    pub id: Uuid,
    pub number: i64,
    pub main_room: Option<String>,
    pub class_advisor: Vec<Teacher>,
    pub contacts: Vec<Contact>,
    pub year: i64,
}

impl FetchVariant for DefaultClassroom {
    type Relation = DbClassroom;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let mut conn = pool.acquire().await?;
        authorizer
            .authorize_classroom(&relation, &mut conn, ActionType::ReadDefault)
            .await?;

        let contact_ids = DbClassroom::get_classroom_contacts(&mut conn, relation.id).await?;
        let class_advisor_ids =
            DbClassroom::get_classroom_advisors(&mut conn, relation.id).await?;
        drop(conn);

        Ok(Self {
            id: relation.id,
            number: relation.number,
            main_room: relation.main_room,
            contacts: Contact::get_by_ids(
                pool,
                &contact_ids,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            year: relation.year,
            class_advisor: Teacher::get_by_ids(
                pool,
                &class_advisor_ids,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
        })
    }
}
