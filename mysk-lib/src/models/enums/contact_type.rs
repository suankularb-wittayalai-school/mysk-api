use serde::{Deserialize, Serialize};
use sqlx::Type as SqlxType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, SqlxType)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "contact_types", rename_all = "snake_case")]
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

impl Display for ContactType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContactType::Phone => write!(f, "phone"),
            ContactType::Email => write!(f, "email"),
            ContactType::Facebook => write!(f, "facebook"),
            ContactType::Line => write!(f, "line"),
            ContactType::Instagram => write!(f, "instagram"),
            ContactType::Website => write!(f, "website"),
            ContactType::Discord => write!(f, "discord"),
            ContactType::Other => write!(f, "other"),
        }
    }
}
