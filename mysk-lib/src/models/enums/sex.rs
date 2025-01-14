use serde::{Deserialize, Serialize};
use sqlx::Type as SqlxType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, SqlxType)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "sex", rename_all = "snake_case")]
pub enum Sex {
    Male,
    Female,
    Other,
}

impl Display for Sex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sex::Male => write!(f, "male"),
            Sex::Female => write!(f, "female"),
            Sex::Other => write!(f, "other"),
        }
    }
}
