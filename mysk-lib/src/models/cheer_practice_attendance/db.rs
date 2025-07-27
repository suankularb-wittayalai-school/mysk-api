use chrono::{DateTime, Utc};
use mysk_lib_macros::GetById;
use serde::Deserialize;
use sqlx::{PgConnection, prelude::FromRow, query_scalar};
use uuid::Uuid;

use crate::{models::enums::CheerPracticeAttendanceType, prelude::*};

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

impl DbCheerPracticeAttendance {
    pub async fn get_by_student_id(conn: &mut PgConnection, student_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query_scalar!(
            "SELECT id FROM cheer_practice_attendances WHERE student_id = $1",
            student_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(res)
    }

    pub async fn get_by_classroom_id(
        conn: &mut PgConnection,
        practice_period_id: Uuid,
        classroom_id: Uuid,
    ) -> Result<Vec<Uuid>> {
        let res = query_scalar!(
            "\
            SELECT a.id \
            FROM cheer_practice_attendances AS a \
                JOIN classroom_students AS c ON c.student_id = a.student_id \
            WHERE a.practice_period_id = $1 AND c.classroom_id = $2 \
            ORDER BY c.student_id\
            ",
            practice_period_id,
            classroom_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(res)
    }
}
