use serde::{Deserialize, Serialize};
use sqlx::Type as SqlxType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, SqlxType)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "subject_type_en_enum", rename_all = "snake_case")]
pub enum SubjectType {
    CoreCourse,
    AdditionalCourse,
    ElectiveCourse,
    LearnersDevelopmentActivities,
}

impl Display for SubjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubjectType::CoreCourse => write!(f, "core_course"),
            SubjectType::AdditionalCourse => write!(f, "additional_course"),
            SubjectType::ElectiveCourse => write!(f, "elective"),
            SubjectType::LearnersDevelopmentActivities => {
                write!(f, "learners_development_activities")
            }
        }
    }
}
