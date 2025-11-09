use crate::{
    common::requests::FetchLevel,
    models::{
        cheer_practice_attendance::db::DbCheerPracticeAttendance,
        cheer_practice_period::CheerPracticePeriod, enums::CheerPracticeAttendanceType,
        student::Student, traits::FetchVariant,
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
    pub student: Student,
    pub practice_period: CheerPracticePeriod,
    pub presence: Option<CheerPracticeAttendanceType>,
    pub presence_at_end: Option<CheerPracticeAttendanceType>,
    pub absence_reason: Option<String>,
    pub disabled: bool,
}

impl FetchVariant for DefaultCheerPracticeAttendance {
    type Relation = DbCheerPracticeAttendance;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let student = Student::get_by_id(
            pool,
            relation.student_id,
            descendant_fetch_level,
            FetchLevel::IdOnly,
            authorizer,
        )
        .await?;

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
            student,
            practice_period,
            presence: relation.presence,
            presence_at_end: relation.presence_at_end,
            absence_reason: relation.absence_reason,
            disabled: relation.disabled,
        })
    }
}
