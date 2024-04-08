use chrono::{DateTime, Utc};
use sqlx::Row;
use sqlx::{query, Execute, QueryBuilder};
use uuid::Uuid;

use crate::models::common::requests::{
    FilterConfig, PaginationConfig, QueryParam, SortingConfig, SqlSection,
};
use crate::models::common::response::PaginationType;
use crate::models::common::traits::{QueryDb, Queryable};
use crate::prelude::*;
use crate::{
    helpers::date::get_current_academic_year, models::subject::enums::subject_type::SubjectType,
};

use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{self, BaseQuery, GetById};

use super::request::queryable::QueryableElectiveSubject;
use super::request::sortable::SortableElectiveSubject;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT * FROM complete_elective_subjects_view",
    count_query = "SELECT COUNT(*) FROM complete_elective_subjects_view"
)]
pub struct DbElectiveSubject {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub subject_id: Uuid,
    pub cap_size: i64,
    pub class_size: i64,
    pub room: String,
    pub name_th: String,
    pub name_en: String,
    pub code_th: String,
    pub code_en: String,
    pub short_name_th: Option<String>,
    pub short_name_en: Option<String>,
    pub r#type: SubjectType,
    pub credit: f64,
    pub description_th: Option<String>,
    pub description_en: Option<String>,
    pub semester: Option<i64>,
    pub subject_group_id: i64,
    pub syllabus: Option<String>,
}

impl DbElectiveSubject {
    pub async fn get_subject_applicable_classrooms(
        &self,
        pool: &sqlx::PgPool,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            r#"SELECT classroom_id FROM elective_subject_classrooms WHERE elective_subject_id = $1"#,
            self.id
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.classroom_id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbElectiveSubject::get_subject_applicable_classrooms".to_string(),
            )),
        }
    }

    pub async fn get_enrolled_students(
        &self,
        pool: &sqlx::PgPool,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            r#"SELECT student_id FROM student_elective_subjects WHERE elective_subject_id = $1 and year = $2"#,
            self.id,
            academic_year.unwrap_or_else(|| get_current_academic_year(None))
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.student_id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbElectiveSubject::get_enrolled_students".to_string(),
            )),
        }
    }
}

impl QueryDb<QueryableElectiveSubject, SortableElectiveSubject> for DbElectiveSubject {
    async fn query(
        pool: &sqlx::PgPool,
        filter: Option<&FilterConfig<QueryableElectiveSubject>>,
        sort: Option<&SortingConfig<SortableElectiveSubject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        let mut query = QueryBuilder::<'_, sqlx::Postgres>::new(DbElectiveSubject::base_query());

        let mut where_sections: Vec<SqlSection> = Vec::new();

        if let Some(filter) = filter {
            if let Some(q) = &filter.q {
                // (name_th ILIKE '%q%' OR name_en ILIKE '%q%' OR code_th ILIKE '%q%' OR code_en ILIKE '%q%')
                where_sections.push(SqlSection {
                    sql: vec![
                        "(name_th ILIKE concat('%', ".to_string(),
                        ", '%') OR name_en ILIKE concat('%', ".to_string(),
                        ", '%') OR code_th ILIKE concat('%', ".to_string(),
                        ", '%') OR code_en ILIKE concat('%', ".to_string(),
                        ", '%'))".to_string(),
                    ],
                    params: vec![
                        QueryParam::String(q.to_string()),
                        QueryParam::String(q.to_string()),
                        QueryParam::String(q.to_string()),
                        QueryParam::String(q.to_string()),
                    ],
                });
            }
            if let Some(data) = &filter.data {
                let mut data_sections = data.to_query_string();
                where_sections.append(&mut data_sections);
            }
        }

        for (i, section) in where_sections.iter().enumerate() {
            // add WHERE or AND before each section
            query.push(if i == 0 { " WHERE " } else { "AND " });
            // len of sql should be len of params + 1
            // loop through index of sql
            //   push sql[i]
            //   if i < len of params
            //     push params[i]
            for (j, sql) in section.sql.iter().enumerate() {
                query.push(sql);
                if j < section.params.len() {
                    match &section.params[j] {
                        QueryParam::Int(v) => query.push_bind(v),
                        QueryParam::Float(v) => query.push_bind(v),
                        QueryParam::String(v) => query.push_bind(v),
                        QueryParam::Bool(v) => query.push_bind(v),
                        QueryParam::Uuid(v) => query.push_bind(v),
                        QueryParam::ArrayInt(v) => query.push_bind(v),
                        QueryParam::ArrayFloat(v) => query.push_bind(v),
                        QueryParam::ArrayString(v) => query.push_bind(v),
                        QueryParam::ArrayBool(v) => query.push_bind(v),
                        QueryParam::ArrayUuid(v) => query.push_bind(v),
                    };
                }
            }
        }

        if let Some(sorting) = sort {
            query.push(" ORDER BY ");
            let columns = sorting
                .by
                .clone()
                .into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>();
            query.push(columns.join(", "));

            if sorting.ascending.unwrap_or(true) {
                query.push(" ASC");
            } else {
                query.push(" DESC");
            }
        }

        if let Some(pagination) = pagination {
            let limit_section = pagination.to_limit_clause();
            // dbg!(&limit_section);
            query.push(" ");
            for (i, sql) in limit_section.sql.iter().enumerate() {
                query.push(sql);
                if i < limit_section.params.len() {
                    match limit_section.params[i] {
                        QueryParam::Int(v) => query.push_bind(v),
                        _ => {
                            return Err(Error::InternalSeverError(
                                "Invalid pagination params".to_string(),
                                "DbElectiveSubject::query".to_string(),
                            ));
                        }
                    };
                }
            }
        }

        query
            .build_query_as::<DbElectiveSubject>()
            .fetch_all(pool)
            .await
            .map_err(|e| {
                Error::InternalSeverError(e.to_string(), "DbElectiveSubject::query".to_string())
            })
    }

    async fn response_pagination(
        pool: &sqlx::PgPool,
        filter: Option<&FilterConfig<QueryableElectiveSubject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType> {
        let mut query = QueryBuilder::<'_, sqlx::Postgres>::new(DbElectiveSubject::count_query());

        let mut where_sections: Vec<SqlSection> = Vec::new();

        if let Some(filter) = filter {
            if let Some(q) = &filter.q {
                // (name_th ILIKE '%q%' OR name_en ILIKE '%q%' OR code_th ILIKE '%q%' OR code_en ILIKE '%q%')
                where_sections.push(SqlSection {
                    sql: vec![
                        "(name_th ILIKE concat('%', ".to_string(),
                        ", '%') OR name_en ILIKE concat('%', ".to_string(),
                        ", '%') OR code_th ILIKE concat('%', ".to_string(),
                        ", '%') OR code_en ILIKE concat('%', ".to_string(),
                        ", '%'))".to_string(),
                    ],
                    params: vec![
                        QueryParam::String(q.to_string()),
                        QueryParam::String(q.to_string()),
                        QueryParam::String(q.to_string()),
                        QueryParam::String(q.to_string()),
                    ],
                });
            }
            if let Some(data) = &filter.data {
                let mut data_sections = data.to_query_string();
                where_sections.append(&mut data_sections);
            }
        }

        for (i, section) in where_sections.iter().enumerate() {
            // add WHERE or AND before each section
            query.push(if i == 0 { " WHERE " } else { "AND " });
            // len of sql should be len of params + 1
            // loop through index of sql
            //   push sql[i]
            //   if i < len of params
            //     push params[i]
            for (j, sql) in section.sql.iter().enumerate() {
                query.push(sql);
                if j < section.params.len() {
                    match &section.params[j] {
                        QueryParam::Int(v) => query.push_bind(v),
                        QueryParam::Float(v) => query.push_bind(v),
                        QueryParam::String(v) => query.push_bind(v),
                        QueryParam::Bool(v) => query.push_bind(v),
                        QueryParam::Uuid(v) => query.push_bind(v),
                        QueryParam::ArrayInt(v) => query.push_bind(v),
                        QueryParam::ArrayFloat(v) => query.push_bind(v),
                        QueryParam::ArrayString(v) => query.push_bind(v),
                        QueryParam::ArrayBool(v) => query.push_bind(v),
                        QueryParam::ArrayUuid(v) => query.push_bind(v),
                    };
                }
            }
        }

        let count = query.build().fetch_one(pool).await.map_err(|e| {
            Error::InternalSeverError(
                e.to_string(),
                "DbElectiveSubject::response_pagination".to_string(),
            )
        })?;

        let count = count.get::<i64, _>("count");

        Ok(PaginationType::new(
            pagination.unwrap_or(&PaginationConfig::default()).p,
            pagination
                .unwrap_or(&PaginationConfig::default())
                .size
                .unwrap_or(50),
            count as u32,
        ))
    }
}
