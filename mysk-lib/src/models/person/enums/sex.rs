use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgTypeInfo, PgValueRef},
    Postgres,
};
use std::fmt::{Display, Formatter};

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

impl sqlx::Type<Postgres> for Sex {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("sex")
    }
}

impl sqlx::Encode<'_, Postgres> for Sex {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s: String = self.to_string();
        <String as sqlx::Encode<Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for Sex {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        match <String as sqlx::Decode<Postgres>>::decode(value)?.as_str() {
            "male" => Ok(Sex::Male),
            "female" => Ok(Sex::Female),
            _ => Err(Box::new(sqlx::Error::Decode("Unknown sex".into()))),
        }
    }
}
