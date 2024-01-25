use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum BloodGroup {
    #[serde(rename = "A+")]
    AP,
    #[serde(rename = "A-")]
    AN,
    #[serde(rename = "B+")]
    BP,
    #[serde(rename = "B-")]
    BN,
    #[serde(rename = "O+")]
    OP,
    #[serde(rename = "O-")]
    ON,
    #[serde(rename = "AB+")]
    ABP,
    #[serde(rename = "AB-")]
    ABN,
}

impl Display for BloodGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BloodGroup::AP => write!(f, "A+"),
            BloodGroup::AN => write!(f, "A-"),
            BloodGroup::BP => write!(f, "B+"),
            BloodGroup::BN => write!(f, "B-"),
            BloodGroup::OP => write!(f, "O+"),
            BloodGroup::ON => write!(f, "O-"),
            BloodGroup::ABP => write!(f, "AB+"),
            BloodGroup::ABN => write!(f, "AB-"),
        }
    }
}

impl Type<sqlx::Postgres> for BloodGroup {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("blood_group")
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for BloodGroup {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s: String = self.to_string();
        <String as sqlx::Encode<sqlx::Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for BloodGroup {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let s: String = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match s.as_str() {
            "A+" => Ok(BloodGroup::AP),
            "A-" => Ok(BloodGroup::AN),
            "B+" => Ok(BloodGroup::BP),
            "B-" => Ok(BloodGroup::BN),
            "O+" => Ok(BloodGroup::OP),
            "O-" => Ok(BloodGroup::ON),
            "AB+" => Ok(BloodGroup::ABP),
            "AB-" => Ok(BloodGroup::ABN),
            _ => Err(Box::new(sqlx::Error::Decode("Unknown blood group".into()))),
        }
    }
}
