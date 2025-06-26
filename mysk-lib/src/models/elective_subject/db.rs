use crate::{
    common::requests::FilterConfig,
    helpers::date::{get_current_academic_year, get_current_semester},
    models::{
        elective_subject::request::{
            queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject,
        },
        enums::SubjectType,
        student::db::DbStudent,
        traits::QueryDb,
    },
    prelude::*,
    query::Queryable as _,
    query::{QueryParam, SqlWhereClause},
};
use chrono::{DateTime, Utc};
use mysk_lib_macros::GetById;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgConnection, Postgres, QueryBuilder, query};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, GetById)]
#[from_query(
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
    pub async fn is_student_eligible(
        conn: &mut PgConnection,
        session_id: Uuid,
        student_id: Uuid,
    ) -> Result<bool> {
        let student_classroom_id =
            match DbStudent::get_student_classroom(&mut *conn, student_id, None).await? {
                Some(classroom) => classroom.id,
                None => {
                    return Err(Error::InvalidPermission(
                        "Student has no classroom".to_string(),
                        "DbElectiveSubject::is_student_eligible".to_string(),
                    ));
                }
            };

        let is_eligible = query!(
            "\
            SELECT EXISTS (\
                SELECT FROM elective_subject_session_classrooms \
                WHERE elective_subject_session_id = $1 AND classroom_id = $2\
            )\
            ",
            session_id,
            student_classroom_id,
        )
        .fetch_one(conn)
        .await?;

        Ok(is_eligible.exists.unwrap_or(false))
    }

    /// Checks if the student is "blacklisted" from enrolling in an elective.
    pub async fn is_student_blacklisted(conn: &mut PgConnection, student_id: Uuid) -> Result<bool> {
        let res = query!(
            "\
            SELECT EXISTS (\
                SELECT FROM elective_subject_session_blacklisted_students WHERE student_id = $1\
            )\
            ",
            student_id,
        )
        .fetch_one(conn)
        .await?;

        Ok(res.exists.unwrap_or(false))
    }

    /// Checks if the student has already enrolled in an elective in the current semester.
    /// Returns the elective subject session ID if an enrollment exists.
    pub async fn is_currently_enrolled(
        conn: &mut PgConnection,
        student_id: Uuid,
    ) -> Result<Option<Uuid>> {
        let res = query!(
            "\
            SELECT elective_subject_session_id \
            FROM elective_subject_session_enrolled_students AS esses \
            JOIN elective_subject_sessions AS ess ON ess.id = esses.elective_subject_session_id \
            WHERE student_id = $1 and year = $2 AND semester = $3\
            ",
            student_id,
            get_current_academic_year(None),
            get_current_semester(None),
        )
        .fetch_optional(conn)
        .await?;

        Ok(res.map(|r| r.elective_subject_session_id))
    }

    pub async fn get_previously_enrolled_electives(
        conn: &mut PgConnection,
        student_id: Uuid,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            "\
            SELECT ess.id FROM elective_subject_sessions AS ess \
            JOIN subjects AS su ON su.id = ess.subject_id \
            WHERE su.id IN (\
                SELECT i_su.id FROM subjects AS i_su \
                JOIN elective_subject_sessions AS i_ess ON i_ess.subject_id = i_su.id \
                JOIN elective_subject_session_enrolled_students AS i_esses \
                    ON i_esses.elective_subject_session_id = i_ess.id \
                    AND (i_ess.year != $2 OR i_ess.semester != $3)\
                WHERE i_esses.student_id = $1\
            ) AND ess.year = $2 AND ess.semester = $3\
            ",
            student_id,
            get_current_academic_year(None),
            get_current_semester(None),
        )
        .fetch_all(conn)
        .await?;

        Ok(res.iter().map(|r| r.id).collect())
    }

    pub async fn get_subject_applicable_classrooms(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            "\
            SELECT classroom_id FROM elective_subject_session_classrooms \
            WHERE elective_subject_session_id = $1\
            ",
            self.id,
        )
        .fetch_all(conn)
        .await?;

        Ok(res.into_iter().map(|r| r.classroom_id).collect())
    }

    pub async fn get_enrolled_students(&self, conn: &mut PgConnection) -> Result<Vec<Uuid>> {
        let res = query!(
            "\
            SELECT student_id FROM elective_subject_session_enrolled_students \
            WHERE elective_subject_session_id = $1\
            ",
            self.id,
        )
        .fetch_all(conn)
        .await?;

        Ok(res.into_iter().map(|r| r.student_id).collect())
    }

    pub async fn get_randomized_students(&self, conn: &mut PgConnection) -> Result<Vec<Uuid>> {
        let res = query!(
            "\
            SELECT student_id FROM elective_subject_session_enrolled_students \
            WHERE elective_subject_session_id = $1 AND is_randomized\
            ",
            self.id,
        )
        .fetch_all(conn)
        .await?;

        Ok(res.iter().map(|r| r.student_id).collect())
    }

    pub async fn is_enrollment_period(conn: &mut PgConnection, student_id: Uuid) -> Result<bool> {
        let res = query!(
            "\
            SELECT EXISTS (\
                SELECT FROM elective_subject_enrollment_periods \
                WHERE now() BETWEEN start_time AND end_time AND (\
                    grade IS NULL OR grade = floor((\
                        SELECT number FROM classrooms AS c \
                        JOIN classroom_students AS cs ON cs.classroom_id = c.id \
                        WHERE cs.student_id = $1 AND year = $2\
                    ) / 100)\
                )\
            )\
            ",
            student_id,
            get_current_academic_year(None),
        )
        .fetch_one(conn)
        .await?;

        Ok(res.exists.unwrap_or(false))
    }
}

impl QueryDb<QueryableElectiveSubject, SortableElectiveSubject> for DbElectiveSubject {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<QueryableElectiveSubject>>,
    ) {
        if let Some(filter) = filter {
            let query_is_none = filter.q.is_none();

            if let Some(query) = filter.q {
                let mut wc = SqlWhereClause::new();
                wc.push_sql("name_th ILIKE ('%' || ")
                    .push_param(QueryParam::String(query))
                    .push_sql(" || '%') OR name_en ILIKE ('%' || ")
                    .push_prev_param()
                    .push_sql(" || '%') OR code_th ILIKE ('%' || ")
                    .push_prev_param()
                    .push_sql(" || '%') OR code_en ILIKE ('%' || ")
                    .push_prev_param()
                    .push_sql(" || '%')");

                wc.append_into_query_builder(query_builder);
            }
            if let Some(data) = filter.data {
                if query_is_none {
                    query_builder.push(" WHERE ");
                }

                data.to_where_clause()
                    .append_into_query_builder(query_builder);
            }
        }
    }
}
