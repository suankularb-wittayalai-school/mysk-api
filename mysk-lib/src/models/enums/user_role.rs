use serde::{Deserialize, Serialize};
use sqlx::Type as SqlxType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, SqlxType)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    Student,
    Teacher,
    Organization,
    Staff,
    Management,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Student => write!(f, "student"),
            UserRole::Teacher => write!(f, "teacher"),
            UserRole::Organization => write!(f, "organization"),
            UserRole::Staff => write!(f, "staff"),
            UserRole::Management => write!(f, "management"),
        }
    }
}
