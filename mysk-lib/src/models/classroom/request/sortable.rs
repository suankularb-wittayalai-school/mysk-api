use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum SortableClassroom {
    #[default]
    Id,
    Number,
}

impl Display for SortableClassroom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableClassroom::Id => write!(f, "id"),
            SortableClassroom::Number => write!(f, "number"),
        }
    }
}
