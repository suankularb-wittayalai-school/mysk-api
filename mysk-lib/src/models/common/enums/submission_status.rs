use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, Postgres, Type};

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

impl Type<Postgres> for SubmissionStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("submission_status")
    }
}

impl Encode<'_, Postgres> for SubmissionStatus {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s = self.to_string();
        <String as sqlx::Encode<sqlx::Postgres>>::encode(s, buf)
    }
}

impl Decode<'_, Postgres> for SubmissionStatus {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as Decode<Postgres>>::decode(value)?;

        match s.as_str() {
            "pending" => Ok(SubmissionStatus::Pending),
            "approved" => Ok(SubmissionStatus::Approved),
            "declined" => Ok(SubmissionStatus::Declined),
            _ => Err(sqlx::error::BoxDynError::from(format!(
                "Unknown submission status: {}",
                s
            ))),
        }
    }
}
