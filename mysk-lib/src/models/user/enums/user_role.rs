use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgTypeInfo, PgValueRef},
    Postgres,
};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Student,
    Teacher,
    Organization,
    Staff,
    Management,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Student => write!(f, "student"),
            UserRole::Teacher => write!(f, "teacher"),
            UserRole::Organization => write!(f, "organization"),
            UserRole::Staff => write!(f, "staff"),
            UserRole::Management => write!(f, "management"),
        }
    }
}

impl sqlx::Type<Postgres> for UserRole {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("user_role")
    }
}

impl sqlx::Encode<'_, Postgres> for UserRole {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s: String = self.to_string();
        <String as sqlx::Encode<Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for UserRole {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        match <String as sqlx::Decode<Postgres>>::decode(value)?.as_str() {
            "student" => Ok(UserRole::Student),
            "teacher" => Ok(UserRole::Teacher),
            "organization" => Ok(UserRole::Organization),
            "staff" => Ok(UserRole::Staff),
            "management" => Ok(UserRole::Management),
            _ => Err(Box::new(sqlx::Error::Decode("Unknown user role".into()))),
        }
    }
}
