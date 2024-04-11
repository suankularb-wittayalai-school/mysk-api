use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgTypeInfo, PgValueRef},
    Postgres,
};
use std::fmt::{Display, Formatter};

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

impl sqlx::Type<Postgres> for SubjectType {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("subject_type_en_enum")
    }
}

impl sqlx::Encode<'_, Postgres> for SubjectType {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s: String = self.to_string();
        <String as sqlx::Encode<Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for SubjectType {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        match <String as sqlx::Decode<Postgres>>::decode(value)?.as_str() {
            "core_course" => Ok(SubjectType::CoreCourse),
            "additional_course" => Ok(SubjectType::AdditionalCourse),
            "elective" => Ok(SubjectType::ElectiveCourse),
            "learners_development_activities" => Ok(SubjectType::LearnersDevelopmentActivities),

            _ => Err(Box::new(sqlx::Error::Decode("Unknown subject type".into()))),
        }
    }
}
