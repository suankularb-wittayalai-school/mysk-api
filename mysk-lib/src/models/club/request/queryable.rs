use crate::{
    helpers::date::get_current_academic_year,
    query::{QueryParam, Queryable, SqlWhereClause},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableClub {
    pub ids: Option<Vec<Uuid>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub contact_ids: Option<Vec<Uuid>>,
    pub member_ids: Option<Vec<Uuid>>,
    pub staff_ids: Option<Vec<Uuid>>,
}

impl Queryable for QueryableClub {
    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new_empty();
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
        .push_if_some(self.description, |mut f, description| {
            f.push_sql("(description_th ILIKE ('%' || ")
                .push_param(QueryParam::String(description))
                .push_sql(" || '%') OR description_en ILIKE ('%' || ")
                .push_prev_param()
                .push_sql(" || '%'))");

            f
        })
        .push_if_some(self.contact_ids, |mut f, contact_ids| {
            f.push_sql("id IN (SELECT club_id FROM club_contacts WHERE contact_id = ANY(")
                .push_param(QueryParam::ArrayUuid(contact_ids))
                .push_sql("))");

            f
        })
        .push_if_some(self.member_ids, |mut f, member_ids| {
            f.push_sql("id (SELECT club_id FROM club_members WHERE student_id = ANY(")
                .push_param(QueryParam::ArrayUuid(member_ids))
                .push_sql(
                    "
                    ) AND membership_status = 'approved' AND year = get_current_academic_year(\
                    CAST(NOW() as DATE)))
                    ",
                );

            f
        })
        .push_if_some(self.staff_ids, |mut f, staff_ids| {
            f.push_sql("id IN (SELECT club_id FROM club_staffs WHERE student_id = ANY(")
                .push_param(QueryParam::ArrayUuid(staff_ids))
                .push_sql(") AND year = ")
                .push_param(QueryParam::Int(get_current_academic_year(None)));

            f
        });

        wc
    }
}
