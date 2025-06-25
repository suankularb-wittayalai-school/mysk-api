use crate::{
    common::requests::FetchLevel,
    models::{
        club::Club,
        club_request::db::DbClubRequest,
        enums::SubmissionStatus,
        student::Student,
        traits::{FetchLevelVariant, TopLevelGetById as _},
    },
    permissions::Authorizer,
    prelude::*,
};
use async_trait::async_trait;
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

#[async_trait]
impl FetchLevelVariant<DbClubRequest> for DefaultClubRequest {
    async fn from_table(
        pool: &PgPool,
        table: DbClubRequest,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        Ok(Self {
            id: table.id,
            created_at: table.created_at,
            club: Club::get_by_id(
                pool,
                table.club_id,
                descendant_fetch_level,
                Some(FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            student: Student::get_by_id(
                pool,
                table.student_id,
                descendant_fetch_level,
                Some(FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            year: table.year,
            membership_status: table.membership_status,
        })
    }
}
