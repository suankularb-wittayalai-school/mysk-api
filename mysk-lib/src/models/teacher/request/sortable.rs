use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum SortableTeacher {
    #[default]
    Id,
    SubjectGroupId,
}

impl Display for SortableTeacher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableTeacher::Id => write!(f, "id"),
            SortableTeacher::SubjectGroupId => write!(f, "subject_group_id"),
        }
    }
}
