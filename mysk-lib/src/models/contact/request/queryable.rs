use crate::{
    models::{contact::db::DbContact, enums::ContactType},
    query::{QueryParam, Queryable, SqlWhereClause},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryableContact {
    pub ids: Option<Vec<Uuid>>,
    pub name: Option<String>,
    pub r#type: Option<ContactType>,
    pub value: Option<String>,
    pub classroom_ids: Option<Vec<Uuid>>,
    pub club_ids: Option<Vec<Uuid>>,
    pub student_ids: Option<Vec<Uuid>>,
    pub teacher_ids: Option<Vec<Uuid>>,
}

impl Queryable for QueryableContact {
    type Relation = DbContact;

    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.name, |mut f, name| {
            f.push_sql("(name_th ILIKE ('%' || ")
                .push_param(QueryParam::String(name))
                .push_sql(" || '%') OR name_en ILIKE ('%' || ")
                .push_prev_param()
                .push_sql(" || '%'))");

            f
        })
        .push_if_some(self.r#type, |mut f, r#type| {
            f.push_sql("type = ")
                .push_param(QueryParam::ContactType(r#type));

            f
        })
        .push_if_some(self.value, |mut f, value| {
            f.push_sql("value ILIKE ('%' || ")
                .push_param(QueryParam::String(value))
                .push_sql(" || '%')");

            f
        })
        .push_if_some(self.classroom_ids, |mut f, classroom_ids| {
            f.push_sql(
                "id IN (SELECT contact_id FROM classroom_contacts WHERE classroom_id = ANY(",
            )
            .push_param(QueryParam::ArrayUuid(classroom_ids))
            .push_sql("))");

            f
        })
        .push_if_some(self.club_ids, |mut f, club_ids| {
            f.push_sql("id IN (SELECT contact_id FROM club_contacts WHERE club_id = ANY(")
                .push_param(QueryParam::ArrayUuid(club_ids))
                .push_sql("))");

            f
        })
        .push_if_some(self.student_ids, |mut f, student_ids| {
            f.push_sql(
                "
                id IN (SELECT contact_id FROM person_contacts WHERE person_id = ANY(SELECT \
                person_id FROM students WHERE id = ANY(
                ",
            )
            .push_param(QueryParam::ArrayUuid(student_ids))
            .push_sql(")))");

            f
        })
        .push_if_some(self.teacher_ids, |mut f, teacher_ids| {
            f.push_sql(
                "
                id IN (SELECT contact_id FROM person_contacts WHERE person_id = ANY(SELECT \
                person_id FROM teachers WHERE id = ANY(
                ",
            )
            .push_param(QueryParam::ArrayUuid(teacher_ids))
            .push_sql(")))");

            f
        });

        wc
    }
}
