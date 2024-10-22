use crate::{
    common::{
        requests::{FilterConfig, PaginationConfig, QueryParam, SortingConfig, SqlSection},
        response::PaginationType,
    },
    helpers::date::{get_current_academic_year, get_current_semester},
    models::{
        elective_subject::request::{
            queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject,
        },
        enums::SubjectType,
        student::db::DbStudent,
        traits::{QueryDb, Queryable as _},
    },
    prelude::*,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{query, Acquire, FromRow, PgPool, Postgres, QueryBuilder, Row as _};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT * FROM elective_subject_sessions_with_detail_view",
    count_query = "SELECT COUNT(*) FROM elective_subject_sessions_with_detail_view"
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
    pub year: Option<i64>,
    pub semester: Option<i64>,
    pub subject_group_id: i64,
    pub syllabus: Option<String>,
    pub session_code: String,
}

impl DbElectiveSubject {
    /// Checks if the student is in a class available for the elective.
    pub async fn is_student_eligible<'a, A>(
        conn: A,
        session_id: Uuid,
        student_id: Uuid,
    ) -> Result<bool>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let mut conn = conn.acquire().await?;
        let student_classroom_id =
            match DbStudent::get_student_classroom(&mut *conn, student_id, None).await? {
                Some(classroom) => classroom.id,
                None => {
                    return Err(Error::InvalidPermission(
                        "Student has no classroom".to_string(),
                        "DbElectiveSubject::is_student_eligible".to_string(),
                    ))
                }
            };

        let is_eligible = query!(
            "
            SELECT EXISTS (
                SELECT FROM elective_subject_session_classrooms
                WHERE elective_subject_session_id = $1 AND classroom_id = $2
            )
            ",
            session_id,
            student_classroom_id,
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(is_eligible.exists.unwrap_or(false))
    }

    /// Checks if the student is "blacklisted" from enrolling in an elective.
    pub async fn is_student_blacklisted<'a, A>(conn: A, student_id: Uuid) -> Result<bool>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let res = query!(
            "
            SELECT EXISTS (
                SELECT FROM elective_subject_session_blacklisted_students WHERE student_id = $1
            )
            ",
            student_id,
        )
        .fetch_one(&mut *(conn.acquire().await?))
        .await?;

        Ok(res.exists.unwrap_or(false))
    }

    /// Checks if the student has already enrolled in an elective in the current semester.
    /// Returns the elective subject session ID if an enrollment exists.
    pub async fn is_currently_enrolled<'a, A>(conn: A, student_id: Uuid) -> Result<Option<Uuid>>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let res = query!(
            "
            SELECT elective_subject_session_id
            FROM
                elective_subject_session_enrolled_students AS esses
                JOIN elective_subject_sessions AS ess ON ess.id = esses.elective_subject_session_id
            WHERE student_id = $1 and year = $2 AND semester = $3
            ",
            student_id,
            get_current_academic_year(None),
            get_current_semester(None),
        )
        .fetch_optional(&mut *(conn.acquire().await?))
        .await?;

        Ok(res.map(|r| r.elective_subject_session_id))
    }

    pub async fn get_previously_enrolled_electives<'a, A>(
        conn: A,
        student_id: Uuid,
    ) -> Result<Vec<Uuid>>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let res = query!(
            "
            SELECT ess.id FROM elective_subject_sessions AS ess
            JOIN subjects AS su ON su.id = ess.subject_id
            WHERE su.id IN (
                    SELECT i_su.id FROM subjects AS i_su
                    JOIN elective_subject_sessions AS i_ess ON i_ess.subject_id = i_su.id
                    JOIN
                        elective_subject_session_enrolled_students AS i_esses
                        ON i_esses.elective_subject_session_id = i_ess.id
                    WHERE i_esses.student_id = $1
                )
            AND ess.year = $2
            AND ess.semester = $3
            ",
            student_id,
            get_current_academic_year(None),
            get_current_semester(None),
        )
        .fetch_all(&mut *(conn.acquire().await?))
        .await?;

        Ok(res.iter().map(|r| r.id).collect())
    }

    pub async fn get_subject_applicable_classrooms(&self, pool: &PgPool) -> Result<Vec<Uuid>> {
        let res = query!(
            "
            SELECT classroom_id FROM elective_subject_session_classrooms
            WHERE elective_subject_session_id = $1
            ",
            self.id,
        )
        .fetch_all(pool)
        .await?;

        Ok(res.iter().map(|r| r.classroom_id).collect())
    }

    pub async fn get_enrolled_students(&self, pool: &PgPool) -> Result<Vec<Uuid>> {
        let res = query!(
            "
            SELECT student_id FROM elective_subject_session_enrolled_students
            WHERE elective_subject_session_id = $1
            ",
            self.id,
        )
        .fetch_all(pool)
        .await?;

        Ok(res.iter().map(|r| r.student_id).collect())
    }

    pub async fn get_randomized_students(&self, pool: &PgPool) -> Result<Vec<Uuid>> {
        let res = query!(
            "
            SELECT student_id FROM elective_subject_session_enrolled_students
            WHERE elective_subject_session_id = $1 AND is_randomized
            ",
            self.id,
        )
        .fetch_all(pool)
        .await?;

        Ok(res.iter().map(|r| r.student_id).collect())
    }

    pub async fn is_enrollment_period<'a, A>(conn: A, student_id: Uuid) -> Result<bool>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let res = query!(
            "
            SELECT EXISTS (
                SELECT FROM elective_subject_enrollment_periods
                WHERE 
                    now() BETWEEN start_time AND end_time
                    AND (
                        grade IS NULL OR grade = floor(
                            (
                                SELECT number
                                FROM classrooms AS c
                                JOIN classroom_students AS cs ON cs.classroom_id = c.id
                                WHERE cs.student_id = $1 AND year = $2
                            ) / 100
                        )
                    )
            )
            ",
            student_id,
            get_current_academic_year(None),
        )
        .fetch_one(&mut *(conn.acquire().await?))
        .await?;

        Ok(res.exists.unwrap_or(false))
    }
}

#[async_trait]
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
            let limit_section = pagination.to_limit_clause()?;
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
