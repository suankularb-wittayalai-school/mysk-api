use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum SortableClub {
    #[default]
    Id,
    Year,
}

impl Display for SortableClub {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableClub::Id => write!(f, "id"),
            SortableClub::Year => write!(f, "year"),
        }
    }
}
