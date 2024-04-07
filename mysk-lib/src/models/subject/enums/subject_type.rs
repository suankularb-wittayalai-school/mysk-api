use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
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

impl Type<sqlx::Postgres> for SubjectType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("subject_type_en_enum")
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for SubjectType {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s: String = self.to_string();
        <String as sqlx::Encode<sqlx::Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for SubjectType {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let s: String = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match s.as_str() {
            "core_course" => Ok(SubjectType::CoreCourse),
            "additional_course" => Ok(SubjectType::AdditionalCourse),
            "elective" => Ok(SubjectType::ElectiveCourse),
            "learners_development_activities" => Ok(SubjectType::LearnersDevelopmentActivities),

            _ => Err(Box::new(sqlx::Error::Decode("Unknown subject type".into()))),
        }
    }
}
