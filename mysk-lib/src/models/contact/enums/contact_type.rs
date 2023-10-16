use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ContactType {
    Phone,
    Email,
    Facebook,
    Line,
    Instagram,
    Website,
    Other,
}

use ContactType as CT;

impl Display for CT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CT::Phone => write!(f, "phone"),
            CT::Email => write!(f, "email"),
            CT::Facebook => write!(f, "facebook"),
            CT::Line => write!(f, "line"),
            CT::Instagram => write!(f, "instagram"),
            CT::Website => write!(f, "website"),
            CT::Other => write!(f, "other"),
        }
    }
}

impl Type<sqlx::Postgres> for CT {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("contact_types")
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for CT {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s: String = self.to_string();
        <String as sqlx::Encode<sqlx::Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for CT {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let s: String = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match s.as_str() {
            "phone" => Ok(CT::Phone),
            "email" => Ok(CT::Email),
            "facebook" => Ok(CT::Facebook),
            "line" => Ok(CT::Line),
            "instagram" => Ok(CT::Instagram),
            "website" => Ok(CT::Website),
            "other" => Ok(CT::Other),

            _ => Err(Box::new(sqlx::Error::Decode("Unknown contact type".into()))),
        }
    }
}
