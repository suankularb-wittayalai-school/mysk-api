use crate::{
    common::requests::FetchLevel,
    models::{elective_trade_offer::db::DbElectiveTradeOffer, traits::FetchLevelVariant},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyElectiveTradeOffer {
    pub id: Uuid,
}

impl_id_only_variant_from!(IdOnlyElectiveTradeOffer, DbElectiveTradeOffer);
