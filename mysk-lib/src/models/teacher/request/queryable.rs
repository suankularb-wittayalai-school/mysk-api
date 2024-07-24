use crate::{
    common::requests::{QueryParam, SqlSection},
    models::traits::Queryable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableTeacher {
    pub ids: Option<Vec<Uuid>>,
    // pub teacher_ids: Option<String>,
    pub subject_group_ids: Option<i64>,
    // pub person_ids: Option<Vec<Uuid>>,
    // pub user_ids: Option<Vec<Uuid>>,
}

impl Queryable for QueryableTeacher {
    fn to_query_string(&self) -> Vec<SqlSection> {
        let mut where_sections = Vec::<SqlSection>::new();

        // WHERE id = ANY($1)
        if let Some(ids) = &self.ids {
            where_sections.push(SqlSection {
                sql: vec!["id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayUuid(ids.clone())],
            });
        }

        // WHERE subject_group_id IN (SELECT id FROM subject_groups WHERE id IN ANY($1))
        if let Some(subject_group_ids) = &self.subject_group_ids {
            where_sections.push(SqlSection {
                sql: vec!["id IN (SELECT id FROM subject_groups WHERE id = ANY(".to_string()],
                params: vec![QueryParam::Int(subject_group_ids.clone())],
            })
        }

        where_sections
    }
}
