use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    common::{
        enums::submission_status::SubmissionStatus,
        requests::FetchLevel,
        traits::{FetchLevelVariant, TopLevelGetById},
    },
    elective_subject::ElectiveSubject,
    elective_trade_offer::db::DbElectiveTradeOffer,
    student::Student,
};
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultElectiveTradeOffer {
    pub id: Uuid,
    pub sender: Student,
    pub receiver: Student,
    pub sender_elective_subject: ElectiveSubject,
    pub receiver_elective_subject: ElectiveSubject,
    pub status: SubmissionStatus,
}

impl FetchLevelVariant<DbElectiveTradeOffer> for DefaultElectiveTradeOffer {
    async fn from_table(
        pool: &PgPool,
        table: DbElectiveTradeOffer,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        Ok(Self {
            id: table.id,
            sender: Student::get_by_id(
                pool,
                table.sender_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
            )
            .await?,
            receiver: Student::get_by_id(
                pool,
                table.receiver_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
            )
            .await?,
            sender_elective_subject: ElectiveSubject::get_by_id(
                pool,
                table.sender_elective_subject_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
            )
            .await?,
            receiver_elective_subject: ElectiveSubject::get_by_id(
                pool,
                table.receiver_elective_subject_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
            )
            .await?,
            status: table.status,
        })
    }
}
