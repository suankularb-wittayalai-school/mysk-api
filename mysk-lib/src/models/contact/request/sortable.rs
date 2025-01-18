use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortableContact {
    Id,
    Name,
    Type,
    IncludeStudents,
    IncludeTeachers,
    IncludeParents,
}

impl Default for SortableContact {
    fn default() -> Self {
        Self::Id
    }
}

impl Display for SortableContact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableContact::Id => write!(f, "id"),
            SortableContact::Name => write!(f, "name"),
            SortableContact::Type => write!(f, "type"),
            SortableContact::IncludeStudents => write!(f, "include_students"),
            SortableContact::IncludeTeachers => write!(f, "include_teachers"),
            SortableContact::IncludeParents => write!(f, "include_parents"),
        }
    }
}
