use crate::{
    common::requests::{QueryParam, SqlSection},
    helpers::date::get_current_academic_year,
    models::traits::Queryable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableClub {
    pub ids: Option<Vec<Uuid>>,
    pub contact_ids: Option<Vec<Uuid>>,
    pub description: Option<String>,
    pub member_ids: Option<Vec<Uuid>>,
    pub name: Option<String>,
    pub staff_ids: Option<Vec<Uuid>>,
}

impl Queryable for QueryableClub {
    fn to_query_string(&self) -> Vec<SqlSection> {
        let mut where_sections = Vec::<SqlSection>::new();

        // WHERE id = ANY($1)
        if let Some(ids) = &self.ids {
            where_sections.push(SqlSection {
                sql: vec!["id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayUuid(ids.clone())],
            });
        }

        // WHERE id IN (SELECT club_id FROM club_contacts WHERE contact_id IN ANY($1))
        if let Some(contact_ids) = &self.contact_ids {
            where_sections.push(SqlSection {
                sql: vec![
                    "id IN (SELECT club_id FROM club_contacts WHERE contact_id = ANY(".to_string(),
                    "))".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(contact_ids.clone())],
            });
        }

        // WHERE description_th ILIKE $1 OR description_en ILIKE $1
        if let Some(description) = &self.description {
            where_sections.push(SqlSection {
                sql: vec![
                    "(description_th ILIKE ".to_string(),
                    " OR description_en ILIKE ".to_string(),
                    ")".to_string(),
                ],
                params: vec![
                    QueryParam::String(description.clone()),
                    QueryParam::String(description.clone()),
                ],
            });
        }

        // WHERE clubs.id IN (SELECT club_id FROM club_members WHERE student_id IN ANY($1) AND
        // membership_status = 'approved')
        if let Some(member_ids) = &self.member_ids {
            where_sections.push(SqlSection {
                sql: vec![
                    concat!(
                        "id IN (SELECT club_id FROM club_members WHERE",
                        " student_id = ANY(",
                    )
                    .to_string(),
                    ") AND membership_status = 'approved' AND year = get_current_academic_year(CAST(NOW() as DATE)))".to_string(),
                ],
                params: vec![QueryParam::ArrayUuid(member_ids.clone())],
            });
        }

        // WHERE name_th ILIKE $1 OR name_en ILIKE $1
        if let Some(name) = &self.name {
            where_sections.push(SqlSection {
                sql: vec![
                    "(name_th ILIKE ".to_string(),
                    " OR name_en ILIKE ".to_string(),
                    ")".to_string(),
                ],
                params: vec![
                    QueryParam::String(name.clone()),
                    QueryParam::String(name.clone()),
                ],
            });
        }

        // WHERE id IN (SELECT club_id FROM club_staffs WHERE student_id IN ANY($1) AND
        // year = {get_current_academic_year(None)})
        if let Some(staff_ids) = &self.staff_ids {
            where_sections.push(SqlSection {
                sql: vec![
                    "id IN (SELECT club_id FROM club_staffs WHERE student_id = ANY(".to_string(),
                    format!(") AND year = {})", get_current_academic_year(None)),
                ],
                params: vec![QueryParam::ArrayUuid(staff_ids.clone())],
            });
        }

        where_sections
    }
}
