use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, student::LoggedInStudent},
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
        club::{db::DbClub, Club},
        club_request::ClubRequest,
        enums::SubmissionStatus,
        traits::TopLevelGetById as _,
    },
    permissions,
    prelude::*,
};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct UpdateClubRequest {
    pub status: SubmissionStatus,
}

#[put("/{id}")]
pub async fn update_club_requests(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    student_id: LoggedInStudent,
    club_request_id: Path<Uuid>,
    request_body: Json<RequestType<UpdateClubRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let student_id = student_id.0;
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
        None => {
            return Err(Error::InvalidRequest(
                "Json deserialize error: field `data` can not be empty".to_string(),
                format!("/clubs/requests/{club_request_id}"),
            ));
        }
    };
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/clubs/requests/{club_request_id}"))
            .await?;

    // Check if the club request exists
    let club_request = match ClubRequest::get_by_id(
        pool,
        club_request_id,
        Some(&FetchLevel::Default),
        Some(&FetchLevel::IdOnly),
        &*authorizer,
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
                    club_request.membership_status,
                ),
                format!("/clubs/requests/{club_request_id}"),
            ))
        }
        SubmissionStatus::Pending => (),
    }

    // Check if the student is a staff of the club
    match club_request.club {
        Club::IdOnly(club, _) => {
            if !DbClub::get_club_staffs(pool, club.id)
                .await?
                .iter()
                .any(|staff_id| *staff_id == student_id)
            {
                return Err(Error::InvalidPermission(
                    "Student must be a staff member of the club to update club requests"
                        .to_string(),
                    format!("/clubs/requests/{club_request_id}"),
                ));
            }
        }
        _ => unreachable!("Club::get_by_id should always return an IdOnly variant"),
    }

    query!(
        "UPDATE club_members SET membership_status = $1 WHERE id = $2",
        club_request_status as SubmissionStatus,
        club_request_id,
    )
    .execute(pool)
    .await?;

    let updated_club_request = ClubRequest::get_by_id(
        pool,
        club_request_id,
        fetch_level,
        descendant_fetch_level,
        &*authorizer,
    )
    .await?;
    let response = ResponseType::new(updated_club_request, None);

    Ok(HttpResponse::Ok().json(response))
}
