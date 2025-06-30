use serde::{Deserialize, Serialize};
use sqlx::Type as SqlxType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, SqlxType)]
#[serde(rename_all = "snake_case")]
#[sqlx(
    type_name = "cheer_practice_attendance_type",
    rename_all = "snake_case"
)]
pub enum CheerPracticeAttendanceType {
    Present,
    Late,
    AbsentWithLeave,
    AbsentWithoutLeave,
    Deserted,
}

impl Display for CheerPracticeAttendanceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CheerPracticeAttendanceType::Present => write!(f, "present"),
            CheerPracticeAttendanceType::Late => write!(f, "late"),
            CheerPracticeAttendanceType::AbsentWithLeave => write!(f, "absent_with_leave"),
            CheerPracticeAttendanceType::AbsentWithoutLeave => write!(f, "absent_without_leave"),
            CheerPracticeAttendanceType::Deserted => write!(f, "deserted"),
        }
    }
}
