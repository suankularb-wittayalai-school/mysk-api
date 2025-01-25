use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortableOnlineTeachingReports {
    Id,
    Date,
}

impl Display for SortableOnlineTeachingReports {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableOnlineTeachingReports::Id => write!(f, "id"),
            SortableOnlineTeachingReports::Date => write!(f, "date"),
        }
    }
}
