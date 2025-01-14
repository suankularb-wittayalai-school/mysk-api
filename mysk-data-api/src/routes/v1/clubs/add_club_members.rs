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
        requests::{FetchLevel, QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    helpers::date::get_current_academic_year,
    models::{
        club::Club, club_request::ClubRequest, enums::SubmissionStatus, student::Student,
        traits::TopLevelGetById as _,
    },
    permissions,
    prelude::*,
};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct AddClubMemberRequest {
    id: Uuid,
}

#[allow(clippy::similar_names, clippy::too_many_lines)]
#[post("/{id}/add")]
pub async fn add_club_members(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    student_id: LoggedInStudent,
    club_id: Path<Uuid>,
    request_body: Json<
        RequestType<AddClubMemberRequest, QueryablePlaceholder, SortablePlaceholder>,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let inviter_student_id = student_id.0;
    let club_id = club_id.into_inner();
    let invitee_student_id = match &request_body.data {
        Some(request_data) => request_data.id,
        None => {
            return Err(Error::InvalidRequest(
                "Json deserialize error: field `data` can not be empty".to_string(),
                format!("/clubs/{club_id}/add"),
            ));
        }
    };
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/clubs/{club_id}/add")).await?;
    let current_year = get_current_academic_year(None);

    // Check if the invitee student exists
    match Student::get_by_id(
        pool,
        invitee_student_id,
        Some(&FetchLevel::Default),
        Some(&FetchLevel::IdOnly),
        &*authorizer,
    )
    .await
    {
        Ok(Student::Default(student, _)) => {
            if student.classroom.is_none() {
                return Err(Error::EntityNotFound(
                    "Invitee student not found".to_string(),
                    format!("/clubs/{club_id}/add"),
                ));
            }
        }
        Err(Error::InternalSeverError(_, _)) => {
            return Err(Error::EntityNotFound(
                "Invitee student not found".to_string(),
                format!("/clubs/{club_id}/add"),
            ));
        }
        _ => unreachable!("Student::get_by_id should always return a Default variant"),
    };

    // Check if the club exists
    let club = match Club::get_by_id(
        pool,
        club_id,
        Some(&FetchLevel::Detailed),
        Some(&FetchLevel::IdOnly),
        &*authorizer,
    )
    .await
    {
        Ok(Club::Detailed(club, _)) => club,
        Err(Error::InternalSeverError(_, _)) => {
            return Err(Error::EntityNotFound(
                "Club contact not found".to_string(),
                format!("/clubs/{club_id}/add"),
            ));
        }
        _ => unreachable!("Club::get_by_id should always return a Detailed variant"),
    };

    // Check if the inviting student is a staff of the club
    if !club.staffs.iter().any(|staff| match staff {
        Student::IdOnly(staff, _) => staff.id == inviter_student_id,
        _ => unreachable!("Staff should always be an IdOnly variant"),
    }) {
        return Err(Error::InvalidPermission(
            "Student must be a staff of the club to add club members".to_string(),
            format!("/clubs/{club_id}/add"),
        ));
    }

    let mut insert_new_member: bool = true;

    // Check if the invitee student is already a club staff
    if club.staffs.iter().any(|staff| match staff {
        Student::IdOnly(staff, _) => staff.id == invitee_student_id,
        _ => unreachable!("Staff should always be an IdOnly variant"),
    }) {
        return Err(Error::InvalidPermission(
            "Invitee student is already a staff member of the club".to_string(),
            format!("/clubs/{club_id}/add"),
        ));
    }

    // Check if the invitee student is already a club member or a club request already exists
    if let Some(club_request) = query!(
        r#"
        SELECT membership_status AS "membership_status: SubmissionStatus" FROM club_members
        WHERE club_id = $1 AND year = $2 AND membership_status != $3 AND student_id = $4
        "#,
        club.id,
        current_year,
        SubmissionStatus::Declined as SubmissionStatus,
        invitee_student_id,
    )
    .fetch_optional(pool)
    .await?
    {
        match club_request.membership_status {
            SubmissionStatus::Approved => {
                return Err(Error::InvalidPermission(
                    "Invitee student is already a member of the club".to_string(),
                    format!("/clubs/{club_id}/add"),
                ))
            }
            SubmissionStatus::Pending => insert_new_member = false,
            SubmissionStatus::Declined => unreachable!(),
        }
    }

    let club_member_id = if insert_new_member {
        query!(
            "
            INSERT INTO club_members (club_id, year, membership_status, student_id)
            VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING RETURNING id
            ",
            club.id,
            current_year,
            SubmissionStatus::Approved as SubmissionStatus,
            invitee_student_id,
        )
        .fetch_one(pool)
        .await?
        .id
    } else {
        query!(
            "
            UPDATE club_members SET membership_status = $1
            WHERE club_id = $2 AND year = $3 AND student_id = $4 RETURNING id
            ",
            SubmissionStatus::Approved as SubmissionStatus,
            club.id,
            current_year,
            invitee_student_id,
        )
        .fetch_one(pool)
        .await?
        .id
    };

    let club_member = ClubRequest::get_by_id(
        pool,
        club_member_id,
        fetch_level,
        descendant_fetch_level,
        &*authorizer,
    )
    .await?;
    let response = ResponseType::new(club_member, None);

    Ok(HttpResponse::Ok().json(response))
}
