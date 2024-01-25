use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use sqlx::Type;

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

impl Type<sqlx::Postgres> for UserRole {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("user_role")
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for UserRole {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s: String = self.to_string();
        <String as sqlx::Encode<sqlx::Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for UserRole {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let s: String = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match s.as_str() {
            "student" => Ok(UserRole::Student),
            "teacher" => Ok(UserRole::Teacher),
            "organization" => Ok(UserRole::Organization),
            "staff" => Ok(UserRole::Staff),
            "management" => Ok(UserRole::Management),
            _ => Err(Box::new(sqlx::Error::Decode("Unknown user role".into()))),
        }
    }
}
