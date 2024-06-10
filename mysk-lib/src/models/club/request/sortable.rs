use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortableClub {
    Id,
    House,
    MapLocation,
    Name,
}

impl Default for SortableClub {
    fn default() -> Self {
        Self::Id
    }
}

impl Display for SortableClub {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableClub::Id => write!(f, "id"),
            SortableClub::House => write!(f, "house"),
            SortableClub::MapLocation => write!(f, "map_location"),
            SortableClub::Name => write!(f, "name"),
        }
    }
}
