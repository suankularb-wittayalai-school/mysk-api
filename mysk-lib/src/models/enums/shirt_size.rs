use serde::{Deserialize, Serialize};
use sqlx::Type as SqlxType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, SqlxType)]
#[sqlx(type_name = "shirt_size")]
pub enum ShirtSize {
    XS,
    S,
    M,
    L,
    XL,
    #[serde(rename = "2XL")]
    #[sqlx(rename = "2XL")]
    X2L,
    #[serde(rename = "3XL")]
    #[sqlx(rename = "3XL")]
    X3L,
    #[serde(rename = "4XL")]
    #[sqlx(rename = "4XL")]
    X4L,
    #[serde(rename = "5XL")]
    #[sqlx(rename = "5XL")]
    X5L,
    #[serde(rename = "6XL")]
    #[sqlx(rename = "6XL")]
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

