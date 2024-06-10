use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
    AppState,
};
use actix_web::{
    delete,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::FetchLevel,
        response::{EmptyResponseData, ResponseType},
    },
    models::{
        club_request::ClubRequest, enums::SubmissionStatus, student::Student,
        traits::TopLevelGetById as _,
    },
    prelude::*,
};
use sqlx::query;
use uuid::Uuid;

#[delete("/{id}")]
pub async fn delete_club_requests(
    data: Data<AppState>,
    club_request_id: Path<Uuid>,
    student_id: LoggedInStudent,
    _: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = student_id.0;
    let club_request_id = club_request_id.into_inner();

    // Check if the club request exists
    let club_request = match ClubRequest::get_by_id(
        pool,
        club_request_id,
        Some(&FetchLevel::Default),
        Some(&FetchLevel::IdOnly),
    )
    .await
    {
        Ok(ClubRequest::Default(club_request, _)) => club_request,
        Err(Error::InternalSeverError(_, _)) => {
            return Err(Error::EntityNotFound(
                "Club request not found".to_string(),
                format!("/clubs/requests/{club_request_id}"),
            ))
        }
        _ => unreachable!("ClubRequest::get_by_id should always return a Default variant"),
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
    .execute(pool)
    .await?;

    let response = ResponseType::new(EmptyResponseData {}, None);

    Ok(HttpResponse::Ok().json(response))
}
