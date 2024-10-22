use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
    AppState,
};
use actix_web::{
    put,
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
        traits::TopLevelGetById as _,
    },
    prelude::*,
};
use sqlx::query;
use uuid::Uuid;

#[allow(clippy::too_many_lines)]
#[put("/{id}/enroll")]
async fn modify_elective_subject(
    data: Data<AppState>,
    _: ApiKeyHeader,
    student_id: LoggedInStudent,
    elective_subject_session_id: Path<Uuid>,
    request_body: Json<RequestType<ElectiveSubject, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut transaction = pool.begin().await?;
    let student_id = student_id.0;
    let elective_subject_session_id = elective_subject_session_id.into_inner();
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();

    // Checks if the student is "blacklisted" from enrolling in an elective
    if DbElectiveSubject::is_student_blacklisted(&mut *transaction, student_id).await? {
        return Err(Error::InvalidPermission(
            "Student is blacklisted from enrolling in electives".to_string(),
            format!("/subjects/electives/{elective_subject_session_id}/enroll"),
        ));
    }

    // Checks if the current time is within the elective's enrollment period
    if !DbElectiveSubject::is_enrollment_period(&mut *transaction, student_id).await? {
        return Err(Error::InvalidPermission(
            "The elective's enrollment period has ended".to_string(),
            format!("/subjects/electives/{elective_subject_session_id}/enroll"),
        ));
    }

    // Checks if the student hasn't enrolled in an elective subject in the current semester yet
    let Some(current_elective_subject_id) =
        DbElectiveSubject::is_currently_enrolled(&mut *transaction, student_id).await?
    else {
        return Err(Error::InvalidPermission(
            "Student has not enrolled in an elective this semester".to_string(),
            format!("/subjects/electives/{elective_subject_session_id}/enroll"),
        ));
    };

    // Refer to enroll_electives.rs on line 65 for a detailed explanation.
    //
    // P.S. The numbers "77 69 76" are ASCII code that translates to "M E L"
    //      (Modify Electives Lock).
    query!(
        "
        SELECT pg_advisory_xact_lock(776976, session_code::int)
        FROM elective_subject_sessions WHERE id = $1
        ",
        elective_subject_session_id,
    )
    .execute(&mut *transaction)
    .await?;

    // Checks if the elective the student is trying to enroll in is available
    let elective = match ElectiveSubject::get_by_id(
        pool,
        elective_subject_session_id,
        Some(&FetchLevel::Detailed),
        None,
    )
    .await?
    {
        ElectiveSubject::Detailed(elective, _) => {
            if elective.year != Some(get_current_academic_year(None))
                || elective.semester != Some(get_current_semester(None))
            {
                return Err(Error::InvalidPermission(
                    "Student cannot enroll in a non-current elective".to_string(),
                    format!("/subjects/electives/{elective_subject_session_id}/enroll"),
                ));
            }
            if elective.class_size >= elective.cap_size {
                return Err(Error::InvalidPermission(
                    "The elective is already full".to_string(),
                    format!("/subjects/electives/{elective_subject_session_id}/enroll"),
                ));
            }

            elective
        }
        _ => unreachable!("ElectiveSubject::get_by_id should always return a Detailed variant"),
    };

    // Checks if the student is in a class available for the elective
    if !DbElectiveSubject::is_student_eligible(
        &mut *transaction,
        elective_subject_session_id,
        student_id,
    )
    .await?
    {
        return Err(Error::InvalidPermission(
            "Student is not eligible to enroll in this elective".to_string(),
            format!("/subjects/electives/{elective_subject_session_id}/enroll"),
        ));
    }

    query!(
        "
        UPDATE elective_subject_session_enrolled_students
        SET updated_at = now(), elective_subject_session_id = $1
        WHERE student_id = $2 AND elective_subject_session_id = $3
        ",
        elective.id,
        student_id,
        current_elective_subject_id,
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

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
