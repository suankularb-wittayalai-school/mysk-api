use crate::{
    common::requests::{QueryParam, SqlSection},
    models::traits::Queryable,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableOnlineTeachingReports {
    pub ids: Option<Vec<Uuid>>,
    pub dates: Option<Vec<NaiveDate>>,
    pub as_teacher_id: Option<Uuid>,
}

impl Queryable for QueryableOnlineTeachingReports {
    fn to_query_string(&self) -> Vec<SqlSection> {
        let mut where_sections = Vec::<SqlSection>::new();

        // WHERE id = ANY($1)
        if let Some(ids) = &self.ids {
            where_sections.push(SqlSection {
                sql: vec!["id = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayUuid(ids.clone())],
            });
        }

        // WHERE date = ANY($1)
        if let Some(dates) = &self.dates {
            where_sections.push(SqlSection {
                sql: vec!["date = ANY(".to_string(), ")".to_string()],
                params: vec![QueryParam::ArrayNaiveDate(dates.clone())],
            });
        }

        // WHERE teacher_id = $1
        if let Some(as_teacher_id) = &self.as_teacher_id {
            where_sections.push(SqlSection {
                sql: vec!["teacher_id = ".to_string()],
                params: vec![QueryParam::Uuid(as_teacher_id.clone())],
            });
        }

        where_sections
    }
}
