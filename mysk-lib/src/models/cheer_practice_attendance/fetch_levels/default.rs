use crate::{
    common::requests::FetchLevel,
    models::{
        cheer_practice_attendance::db::DbCheerPracticeAttendance,
        cheer_practice_period::CheerPracticePeriod, enums::CheerPracticeAttendanceType,
        traits::FetchVariant,
    },
    permissions::Authorizer,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefaultCheerPracticeAttendance {
    pub id: Uuid,
    pub practice_period: CheerPracticePeriod,
    pub presence: CheerPracticeAttendanceType,
    pub presence_at_end: Option<CheerPracticeAttendanceType>,
    pub absence_reason: Option<String>,
}

impl FetchVariant for DefaultCheerPracticeAttendance {
    type Relation = DbCheerPracticeAttendance;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let practice_period = CheerPracticePeriod::get_by_id(
            pool,
            relation.practice_period_id,
            descendant_fetch_level,
            FetchLevel::IdOnly,
            authorizer,
        )
        .await?;

        Ok(Self {
            id: relation.id,
            practice_period,
            presence: relation.presence,
            presence_at_end: relation.presence_at_end,
            absence_reason: relation.absence_reason,
        })
    }
}
