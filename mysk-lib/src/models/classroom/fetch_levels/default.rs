use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{classroom::db::DbClassroom, contact::Contact, student::Student};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultClassroom {
    pub id: Uuid,
    pub number: i64,
    pub room: String,
    // pub class_advisor: Vec<Teacher>, // TODO: Change to Teacher
    pub students: Vec<Student>,
    pub contacts: Vec<Contact>,
    pub year: i64,
}

// TODO add implentation for DefaultClassroom
