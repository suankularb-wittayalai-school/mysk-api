use crate::{
    common::requests::{QueryParam, SqlSection},
    models::{enums::SubmissionStatus, traits::Queryable},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryableElectiveTradeOffer {
    pub ids: Option<Vec<Uuid>>,
    pub sender_ids: Option<Vec<Uuid>>,
    pub receiver_ids: Option<Vec<Uuid>>,
    pub status: Option<SubmissionStatus>,
}

impl Queryable for QueryableElectiveTradeOffer {
    fn to_query_string(&self) -> Vec<SqlSection> {
        let mut where_sections = Vec::<SqlSection>::new();

        // WHERE id = ANY($1)
        if let Some(ids) = &self.ids {
            where_sections.push(SqlSection {
                sql: vec!["id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayUuid(ids.clone())],
            });
        }

        // WHERE sender_id = ANY($1)
        if let Some(sender_ids) = &self.sender_ids {
            where_sections.push(SqlSection {
                sql: vec!["sender_id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayUuid(sender_ids.clone())],
            });
        }

        // WHERE receiver_id = ANY($1)
        if let Some(receiver_ids) = &self.receiver_ids {
            where_sections.push(SqlSection {
                sql: vec!["receiver_id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayUuid(receiver_ids.clone())],
            });
        }

        // WHERE status = $1
        if let Some(status) = &self.status {
            where_sections.push(SqlSection {
                sql: vec!["status = ".to_string()],
                params: vec![QueryParam::SubmissionStatus(*status)],
            });
        }

        where_sections
    }
}
