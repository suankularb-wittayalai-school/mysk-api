use crate::{
    models::cheer_practice_attendance::db::DbCheerPracticeAttendance,
    query::{QueryParam, Queryable, SqlWhereClause},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryableCheerPracticeAttendance {
    pub ids: Option<Vec<Uuid>>,
    pub practice_period_id: Option<Uuid>,
    pub classroom_id: Option<Uuid>,
}

impl Queryable for QueryableCheerPracticeAttendance {
    type Relation = DbCheerPracticeAttendance;

    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.practice_period_id, |mut f, practice_period_id| {
            f.push_sql("practice_period_id = ")
                .push_param(QueryParam::Uuid(practice_period_id));

            f
        })
        .push_if_some(self.classroom_id, |mut f, classroom_id| {
            f.push_sql(
                "student_id IN (SELECT student_id FROM classroom_students WHERE classroom_id = ",
            )
            .push_param(QueryParam::Uuid(classroom_id))
            .push_sql(" ORDER BY classroom_id)");

            f
        });

        wc
    }
}
