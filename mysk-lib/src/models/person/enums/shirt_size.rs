use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgTypeInfo, PgValueRef},
    Postgres,
};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum ShirtSize {
    XS,
    S,
    M,
    L,
    XL,
    #[serde(rename = "2XL")]
    X2L,
    #[serde(rename = "3XL")]
    X3L,
    #[serde(rename = "4XL")]
    X4L,
    #[serde(rename = "5XL")]
    X5L,
    #[serde(rename = "6XL")]
    X6L,
}

impl Display for ShirtSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShirtSize::XS => write!(f, "XS"),
            ShirtSize::S => write!(f, "S"),
            ShirtSize::M => write!(f, "M"),
            ShirtSize::L => write!(f, "L"),
            ShirtSize::XL => write!(f, "XL"),
            ShirtSize::X2L => write!(f, "2XL"),
            ShirtSize::X3L => write!(f, "3XL"),
            ShirtSize::X4L => write!(f, "4XL"),
            ShirtSize::X5L => write!(f, "5XL"),
            ShirtSize::X6L => write!(f, "6XL"),
        }
    }
}

impl sqlx::Type<Postgres> for ShirtSize {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("shirt_size")
    }
}

impl sqlx::Encode<'_, Postgres> for ShirtSize {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s: String = self.to_string();
        <String as sqlx::Encode<Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for ShirtSize {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        match <String as sqlx::Decode<Postgres>>::decode(value)?.as_str() {
            "XS" => Ok(ShirtSize::XS),
            "S" => Ok(ShirtSize::S),
            "M" => Ok(ShirtSize::M),
            "L" => Ok(ShirtSize::L),
            "XL" => Ok(ShirtSize::XL),
            "2XL" => Ok(ShirtSize::X2L),
            "3XL" => Ok(ShirtSize::X3L),
            "4XL" => Ok(ShirtSize::X4L),
            "5XL" => Ok(ShirtSize::X5L),
            "6XL" => Ok(ShirtSize::X6L),
            s => Err(Box::new(sqlx::Error::Decode(
                format!("Unknown shirt size: {}", s).into(),
            ))),
        }
    }
}
