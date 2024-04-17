use serde::{Deserialize, Serialize};
use sqlx::Type as SqlxType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, SqlxType)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "submission_status", rename_all = "snake_case")]
pub enum SubmissionStatus {
    Pending,
    Approved,
    Declined,
}

impl Display for SubmissionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubmissionStatus::Pending => write!(f, "pending"),
            SubmissionStatus::Approved => write!(f, "approved"),
            SubmissionStatus::Declined => write!(f, "declined"),
        }
    }
}
