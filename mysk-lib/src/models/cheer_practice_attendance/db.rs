use chrono::{DateTime, Utc};
use mysk_lib_macros::GetById;
use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::enums::CheerPracticeAttendanceType;

#[derive(Clone, Debug, Deserialize, FromRow, GetById)]
#[from_query(query = "
    SELECT
        id, created_at, practice_period_id, student_id, checker_id, presence, presence_at_end,
        absence_reason
    FROM cheer_practice_attendances
")]
pub struct DbCheerPracticeAttendance {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub practice_period_id: Uuid,
    pub student_id: Uuid,
    pub checker_id: Option<Uuid>,
    pub presence: CheerPracticeAttendanceType,
    pub presence_at_end: Option<CheerPracticeAttendanceType>,
    pub absence_reason: Option<String>,
}
