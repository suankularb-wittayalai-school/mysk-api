use crate::{
    common::requests::{QueryParam, SqlSection},
    models::{enums::SubmissionStatus, traits::Queryable},
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
    fn to_query_string(&self) -> Vec<SqlSection> {
        let mut where_sections = Vec::<SqlSection>::new();

        // WHERE id = ANY($1)
        if let Some(ids) = &self.ids {
            where_sections.push(SqlSection {
                sql: vec!["id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayUuid(ids.clone())],
            });
        }

        // WHERE club_id = ANY($1)
        if let Some(club_ids) = &self.club_ids {
            where_sections.push(SqlSection {
                sql: vec!["club_id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayUuid(club_ids.clone())],
            });
        }

        // WHERE student_id = ANY($1)
        if let Some(student_ids) = &self.student_ids {
            where_sections.push(SqlSection {
                sql: vec!["student_id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayUuid(student_ids.clone())],
            });
        }

        // WHERE membership_status = $1
        if let Some(membership_status) = &self.membership_status {
            where_sections.push(SqlSection {
                sql: vec!["membership_status = ".to_string()],
                params: vec![QueryParam::SubmissionStatus(*membership_status)],
            });
        }

        // WHERE year = $1
        if let Some(year) = &self.year {
            where_sections.push(SqlSection {
                sql: vec!["year = ".to_string()],
                params: vec![QueryParam::Int(*year)],
            });
        }

        where_sections
    }
}
