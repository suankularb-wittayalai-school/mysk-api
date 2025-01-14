use crate::{
    common::requests::FetchLevel,
    models::{
        elective_subject::ElectiveSubject,
        elective_trade_offer::db::DbElectiveTradeOffer,
        enums::SubmissionStatus,
        student::Student,
        traits::{FetchLevelVariant, TopLevelGetById as _},
    },
    permissions::Authorizer,
    prelude::*,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefaultElectiveTradeOffer {
    pub id: Uuid,
    pub sender: Student,
    pub receiver: Student,
    pub sender_elective_subject: ElectiveSubject,
    pub receiver_elective_subject: ElectiveSubject,
    pub status: SubmissionStatus,
}

#[async_trait]
impl FetchLevelVariant<DbElectiveTradeOffer> for DefaultElectiveTradeOffer {
    async fn from_table(
        pool: &PgPool,
        table: DbElectiveTradeOffer,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        Ok(Self {
            id: table.id,
            sender: Student::get_by_id(
                pool,
                table.sender_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            receiver: Student::get_by_id(
                pool,
                table.receiver_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            sender_elective_subject: ElectiveSubject::get_by_id(
                pool,
                table.sender_elective_subject_session_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            receiver_elective_subject: ElectiveSubject::get_by_id(
                pool,
                table.receiver_elective_subject_session_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            status: table.status,
        })
    }
}
