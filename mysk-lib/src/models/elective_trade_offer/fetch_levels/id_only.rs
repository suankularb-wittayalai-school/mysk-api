use crate::{
    models::{
        common::{requests::FetchLevel, traits::FetchLevelVariant},
        elective_trade_offer::db::DbElectiveTradeOffer,
    },
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
