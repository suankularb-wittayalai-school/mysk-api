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
        requests::{FetchLevel, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::{
        club::db::DbClub,
        contact::Contact,
        enums::ContactType,
        traits::{GetById as _, TopLevelGetById as _},
    },
    permissions,
    prelude::*,
    query::QueryablePlaceholder,
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
    _: ApiKeyHeader,
    user: LoggedIn,
    student_id: LoggedInStudent,
    club_id: Path<Uuid>,
    request_body: Json<RequestType<ClubContactRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let student_id = student_id.0;
    let club_id = club_id.into_inner();
    let Some(club_contact) = &request_body.data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/clubs/{club_id}/contacts"),
        ));
    };
    let fetch_level = request_body.fetch_level;
    let descendant_fetch_level = request_body.descendant_fetch_level;
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/clubs/{club_id}/contacts")).await?;

    let club = DbClub::get_by_id(pool, club_id).await?;

    // Check if the student is a staff of the club
    let club_staffs = DbClub::get_club_staffs(pool, club_id).await?;
    if !club_staffs.iter().any(|staff_id| *staff_id == student_id) {
        return Err(Error::InvalidPermission(
            "Insufficient permissions to perform this action".to_string(),
            format!("/clubs/{club_id}/contacts"),
        ));
    }

    // Check if the contact is a duplicate
    let club_contacts = Contact::get_by_ids(
        pool,
        DbClub::get_club_contacts(pool, club_id).await?,
        Some(FetchLevel::Default),
        Some(FetchLevel::IdOnly),
        &*authorizer,
    )
    .await?;
    if club_contacts.iter().any(|contact| match contact {
        Contact::Default(contact, _) => contact.value == club_contact.value,
        _ => unreachable!("Contact::get_by_ids should always return a Default variant"),
    }) {
        return Err(Error::InvalidRequest(
            "Contact with the same value already exists".to_string(),
            format!("/clubs/{club_id}/contacts"),
        ));
    }

    let mut transaction = pool.begin().await?;

    let new_contact_id = query!(
        "INSERT INTO contacts (type, value) VALUES ($1, $2) RETURNING id",
        club_contact.r#type as ContactType,
        club_contact.value,
    )
    .fetch_one(&mut *transaction)
    .await?
    .id;

    query!(
        "INSERT INTO club_contacts (club_id, contact_id) VALUES ($1, $2)",
        club.id,
        new_contact_id,
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    let new_contact = Contact::get_by_id(
        pool,
        new_contact_id,
        fetch_level,
        descendant_fetch_level,
        &*authorizer,
    )
    .await?;
    let response = ResponseType::new(new_contact, None);

    Ok(HttpResponse::Ok().json(response))
}
