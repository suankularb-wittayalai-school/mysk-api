use std::any::Any;

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
    models::{
        club::{self, db::DbClub, Club},
        club_request::ClubRequest,
        enums::SubmissionStatus,
        student::Student,
        traits::TopLevelGetById as _,
    },
    prelude::*,
};
use serde::Deserialize;
use sqlx::{query, query_as};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct UpdatableClubRequest {
    pub status: SubmissionStatus,
}

#[put("/{id}")]
pub async fn update_club_requests(
    data: Data<AppState>,
    club_request_id: Path<Uuid>,
    request_body: Json<
        RequestType<UpdatableClubRequest, QueryablePlaceholder, SortablePlaceholder>,
    >,
    student_id: LoggedInStudent,
    _: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;
    let client_student_id = student_id.0;
    let club_request_id = club_request_id.into_inner();
    let club_request_status = match &request_body.data {
        Some(request_data) => match request_data.status {
            SubmissionStatus::Approved | SubmissionStatus::Declined => request_data.status,
            SubmissionStatus::Pending => {
                return Err(Error::InvalidRequest(
                    "Status must be either 'approved' or 'declined'".to_string(),
                    format!("/clubs/requests/{club_request_id}"),
                ));
            }
        },
        _ => unreachable!("JSON errors are pre-handled by the JsonConfig error handler"),
    };
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();

    // Check if the club request exists
    let club_request = match ClubRequest::get_by_id(
        pool,
        club_request_id,
        Some(&FetchLevel::Default),
        Some(&FetchLevel::Detailed),
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

    // Check if the club request is still pending
    match club_request.membership_status {
        SubmissionStatus::Approved | SubmissionStatus::Declined => {
            return Err(Error::InvalidPermission(
                format!(
                    "Club request has already been {}",
                    club_request.membership_status
                ),
                format!("/clubs/requests/{club_request_id}"),
            ))
        }
        SubmissionStatus::Pending => (),
    }

    // Check if client student is permitted to update the club request by checking if they are club staff

    let club = club_request.club.unwrap_detailed();

    // Check if the client student is a staff member of the club
    if !club
        .staffs
        .iter()
        .any(|staff| staff.id == client_student_id)
    {
        return Err(Error::InvalidPermission(
            "Student must be a staff member of the club to update club requests".to_string(),
            format!("/clubs/requests/{club_request_id}"),
        ));
    }

    // Update the club request status to either approved or declined
    let mut updated_status: Option<SubmissionStatus> = club_request_status.into();

    query!(
        "UPDATE club_members SET membership_status = $1 WHERE id = $2",
        updated_status.unwrap() as SubmissionStatus,
        club_request_id,
    )
    .execute(pool)
    .await?;

    let updated_club_request = ClubRequest::get_by_id(
        pool,
        club_request_id,
        Some(&FetchLevel::Detailed),
        Some(&FetchLevel::IdOnly),
    )
    .await?;

    let response = ResponseType::new(updated_club_request, None);

    Ok(HttpResponse::Ok().json(response))
}
