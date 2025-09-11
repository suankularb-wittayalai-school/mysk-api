use crate::{
    models::online_teaching_reports::db::DbOnlineTeachingReports,
    query::{QueryParam, Queryable, SqlWhereClause},
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableOnlineTeachingReports {
    pub ids: Option<Vec<Uuid>>,
    pub dates: Option<Vec<NaiveDate>>,
    pub as_teacher_id: Option<Uuid>,
}

impl Queryable for QueryableOnlineTeachingReports {
    type Relation = DbOnlineTeachingReports;

    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.dates, |mut f, dates| {
            f.push_sql("date = ANY(")
                .push_param(QueryParam::ArrayNaiveDate(dates))
                .push_sql(")");

            f
        })
        .push_if_some(self.as_teacher_id, |mut f, as_teacher_id| {
            f.push_sql("teacher_id = ")
                .push_param(QueryParam::Uuid(as_teacher_id));

            f
        });

        wc
    }
}
