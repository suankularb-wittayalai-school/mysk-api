use crate::{
    common::requests::FilterConfig,
    models::{
        elective_trade_offer::request::{
            queryable::QueryableElectiveTradeOffer, sortable::SortableElectiveTradeOffer,
        },
        enums::SubmissionStatus,
        traits::QueryDb,
    },
    query::Queryable as _,
};
use chrono::{DateTime, Utc};
use mysk_lib_macros::GetById;
use serde::Deserialize;
use sqlx::{FromRow, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, FromRow, GetById)]
#[from_query(
    query = "SELECT * FROM elective_subject_trade_offers",
    count_query = "SELECT COUNT(*) FROM elective_subject_trade_offers"
)]
pub struct DbElectiveTradeOffer {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub status: SubmissionStatus,
    pub sender_elective_subject_session_id: Uuid,
    pub receiver_elective_subject_session_id: Uuid,
}

impl QueryDb<QueryableElectiveTradeOffer, SortableElectiveTradeOffer> for DbElectiveTradeOffer {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<QueryableElectiveTradeOffer>>,
    ) {
        if let Some(filter) = filter {
            if let Some(data) = filter.data {
                data.to_where_clause()
                    .append_into_query_builder(query_builder);
            }
        }
    }
}
