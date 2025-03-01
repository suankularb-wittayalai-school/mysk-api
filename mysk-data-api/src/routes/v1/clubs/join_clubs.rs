use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, student::LoggedInStudent},
    AppState,
};
use actix_web::{
    post,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{FetchLevel, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    helpers::date::get_current_academic_year,
    models::{
        club::Club, club_request::ClubRequest, enums::SubmissionStatus, student::Student,
        traits::TopLevelGetById as _,
    },
    permissions,
    prelude::*,
    query::QueryablePlaceholder,
};
use sqlx::query;
use uuid::Uuid;

#[post("/{id}/join")]
pub async fn join_clubs(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    LoggedInStudent(student_id): LoggedInStudent,
    club_id: Path<Uuid>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<(), QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let club_id = club_id.into_inner();
    let current_year = get_current_academic_year(None);
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/clubs/{club_id}/join")).await?;

    // Check if club exists
    let Club::Detailed(club, _) = Club::get_by_id(
        pool,
        club_id,
        Some(FetchLevel::Detailed),
        Some(FetchLevel::IdOnly),
        &*authorizer,
    )
    .await?
    else {
        unreachable!("Club::get_by_id should always return a Detailed variant")
    };

    // Check if the student is already a staff of the club
    if club.staffs.iter().any(|staff| match staff {
        Student::IdOnly(staff, _) => staff.id == student_id,
        _ => unreachable!("Staff should always be an IdOnly variant"),
    }) {
        return Err(Error::InvalidPermission(
            "Student is already a staff member of the club".to_string(),
            format!("/clubs/{club_id}/join"),
        ));
    }

    // Check if the student is already a member of the club
    if club.members.iter().any(|member| match member {
        Student::IdOnly(member, _) => member.id == student_id,
        _ => unreachable!("Staff should always be an IdOnly variant"),
    }) {
        return Err(Error::InvalidPermission(
            "Student is already a member of the club".to_string(),
            format!("/clubs/{club_id}/join"),
        ));
    }

    // Check if student has already requested to join the club
    if let Some(has_requested) = query!(
        "\
        SELECT membership_status AS \"membership_status: SubmissionStatus\" FROM club_members \
        WHERE club_id = $1 AND year = $2 and membership_status = $3 AND student_id = $4\
        ",
        club_id,
        current_year,
        SubmissionStatus::Pending as SubmissionStatus,
        student_id,
    )
    .fetch_optional(pool)
    .await?
    {
        match has_requested.membership_status {
            SubmissionStatus::Pending => {
                return Err(Error::InvalidPermission(
                    "Student has already requested to join the club".to_string(),
                    format!("/clubs/{club_id}/join"),
                ));
            }
            _ => unreachable!(),
        }
    }

    let club_member_id = query!(
        "\
        INSERT INTO club_members (club_id, year, membership_status, student_id)\
        VALUES ($1, $2, $3, $4) RETURNING id\
        ",
        club_id,
        current_year,
        SubmissionStatus::Pending as SubmissionStatus,
        student_id,
    )
    .fetch_one(pool)
    .await?
    .id;

    let club_request_id = ClubRequest::get_by_id(
        pool,
        club_member_id,
        fetch_level,
        descendant_fetch_level,
        &*authorizer,
    )
    .await?;
    let response = ResponseType::new(club_request_id, None);

    Ok(HttpResponse::Ok().json(response))
}
