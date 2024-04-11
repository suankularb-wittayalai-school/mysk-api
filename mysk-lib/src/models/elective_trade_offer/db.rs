use crate::models::enums::submission_status::SubmissionStatus;
use chrono::{DateTime, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(query = "SELECT * FROM elective_subject_trade_offers")]
pub struct DbElectiveTradeOffer {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub sender_elective_subject_id: Uuid,
    pub receiver_elective_subject_id: Uuid,
    pub status: SubmissionStatus,
}
