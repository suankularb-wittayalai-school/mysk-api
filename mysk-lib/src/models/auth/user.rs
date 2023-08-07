use actix_web::error::{ErrorNotFound, ErrorUnauthorized};
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpRequest};
// use anyhow::Ok;
use async_trait::async_trait;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use utoipa::openapi::schema;
use utoipa::ToSchema;
use uuid::Uuid;

use std::pin::Pin;

#[derive(Debug, ToSchema)]
pub enum UserRoles {
    Teacher,
    Student,
}

impl UserRoles {
    pub fn to_string(&self) -> String {
        match self {
            UserRoles::Teacher => "\"teacher\"".to_string(),
            UserRoles::Student => "\"student\"".to_string(),
        }
    }
    pub fn from_string(role: &str) -> UserRoles {
        match role {
            "\"teacher\"" => UserRoles::Teacher,
            "\"student\"" => UserRoles::Student,
            _ => UserRoles::Student,
        }
    }

    fn new(role: &str) -> UserRoles {
        self::UserRoles::from_string(role)
    }
}

// implement serialization and deserialization for UserRoles enum using from and to string
impl Serialize for UserRoles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UserRoles {
    fn deserialize<D>(deserializer: D) -> Result<UserRoles, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(UserRoles::from_string(&s))
    }
}
