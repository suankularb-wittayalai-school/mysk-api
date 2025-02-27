use serde::{Deserialize, Serialize};
use sqlx::Type as SqlxType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, SqlxType)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "submission_status", rename_all = "snake_case")]
pub enum CertificateType {
    StudentOfTheYear,
    ExcellentStudent,
    Academic,
    Morale,
    Sports,
    Activity,
}

impl Display for CertificateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CertificateType::StudentOfTheYear => write!(f, "student_of_the_year"),
            CertificateType::ExcellentStudent => write!(f, "excellent_student"),
            CertificateType::Academic => write!(f, "academic"),
            CertificateType::Morale => write!(f, "morale"),
            CertificateType::Sports => write!(f, "sports"),
            CertificateType::Activity => write!(f, "activity"),
        }
    }
}
