use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortableStudent {
    Id,
    StudentId,
}

impl Default for SortableStudent {
    fn default() -> Self {
        Self::Id
    }
}

impl Display for SortableStudent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableStudent::Id => write!(f, "id"),
            SortableStudent::StudentId => write!(f, "student_id"),
        }
    }
}
