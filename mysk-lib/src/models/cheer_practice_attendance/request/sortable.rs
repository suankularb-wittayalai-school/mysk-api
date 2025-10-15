use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortableCheerPracticeAttendance {
    Id,
    CreatedAt,
}

impl Default for SortableCheerPracticeAttendance {
    fn default() -> Self {
        Self::Id
    }
}

impl Display for SortableCheerPracticeAttendance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableCheerPracticeAttendance::Id => write!(f, "id"),
            SortableCheerPracticeAttendance::CreatedAt => write!(f, "created_at"),
        }
    }
}
