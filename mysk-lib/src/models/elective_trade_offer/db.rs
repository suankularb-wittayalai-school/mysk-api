use chrono::{DateTime, Utc};
use uuid::Uuid;

use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};

use crate::models::common::enums::submission_status::SubmissionStatus;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow, BaseQuery, GetById)]
#[base_query(query = "SELECT * FROM elective_subject_trade_offers")]
pub struct DbElectiveSubject {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub sender_elective_subject_id: Uuid,
    pub receiver_elective_subject_id: Uuid,
    pub status: SubmissionStatus,
}
