use super::request::{queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject};
use crate::{
    common::{
        requests::{FilterConfig, PaginationConfig, QueryParam, SortingConfig, SqlSection},
        response::PaginationType,
        string::MultiLangString,
    },
    helpers::date::get_current_academic_year,
    models::{
        student::db::DbStudent,
        subject::enums::subject_type::SubjectType,
        traits::{QueryDb, Queryable as _},
    },
    prelude::*,
};
use chrono::{DateTime, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool, Postgres, QueryBuilder, Row as _};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
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
    pub session_code: i64,
}

impl DbElectiveSubject {
    pub async fn get_by_session_code(pool: &PgPool, session_code: i64) -> Result<Option<Self>> {
        query_as::<_, DbElectiveSubject>(
            r"
            SELECT * FROM complete_elective_subjects_view
            WHERE session_code = $1
            ",
        )
        .bind(session_code)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            Error::InternalSeverError(
                e.to_string(),
                "DbElectiveSubject::get_by_session_code".to_string(),
            )
        })
    }

    /// # Get elective subject by ID with student context
    /// This function is the extension of the `get_by_id` function. Since an elective subject can
    /// be enrolled by students in different classrooms and taught in different sessions, this
    /// function will return the elective subject object which is available for the student which
    /// will always be unique. If the student is not eligible for the elective subject, it will
    /// return `None`. If the student is not in any classroom, it will return an `Error`.
    pub async fn get_by_id_with_student_context(
        pool: &PgPool,
        id: Uuid,
        student_id: Uuid,
    ) -> Result<Option<Self>> {
        // Checks if the student is in a class available for the elective
        let student_class = DbStudent::get_student_classroom(pool, student_id, None).await?;

        let student_classroom_id = match student_class {
            Some(classroom) => classroom.id,
            None => {
                return Err(Error::InvalidPermission(
                    "Student has no classroom".to_string(),
                    "DbElectiveSubject::get_by_id_with_student_context".to_string(),
                ))
            }
        };

        query_as::<_, DbElectiveSubject>(
            r"
            SELECT * FROM complete_elective_subjects_view
            WHERE id = $1 AND session_code IN (
                SELECT session_code FROM elective_subject_classrooms
                WHERE classroom_id = $2
            )
            ",
        )
        .bind(id)
        .bind(student_classroom_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            Error::InternalSeverError(
                e.to_string(),
                "DbElectiveSubject::get_by_id_with_student_context".to_string(),
            )
        })
    }

    /// Get the requirements of the elective subject
    pub async fn get_requirements(pool: &PgPool, id: Uuid) -> Result<Vec<MultiLangString>> {
        query!(
            r"
            SELECT label_th, label_en FROM elective_subject_requirements
            WHERE elective_subject_id = $1
            ",
            id
        )
        .fetch_all(pool)
        .await
        .map(|res| {
            res.iter()
                .map(|r| MultiLangString::new(r.label_th.clone(), r.label_en.clone()))
                .collect::<Vec<MultiLangString>>()
        })
        .map_err(|e| {
            Error::InternalSeverError(
                e.to_string(),
                "DbElectiveSubject::get_requirements".to_string(),
            )
        })
    }

    /// Checks if the student is in a class available for the elective
    pub async fn is_student_eligible(
        pool: &PgPool,
        session_code: i64,
        student_id: Uuid,
    ) -> Result<bool> {
        // Checks if the student is in a class available for the elective
        let student_class = DbStudent::get_student_classroom(pool, student_id, None).await?;

        let student_classroom_id = match student_class {
            Some(classroom) => classroom.id,
            None => {
                return Err(Error::InvalidPermission(
                    "Student has no classroom".to_string(),
                    "DbElectiveSubject::is_student_eligible".to_string(),
                ))
            }
        };

        let is_eligible = query!(
            r"
            SELECT EXISTS (
                SELECT FROM elective_subject_classrooms
                WHERE session_code = $1 AND classroom_id = $2
            )
            ",
            session_code,
            student_classroom_id
        )
        .fetch_one(pool)
        .await?;

        Ok(is_eligible.exists.unwrap_or(false))
    }

    pub async fn get_subject_applicable_classrooms(&self, pool: &PgPool) -> Result<Vec<Uuid>> {
        let res = query!(
            r"
            SELECT classroom_id FROM elective_subject_classrooms
            WHERE session_code = $1
            ",
            self.session_code
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
        pool: &PgPool,
        academic_year: Option<i64>,
        semester: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            r"
            SELECT
                ses.student_id
            FROM
                elective_subject_classrooms AS esc
                INNER JOIN student_elective_subjects AS ses
                    ON esc.elective_subject_id = ses.elective_subject_id
                INNER JOIN classroom_students AS cs
                    ON cs.student_id = ses.student_id
            WHERE
                cs.classroom_id = esc.classroom_id AND esc.session_code = $1 AND year = $2 AND semester = $3
            ",
            self.session_code,
            academic_year.unwrap_or_else(|| get_current_academic_year(None)),
            semester.unwrap_or_else(|| get_current_academic_year(None)),
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

    pub async fn get_randomized_student(
        &self,
        pool: &PgPool,
        academic_year: Option<i64>,
        semester: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            r"
            SELECT
                ses.student_id
            FROM
                elective_subject_classrooms AS esc
                INNER JOIN student_elective_subjects AS ses
                    ON esc.elective_subject_id = ses.elective_subject_id
                INNER JOIN classroom_students AS cs
                    ON cs.student_id = ses.student_id
            WHERE
                cs.classroom_id = esc.classroom_id AND esc.session_code = $1 AND year = $2 AND semester = $3 AND is_randomized = true
            ",
            self.session_code,
            academic_year.unwrap_or_else(|| get_current_academic_year(None)),
            semester.unwrap_or_else(|| get_current_academic_year(None)),
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.student_id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbElectiveSubject::get_randomized_student".to_string(),
            )),
        }
    }

    pub async fn is_enrollment_period(pool: &PgPool) -> Result<bool> {
        let res = query!(
            r"
            SELECT EXISTS (
                SELECT FROM elective_subject_enrollment_periods
                WHERE timezone ('Asia/Bangkok', now()) BETWEEN start_time AND end_time
            )
            ",
        )
        .fetch_one(pool)
        .await;

        match res {
            Ok(res) => Ok(res.exists.unwrap_or(false)),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbElectiveSubject::is_enrollment_period".to_string(),
            )),
        }
    }
}

impl QueryDb<QueryableElectiveSubject, SortableElectiveSubject> for DbElectiveSubject {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<&FilterConfig<QueryableElectiveSubject>>,
    ) where
        Self: Sized,
    {
        let mut where_sections: Vec<SqlSection> = Vec::new();

        if let Some(filter) = filter {
            if let Some(q) = &filter.q {
                // (name_th ILIKE '%q%' OR name_en ILIKE '%q%' OR code_th ILIKE '%q%' OR code_en
                // ILIKE '%q%')
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
            query_builder.push(if i == 0 { " WHERE " } else { " AND " });
            for (j, sql) in section.sql.iter().enumerate() {
                query_builder.push(sql);
                if j < section.params.len() {
                    match section.params.get(j) {
                        Some(QueryParam::Int(v)) => query_builder.push_bind(*v),
                        Some(QueryParam::Float(v)) => query_builder.push_bind(*v),
                        Some(QueryParam::Uuid(v)) => query_builder.push_bind(*v),
                        Some(QueryParam::String(v)) => query_builder.push_bind(v.clone()),
                        Some(QueryParam::ArrayInt(v)) => query_builder.push_bind(v.clone()),
                        Some(QueryParam::ArrayUuid(v)) => query_builder.push_bind(v.clone()),
                        _ => unreachable!(),
                    };
                }
            }
        }
    }

    async fn query(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableElectiveSubject>>,
        sort: Option<&SortingConfig<SortableElectiveSubject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        let mut query = QueryBuilder::new(DbElectiveSubject::base_query());
        Self::build_shared_query(&mut query, filter);

        if let Some(sorting) = sort {
            query.push(sorting.to_order_by_clause());
        }

        if let Some(pagination) = pagination {
            let limit_section = pagination.to_limit_clause();
            query.push(" ");
            for (i, sql) in limit_section.sql.iter().enumerate() {
                query.push(sql);
                if i < limit_section.params.len() {
                    match limit_section.params.get(i) {
                        Some(&QueryParam::Int(v)) => query.push_bind(v),
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

        // dbg!(&query.build_query_as::<DbElectiveSubject>().sql());

        query
            .build_query_as::<DbElectiveSubject>()
            .fetch_all(pool)
            .await
            .map_err(|e| {
                Error::InternalSeverError(e.to_string(), "DbElectiveSubject::query".to_string())
            })
    }

    async fn response_pagination(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableElectiveSubject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType> {
        let mut query = QueryBuilder::new(DbElectiveSubject::count_query());
        Self::build_shared_query(&mut query, filter);

        let count = u32::try_from(
            query
                .build()
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    Error::InternalSeverError(
                        e.to_string(),
                        "DbElectiveSubject::response_pagination".to_string(),
                    )
                })?
                .get::<i64, _>("count"),
        )
        .unwrap();

        match pagination {
            Some(pagination) => Ok(PaginationType::new(
                pagination.p,
                pagination.size.unwrap(),
                count,
            )),
            None => Ok(PaginationType::new(
                PaginationConfig::default().p,
                PaginationConfig::default().size.unwrap(),
                count,
            )),
        }
    }
}
