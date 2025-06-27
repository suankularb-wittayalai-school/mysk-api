use crate::{
    common::requests::FetchLevel,
    models::{
        club::Club, club_request::db::DbClubRequest, enums::SubmissionStatus, student::Student,
        traits::FetchVariant,
    },
    permissions::Authorizer,
    prelude::*,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefaultClubRequest {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub club: Club,
    pub student: Student,
    pub year: Option<i64>,
    pub membership_status: SubmissionStatus,
}

impl FetchVariant for DefaultClubRequest {
    type Relation = DbClubRequest;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        Ok(Self {
            id: relation.id,
            created_at: relation.created_at,
            club: Club::get_by_id(
                pool,
                relation.club_id,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            student: Student::get_by_id(
                pool,
                relation.student_id,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            year: relation.year,
            membership_status: relation.membership_status,
        })
    }
}
