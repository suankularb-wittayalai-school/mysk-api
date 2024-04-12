use crate::{
    common::requests::{QueryParam, SqlSection},
    models::traits::Queryable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableElectiveTradeOffer {}

impl Queryable for QueryableElectiveTradeOffer {
    fn to_query_string(&self) -> Vec<SqlSection> {
        let mut where_sections: Vec<SqlSection> = Vec::new();

        where_sections
    }
}
