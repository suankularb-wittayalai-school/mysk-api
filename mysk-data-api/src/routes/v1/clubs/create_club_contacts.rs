use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
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
    models::{
        club::Club, contact::Contact, enums::ContactType, student::Student,
        traits::TopLevelGetById as _,
    },
    prelude::*,
};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ClubContactRequest {
    r#type: ContactType,
    value: String,
}

#[post("/{id}/contacts")]
pub async fn create_club_contacts(
    data: Data<AppState>,
    club_id: Path<Uuid>,
    student_id: LoggedInStudent,
    request_body: Json<RequestType<ClubContactRequest, QueryablePlaceholder, SortablePlaceholder>>,
    _: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = student_id.0;
    let club_id = club_id.into_inner();
    let club_contact = match &request_body.data {
        Some(club_contact) => club_contact,
        _ => unreachable!("JSON errors are pre-handled by the JsonConfig error handler"),
    };
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();

    // Check if the club exists
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
                "Club contact not found".to_string(),
                format!("/clubs/{club_id}/contacts"),
            ));
        }
        _ => unreachable!("Club::get_by_id should always return a Detailed variant"),
    };

    // Check if the student is a staff of the club
    if !club.staffs.iter().any(|staff| match staff {
        Student::IdOnly(staff, _) => staff.id == student_id,
        _ => unreachable!("Staff should always be an IdOnly variant"),
    }) {
        return Err(Error::InvalidPermission(
            "Student must be a staff of the club to create contacts".to_string(),
            format!("/clubs/{club_id}/contacts"),
        ));
    }

    // Check if the contact is a duplicate
    if club
        .contacts
        .iter()
        .any(|contact| contact.value == club_contact.value)
    {
        return Err(Error::InvalidRequest(
            "Contact with the same value already exists".to_string(),
            format!("/clubs/{club_id}/contacts"),
        ));
    }

    let new_contact_id = query!(
        "INSERT INTO contacts (type, value) VALUES ($1, $2) ON CONFLICT DO NOTHING RETURNING id",
        club_contact.r#type as ContactType,
        club_contact.value,
    )
    .fetch_one(pool)
    .await?
    .id;

    query!(
        "INSERT INTO club_contacts (club_id, contact_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        club.id,
        new_contact_id,
    )
    .execute(pool)
    .await?;

    let new_contact =
        Contact::get_by_id(pool, new_contact_id, fetch_level, descendant_fetch_level).await?;
    let response = ResponseType::new(new_contact, None);

    Ok(HttpResponse::Ok().json(response))
}
