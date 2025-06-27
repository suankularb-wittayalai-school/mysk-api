use crate::{
    common::requests::FetchLevel,
    models::{
        elective_subject::ElectiveSubject, elective_trade_offer::db::DbElectiveTradeOffer,
        enums::SubmissionStatus, student::Student, traits::FetchVariant,
    },
    permissions::Authorizer,
    prelude::*,
};
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

impl FetchVariant for DefaultElectiveTradeOffer {
    type Relation = DbElectiveTradeOffer;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        Ok(Self {
            id: relation.id,
            sender: Student::get_by_id(
                pool,
                relation.sender_id,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            receiver: Student::get_by_id(
                pool,
                relation.receiver_id,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            sender_elective_subject: ElectiveSubject::get_by_id(
                pool,
                relation.sender_elective_subject_session_id,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            receiver_elective_subject: ElectiveSubject::get_by_id(
                pool,
                relation.receiver_elective_subject_session_id,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            status: relation.status,
        })
    }
}
