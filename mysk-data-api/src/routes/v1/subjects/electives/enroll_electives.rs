use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, student::LoggedInStudent},
    AppState,
};
use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    helpers::date::{get_current_academic_year, get_current_semester},
    models::{
        elective_subject::{db::DbElectiveSubject, ElectiveSubject},
        traits::{GetById as _, TopLevelGetById as _},
    },
    permissions::Authorizer,
    prelude::*,
    query::QueryablePlaceholder,
};
use sqlx::query;
use uuid::Uuid;

#[allow(clippy::too_many_lines)]
#[post("/{id}/enroll")]
pub async fn enroll_elective_subject(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    LoggedInStudent(student_id): LoggedInStudent,
    elective_subject_session_id: Path<Uuid>,
    Json(RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<(), QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut transaction = pool.begin().await?;
    let elective_subject_session_id = elective_subject_session_id.into_inner();
    let authorizer = Authorizer::new(
        pool,
        &user,
        format!("/subjects/electives/{elective_subject_session_id}/enroll"),
    )
    .await?;

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
            "The elective enrollment period has ended".to_string(),
            format!("/subjects/electives/{elective_subject_session_id}/enroll"),
        ));
    }

    // Checks if the student has already enrolled in an elective in the current semester
    if DbElectiveSubject::is_currently_enrolled(&mut *transaction, student_id)
        .await?
        .is_some()
    {
        return Err(Error::InvalidPermission(
            "Student has already enrolled in an elective this semester".to_string(),
            format!("/subjects/electives/{elective_subject_session_id}/enroll"),
        ));
    }

    // We use a transaction-level lock to (hopefully) prevent any race conditions. Using
    // the session code as the lock's identifier means that it should only lock when
    // multiple students tries to enroll on the same elective session, and students which
    // are enrolling on other elective sessions should have their own respective locks.
    //
    // P.S. The numbers "69 69 76" are ASCII code that translates to "E E L"
    //      (Enroll Electives Lock).
    //
    // References:
    //   - https://www.postgresql.org/docs/14/functions-admin.html#FUNCTIONS-ADVISORY-LOCKS
    //   - https://www.postgresql.org/docs/14/explicit-locking.html#ADVISORY-LOCKS
    query!(
        "\
        SELECT pg_advisory_xact_lock(696976, session_code::int)\
        FROM elective_subject_sessions WHERE id = $1\
        ",
        elective_subject_session_id,
    )
    .execute(&mut *transaction)
    .await?;

    // Checks if the elective the student is trying to enroll in is available
    let elective = DbElectiveSubject::get_by_id(pool, elective_subject_session_id).await?;

    if DbElectiveSubject::get_previously_enrolled_electives(&mut *transaction, student_id)
        .await?
        .contains(&elective_subject_session_id)
    {
        return Err(Error::InvalidPermission(
            "Student cannot re-enroll in the same elective".to_string(),
            format!("/subjects/electives/{elective_subject_session_id}/enroll"),
        ));
    }

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
        "\
        INSERT INTO elective_subject_session_enrolled_students\
        (student_id, elective_subject_session_id) VALUES ($1, $2)\
        ",
        student_id,
        elective.id,
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    let elective = ElectiveSubject::get_by_id(
        pool,
        elective_subject_session_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(elective, None);

    Ok(HttpResponse::Ok().json(response))
}
