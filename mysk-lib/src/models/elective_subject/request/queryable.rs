use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableElectiveSubject {
    pub ids: Option<Vec<Uuid>>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub teacher_ids: Option<Vec<Uuid>>,
    pub co_teacher_ids: Option<Vec<Uuid>>,
    pub subject_group_id: Option<Vec<i64>>,
    pub credit: Option<f64>,
    pub is_full: Option<bool>,
    pub applicable_classroom_ids: Option<Vec<Uuid>>,
    pub room: Option<String>,
    pub student_ids: Option<Vec<Uuid>>,
}
