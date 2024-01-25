use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::subject::db::DbSubject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlySubject {
    pub id: Uuid,
}

impl From<DbSubject> for IdOnlySubject {
    fn from(subject: DbSubject) -> Self {
        Self { id: subject.id }
    }
}
