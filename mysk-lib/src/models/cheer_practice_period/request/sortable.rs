use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum SortableCheerPracticePeriod {
    #[default]
    Id,
    Date,
    StartTime,
}

impl Display for SortableCheerPracticePeriod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableCheerPracticePeriod::Id => write!(f, "id"),
            SortableCheerPracticePeriod::Date => write!(f, "date"),
            SortableCheerPracticePeriod::StartTime => write!(f, "start_time"),
        }
    }
}
