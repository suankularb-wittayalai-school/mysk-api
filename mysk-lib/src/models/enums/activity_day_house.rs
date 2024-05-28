use serde::{Deserialize, Serialize};
use sqlx::Type as SqlxType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, SqlxType)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "activity_day_houses", rename_all = "snake_case")]
pub enum ActivityDayHouse {
    Cornicula,
    Cyprinus,
    Felis,
    Sciurus,
}

impl Display for ActivityDayHouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivityDayHouse::Cornicula => write!(f, "cornicula"),
            ActivityDayHouse::Cyprinus => write!(f, "cyprinus"),
            ActivityDayHouse::Felis => write!(f, "felis"),
            ActivityDayHouse::Sciurus => write!(f, "sciurus"),
        }
    }
}
