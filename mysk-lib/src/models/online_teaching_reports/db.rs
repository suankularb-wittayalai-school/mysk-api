use crate::{
    common::requests::FilterConfig,
    models::{
        online_teaching_reports::requests::{
            queryable::QueryableOnlineTeachingReports, sortable::SortableOnlineTeachingReports,
        },
        traits::QueryDb,
    },
    query::Queryable as _,
};
use chrono::{DateTime, NaiveDate, Utc};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder, prelude::FromRow};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT * FROM online_teaching_reports",
    count_query = "SELECT COUNT(*) FROM online_teaching_reports"
)]
pub struct DbOnlineTeachingReports {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub subject_id: Option<Uuid>,
    pub teacher_id: Uuid,
    pub classroom_id: Option<Uuid>,
    pub date: NaiveDate,
    pub teaching_methods: Vec<String>,
    pub teaching_topic: String,
    pub suggestions: Option<String>,
    pub absent_student_no: Option<String>,
    pub start_time: i64,
    pub duration: i64,
    pub has_image: bool,
    pub image_ext: Option<String>,
}

impl QueryDb<QueryableOnlineTeachingReports, SortableOnlineTeachingReports>
    for DbOnlineTeachingReports
{
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<QueryableOnlineTeachingReports>>,
    ) {
        if let Some(filter) = filter {
            if let Some(data) = filter.data {
                data.to_where_clause()
                    .append_into_query_builder(query_builder);
            }
        }
    }
}
