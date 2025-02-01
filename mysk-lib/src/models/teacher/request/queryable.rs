use crate::query::{QueryParam, Queryable, SqlWhereClause};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableTeacher {
    pub ids: Option<Vec<Uuid>>,
    pub subject_group_ids: Option<Vec<i64>>,
    pub person_ids: Option<Vec<Uuid>>,
    pub user_ids: Option<Vec<Uuid>>,
}

impl Queryable for QueryableTeacher {
    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.subject_group_ids, |mut f, subject_group_ids| {
            f.push_sql("subject_group_id IN (SELECT id FROM subject_groups WHERE id = ANY(")
                .push_param(QueryParam::ArrayInt(subject_group_ids))
                .push_sql("))");

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
