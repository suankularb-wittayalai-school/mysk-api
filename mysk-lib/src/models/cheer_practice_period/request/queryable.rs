use crate::{
    models::cheer_practice_period::db::DbCheerPracticePeriod,
    query::{QueryParam, Queryable, SqlWhereClause},
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryableCheerPracticePeriod {
    pub ids: Option<Vec<Uuid>>,
    pub date: Option<NaiveDate>,
}

impl Queryable for QueryableCheerPracticePeriod {
    type Relation = DbCheerPracticePeriod;

    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new_empty();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.date, |mut f, date| {
            f.push_sql("date = ")
                .push_param(QueryParam::NaiveDate(date));

            f
        });

        wc
    }
}
