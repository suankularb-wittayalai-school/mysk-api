use crate::{
    models::student::db::DbStudent,
    query::{QueryParam, Queryable, SqlWhereClause},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableStudent {
    pub ids: Option<Vec<Uuid>>,
    pub student_ids: Option<Vec<String>>,
    pub person_ids: Option<Vec<Uuid>>,
    pub user_ids: Option<Vec<Uuid>>,
}

impl Queryable for QueryableStudent {
    type Relation = DbStudent;

    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.student_ids, |mut f, student_ids| {
            f.push_sql("student_id = ANY(")
                .push_param(QueryParam::ArrayString(student_ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.person_ids, |mut f, person_ids| {
            f.push_sql("person_id IN (SELECT id FROM people WHERE id = ANY(")
                .push_param(QueryParam::ArrayUuid(person_ids))
                .push_sql("))");

            f
        })
        .push_if_some(self.user_ids, |mut f, user_ids| {
            f.push_sql("user_id IN (SELECT id FROM users WHERE id = ANY(")
                .push_param(QueryParam::ArrayUuid(user_ids))
                .push_sql("))");

            f
        });

        wc
    }
}
