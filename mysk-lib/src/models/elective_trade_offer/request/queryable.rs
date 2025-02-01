use crate::{
    models::enums::SubmissionStatus,
    query::{QueryParam, Queryable, SqlWhereClause},
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
    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.sender_ids, |mut f, sender_ids| {
            f.push_sql("sender_id = ANY(")
                .push_param(QueryParam::ArrayUuid(sender_ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.receiver_ids, |mut f, receiver_ids| {
            f.push_sql("receiver_id = ANY(")
                .push_param(QueryParam::ArrayUuid(receiver_ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.status, |mut f, status| {
            f.push_sql("status = ")
                .push_param(QueryParam::SubmissionStatus(status))
                .push_sql(")");

            f
        });

        wc
    }
}
