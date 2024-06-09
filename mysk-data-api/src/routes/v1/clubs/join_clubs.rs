use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
    AppState,
};
use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use log::info;
use mysk_lib::{
    common::{
        requests::{FetchLevel, RequestType},
        response::ResponseType,
    },
    helpers::date::get_current_academic_year,
    models::{
        club::Club,
        club_request::{
            request::{queryable::QueryableClubRequest, sortable::SortableClubRequest},
            ClubRequest,
        },
        enums::SubmissionStatus,
        traits::TopLevelGetById,
    },
    prelude::*,
};

use sqlx::{pool, query};
use uuid::Uuid;

#[post("/{id}/join")]
pub async fn join_clubs(
    data: Data<AppState>,
    club_id: Path<Uuid>,
    student_id: LoggedInStudent,
    request_body: Json<RequestType<ClubRequest, QueryableClubRequest, SortableClubRequest>>,
    _: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = student_id.0;
    let club_id = club_id.into_inner();
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();

    // Check if club exists
    let club = match Club::get_by_id(
        pool,
        club_id,
        Some(&FetchLevel::Detailed),
        Some(&FetchLevel::IdOnly),
    )
    .await
    {
        Ok(Club::Detailed(club, _)) => club,
        Err(Error::InternalSeverError(_, _)) => {
            return Err(Error::EntityNotFound(
                "Club not found".to_string(),
                format!("clubs/{club_id}/contacts"),
            ))
        }
        _ => unreachable!("Club::get_by_id should always return a Detailed variant"),
    };

    let mut new_join_request: bool = true;

    // Check if student has already requested to join the club or is already a member
    if let Some(has_requested) = query!(
        r#"
            SELECT membership_status "membership_status: SubmissionStatus" FROM club_members
            WHERE club_id = $1 AND year = $2 and membership_status != $3 AND student_id = $4
        "#,
        club_id,
        get_current_academic_year(None),
        SubmissionStatus::Declined as SubmissionStatus,
        student_id
    )
    .fetch_optional(pool)
    .await?
    {
        match has_requested.membership_status {
            SubmissionStatus::Approved => {
                return Err(Error::InvalidPermission(
                    "Student is already a member of the club".to_string(),
                    format!("clubs/{club_id}/join"),
                ))
            }
            SubmissionStatus::Pending => new_join_request = false,
            SubmissionStatus::Declined => unreachable!(),
        }
    }

    // Insert new club member
    let club_member_id = if new_join_request {
        query!(
            r#"
                INSERT INTO club_members (club_id, year, membership_status, student_id)
                VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING RETURNING id
            "#,
            club.id,
            get_current_academic_year(None),
            SubmissionStatus::Pending as SubmissionStatus,
            student_id
        )
        .fetch_one(pool)
        .await?
        .id
    } else {
        return Err(Error::InvalidPermission(
            "Student has already requested to join the club".to_string(),
            format!("clubs/{club_id}/join"),
        ));
    };

    let club_request_id =
        ClubRequest::get_by_id(pool, club_member_id, fetch_level, descendant_fetch_level).await?;

    let response = ResponseType::new(club_member_id, None);

    Ok(HttpResponse::Ok().json(response))
}
