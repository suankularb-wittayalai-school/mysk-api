use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
    AppState,
};
use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{FetchLevel, QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    helpers::date::{get_current_academic_year, get_current_semester},
    models::{
        elective_subject::{db::DbElectiveSubject, ElectiveSubject},
        traits::TopLevelGetById,
    },
    prelude::*,
};
use mysk_lib_macros::traits::db::GetById;
use sqlx::query;
use uuid::Uuid;

#[allow(clippy::too_many_lines)]
#[post("/{id}/enroll")]
pub async fn enroll_elective_subject(
    data: Data<AppState>,
    elective_subject_session_id: Path<Uuid>,
    student_id: LoggedInStudent,
    request_body: Json<RequestType<ElectiveSubject, QueryablePlaceholder, SortablePlaceholder>>,
    _: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = student_id.0;
    let elective_subject_session_id = elective_subject_session_id.into_inner();
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();

    // Checks if the elective the student is trying to enroll in is available
    let elective = match ElectiveSubject::get_by_id(
        pool,
        elective_subject_session_id,
        Some(&FetchLevel::Detailed),
        None,
    )
    .await
    {
        Ok(ElectiveSubject::Detailed(elective, _)) => {
            if elective.class_size == elective.cap_size {
                return Err(Error::InvalidPermission(
                    "The elective is already full".to_string(),
                    format!("/subjects/electives/{elective_subject_session_id}/enroll"),
                ));
            }

            elective
        }
        Err(Error::InternalSeverError(_, _)) => {
            return Err(Error::EntityNotFound(
                "Elective subject not found".to_string(),
                format!("/subjects/electives/{elective_subject_session_id}/enroll"),
            ));
        }
        _ => unreachable!("ElectiveSubject::get_by_id should always return a Detailed variant"),
    };

    // Check if the current time is within the elective's enrollment period
    if !DbElectiveSubject::is_enrollment_period(pool).await? {
        return Err(Error::InvalidPermission(
            "The elective's enrollment period has ended".to_string(),
            format!("/subjects/electives/{elective_subject_session_id}/enroll"),
        ));
    }

    // Checks if the student is in a class available for the elective
    if !DbElectiveSubject::is_student_eligible(pool, elective_subject_session_id, student_id)
        .await?
    {
        return Err(Error::InvalidPermission(
            "Student is not eligible to enroll in this elective".to_string(),
            format!("/subjects/electives/{elective_subject_session_id}/enroll"),
        ));
    }

    // Checks if the student has already enrolled in the elective before
    let subject_id = DbElectiveSubject::get_by_id(pool, elective_subject_session_id)
        .await?
        .subject_id;

    let enroll_count = query!(
        r"
        SELECT 
            COUNT(*) 
        FROM 
            elective_subject_session_enrolled_students 
            INNER JOIN elective_subject_sessions electives
            ON enrolls.elective_subject_session_id = electives.id
        WHERE student_id = $1 AND subject_id = $2
        ",
        student_id,
        subject_id,
    )
    .fetch_one(pool)
    .await?;

    let enroll_count: i64 = enroll_count.count.unwrap_or(0);
    if enroll_count > 0 {
        return Err(Error::InvalidPermission(
            "Student has already enrolled in this elective before".to_string(),
            format!("/subjects/electives/{elective_subject_session_id}/enroll"),
        ));
    }

    // Checks if the student is already enrolled in an elective this semester
    let has_enrolled = query!(
        r"
        SELECT EXISTS (
            SELECT elective_subject_session_id FROM elective_subject_session_enrolled_students INNER JOIN elective_subject_sessions ON elective_subject_session_enrolled_students.elective_subject_session_id = elective_subject_sessions.id
            WHERE student_id = $1 and year = $2 AND semester = $3
        )
        ",
        student_id,
        get_current_academic_year(None),
        get_current_semester(None),
    )
    .fetch_one(pool)
    .await?
    .exists
    .unwrap_or(false);
    if has_enrolled {
        return Err(Error::InvalidPermission(
            "Student has already enrolled in an elective this semester".to_string(),
            format!("/subjects/electives/{elective_subject_session_id}/enroll"),
        ));
    }

    query!(
        r"
        INSERT INTO elective_subject_session_enrolled_students (student_id, elective_subject_session_id)  VALUES ($1, $2) ON CONFLICT DO NOTHING
        ",
        student_id,
        elective.id,
    )
    .execute(pool)
    .await?;

    // Get the elective subject by session code with the fetch levels requested to return the response
    let elective = ElectiveSubject::get_by_id(
        pool,
        elective_subject_session_id,
        fetch_level,
        descendant_fetch_level,
    )
    .await?;
    let response = ResponseType::new(elective, None);

    Ok(HttpResponse::Ok().json(response))
}
