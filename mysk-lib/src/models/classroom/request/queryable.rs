use crate::{
    models::classroom::db::DbClassroom,
    query::{QueryParam, Queryable, SqlWhereClause},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableClassroom {
    pub classroom_ids: Option<Vec<Uuid>>,
    pub year: Option<i64>,
}

impl Queryable for QueryableClassroom {
    type Relation = DbClassroom;

    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new();
        wc.push_if_some(self.classroom_ids, |mut f, classroom_ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(classroom_ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.year, |mut f, year| {
            f.push_sql("year = ")
                .push_param(QueryParam::Int(year));

            f
        });

        wc
    }
}
