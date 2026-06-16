use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum SortableCheerPracticeAttendance {
    #[default]
    Id,
    CreatedAt,
}

impl Display for SortableCheerPracticeAttendance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableCheerPracticeAttendance::Id => write!(f, "id"),
            SortableCheerPracticeAttendance::CreatedAt => write!(f, "created_at"),
        }
    }
}
