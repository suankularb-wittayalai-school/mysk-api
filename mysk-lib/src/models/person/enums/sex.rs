use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Sex {
    Male,
    Female,
}

impl Display for Sex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sex::Male => write!(f, "male"),
            Sex::Female => write!(f, "female"),
        }
    }
}

impl Type<sqlx::Postgres> for Sex {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("sex")
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for Sex {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s: String = self.to_string();
        <String as sqlx::Encode<sqlx::Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Sex {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let s: String = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match s.as_str() {
            "male" => Ok(Sex::Male),
            "female" => Ok(Sex::Female),
            _ => Err(Box::new(sqlx::Error::Decode("Unknown sex".into()))),
        }
    }
}
