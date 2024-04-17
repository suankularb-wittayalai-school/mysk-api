use serde::{Deserialize, Serialize};
use sqlx::Type as SqlxType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, SqlxType)]
#[sqlx(type_name = "blood_group")]
pub enum BloodGroup {
    #[serde(rename = "A+")]
    #[sqlx(rename = "A+")]
    AP,
    #[serde(rename = "A-")]
    #[sqlx(rename = "A-")]
    AN,
    #[serde(rename = "B+")]
    #[sqlx(rename = "B+")]
    BP,
    #[serde(rename = "B-")]
    #[sqlx(rename = "B-")]
    BN,
    #[serde(rename = "O+")]
    #[sqlx(rename = "O+")]
    OP,
    #[serde(rename = "O-")]
    #[sqlx(rename = "O-")]
    ON,
    #[serde(rename = "AB+")]
    #[sqlx(rename = "AB+")]
    ABP,
    #[serde(rename = "AB-")]
    #[sqlx(rename = "AB-")]
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
