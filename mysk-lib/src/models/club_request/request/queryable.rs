use crate::{
    models::enums::SubmissionStatus,
    query::{QueryParam, Queryable, SqlWhereClause},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableClubRequest {
    pub ids: Option<Vec<Uuid>>,
    pub club_ids: Option<Vec<Uuid>>,
    pub student_ids: Option<Vec<Uuid>>,
    pub membership_status: Option<SubmissionStatus>,
    pub year: Option<i64>,
}

impl Queryable for QueryableClubRequest {
    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.club_ids, |mut f, club_ids| {
            f.push_sql("club_id = ANY(")
                .push_param(QueryParam::ArrayUuid(club_ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.student_ids, |mut f, student_ids| {
            f.push_sql("student_id = ANY(")
                .push_param(QueryParam::ArrayUuid(student_ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.membership_status, |mut f, membership_status| {
            f.push_sql("membership_status = ")
                .push_param(QueryParam::SubmissionStatus(membership_status));

            f
        })
        .push_if_some(self.year, |mut f, year| {
            f.push_sql("year = ").push_param(QueryParam::Int(year));

            f
        });

        wc
    }
}
