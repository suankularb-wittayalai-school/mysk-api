use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgTypeInfo, PgValueRef},
    Postgres,
};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ContactType {
    Phone,
    Email,
    Facebook,
    Line,
    Instagram,
    Website,
    Discord,
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
            CT::Discord => write!(f, "discord"),
            CT::Other => write!(f, "other"),
        }
    }
}

impl sqlx::Type<Postgres> for CT {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("contact_types")
    }
}

impl sqlx::Encode<'_, Postgres> for CT {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s: String = self.to_string();
        <String as sqlx::Encode<Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for CT {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        match <String as sqlx::Decode<Postgres>>::decode(value)?.as_str() {
            "phone" => Ok(CT::Phone),
            "email" => Ok(CT::Email),
            "facebook" => Ok(CT::Facebook),
            "line" => Ok(CT::Line),
            "instagram" => Ok(CT::Instagram),
            "website" => Ok(CT::Website),
            "discord" => Ok(CT::Discord),
            "other" => Ok(CT::Other),
            _ => Err(Box::new(sqlx::Error::Decode("Unknown contact type".into()))),
        }
    }
}
