use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, student::LoggedInStudent},
};
use actix_web::{
    HttpResponse, Responder, put,
    web::{Data, Json, Path},
};
use mysk_lib::{
    common::{
        requests::{FetchLevel, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::{
        club::{Club, db::DbClub},
        club_request::ClubRequest,
        enums::SubmissionStatus,
        traits::TopLevelGetById as _,
    },
    permissions::Authorizer,
    prelude::*,
    query::QueryablePlaceholder,
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
    LoggedIn(user): LoggedIn,
    LoggedInStudent(student_id): LoggedInStudent,
    club_request_id: Path<Uuid>,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<UpdateClubRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let club_request_id = club_request_id.into_inner();
    let club_request_status = if let Some(request_data) = request_data {
        if matches!(request_data.status, SubmissionStatus::Pending) {
            return Err(Error::InvalidRequest(
                "Status must be either `approved` or `declined`".to_string(),
                format!("/clubs/requests/{club_request_id}"),
            ));
        }

        request_data.status
    } else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/clubs/requests/{club_request_id}"),
        ));
    };
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
        Some(FetchLevel::Default),
        Some(FetchLevel::IdOnly),
        &authorizer,
    )
    .await
    .map_err(|e| match e {
        Error::EntityNotFound(_, _) => Error::EntityNotFound(
            "Club request not found".to_string(),
            format!("/clubs/requests/{club_request_id}"),
        ),
        _ => e,
    })?
    else {
        unreachable!("ClubRequest::get_by_id should always return a Default variant")
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
            ));
        }
        SubmissionStatus::Pending => (),
    }

    // Check if the student is a staff of the club
    match club_request.club {
        Club::IdOnly(club, _) => {
            if !DbClub::get_club_staffs(&mut conn, club.id)
                .await?
                .contains(&student_id)
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
    .execute(&mut *conn)
    .await?;

    let updated_club_request = ClubRequest::get_by_id(
        pool,
        club_request_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(updated_club_request, None);

    Ok(HttpResponse::Ok().json(response))
}
