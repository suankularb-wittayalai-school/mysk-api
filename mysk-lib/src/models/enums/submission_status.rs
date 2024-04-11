use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgTypeInfo, PgValueRef},
    Postgres,
};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

impl sqlx::Type<Postgres> for SubmissionStatus {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("submission_status")
    }
}

impl sqlx::Encode<'_, Postgres> for SubmissionStatus {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s = self.to_string();
        <String as sqlx::Encode<Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for SubmissionStatus {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        match <String as sqlx::Decode<Postgres>>::decode(value)?.as_str() {
            "pending" => Ok(SubmissionStatus::Pending),
            "approved" => Ok(SubmissionStatus::Approved),
            "declined" => Ok(SubmissionStatus::Declined),
            s => Err(Box::new(sqlx::Error::Decode(
                format!("Unknown submission status: {}", s).into(),
            ))),
        }
    }
}
