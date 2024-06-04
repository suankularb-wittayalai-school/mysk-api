use crate::{
    common::requests::FetchLevel,
    models::{
        club::Club,
        club_request::{db::DbClubRequest, ClubRequest},
        enums::SubmissionStatus,
        student::Student,
        traits::{FetchLevelVariant, TopLevelGetById},
    },
    prelude::*,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefaultClubRequest {
    pub ids: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub clubs: Club,
    pub student_ids: Student,
    pub year: Option<i64>,
    pub membership_status: SubmissionStatus,
}

impl FetchLevelVariant<DbClubRequest> for DefaultClubRequest {
    async fn from_table(
        pool: &PgPool,
        table: DbClubRequest,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        let club = Club::get_by_id(
            pool,
            table.club_id,
            descendant_fetch_level,
            Some(&FetchLevel::IdOnly),
        )
        .await?;
        let student = Student::get_by_id(
            pool,
            table.student_id,
            descendant_fetch_level,
            Some(&FetchLevel::IdOnly),
        )
        .await?;

        Ok(Self {
            ids: Some(table.id),
            created_at: table.created_at,
            clubs: club,
            student_ids: student,
            year: table.year,
            membership_status: table.membership_status,
        })
    }
}
