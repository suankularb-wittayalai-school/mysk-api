use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, student::LoggedInStudent},
};
use actix_web::{
    HttpResponse, Responder, delete,
    web::{Data, Path},
};
use mysk_lib::{
    common::{
        requests::FetchLevel,
        response::{EmptyResponseData, ResponseType},
    },
    models::{club_request::ClubRequest, enums::SubmissionStatus, student::Student},
    permissions::Authorizer,
    prelude::*,
};
use sqlx::query;
use uuid::Uuid;

#[delete("/{id}")]
pub async fn delete_club_requests(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    LoggedInStudent(student_id): LoggedInStudent,
    club_request_id: Path<Uuid>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let club_request_id = club_request_id.into_inner();
    let authorizer = Authorizer::new(
        &mut conn,
        &user,
        format!("/clubs/requests/{club_request_id}"),
    )
    .await?;

    // Check if the club request exists
    let ClubRequest::Default(club_request, _) = ClubRequest::get_by_id(
        pool,
        club_request_id,
        FetchLevel::Default,
        FetchLevel::IdOnly,
        &authorizer,
    )
    .await?
    else {
        unreachable!("ClubRequest::get_by_id should always return a Default variant")
    };

    // Check if the club request's student id matches the logged in student id
    if student_id
        != match club_request.student {
            Student::IdOnly(student, _) => student.id,
            _ => unreachable!("Student should always be an IdOnly variant"),
        }
    {
        return Err(Error::InvalidPermission(
            "Student is not allowed to interact with this club request".to_string(),
            format!("/clubs/requests/{club_request_id}"),
        ));
    }

    // Returns early if the club request is not pending
    if SubmissionStatus::Pending != club_request.membership_status {
        return Err(Error::InvalidPermission(
            format!(
                "Club request has already been {}",
                club_request.membership_status,
            ),
            format!("/clubs/requests/{club_request_id}"),
        ));
    }

    query!(
        "DELETE FROM club_members WHERE id = $1 AND membership_status = $2",
        club_request_id,
        SubmissionStatus::Pending as SubmissionStatus,
    )
    .execute(&mut *conn)
    .await?;

    let response = ResponseType::new(EmptyResponseData {}, None);

    Ok(HttpResponse::Ok().json(response))
}
