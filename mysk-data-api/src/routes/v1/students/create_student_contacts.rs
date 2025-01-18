use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    AppState,
};
use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
        string::MultiLangString,
    },
    models::{
        contact::{db::DbContact, Contact},
        enums::ContactType,
        student::db::DbStudent,
        traits::TopLevelGetById as _,
    },
    permissions::{self, ActionType},
    prelude::*,
};
use mysk_lib_macros::traits::db::GetById as _;
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct StudentContactRequest {
    name: MultiLangString,
    r#type: ContactType,
    value: String,
}

#[post("/{id}/contacts")]
pub async fn create_student_contacts(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    student_id: Path<Uuid>,
    request_body: Json<
        RequestType<StudentContactRequest, QueryablePlaceholder, SortablePlaceholder>,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let student_id = student_id.into_inner();
    let Some(student_contact) = &request_body.data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/students/{student_id}/contacts"),
        ));
    };
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/students/{student_id}/contacts"))
            .await?;

    // Fetch and authorize db_student instance
    let student = DbStudent::get_by_id(pool, student_id).await?;
    authorizer
        // TODO: Fix later
        .authorize_student(&student, pool, ActionType::Create)
        .await?;

    // Check for duplicate contacts
    let existing_contacts = DbStudent::get_student_contacts(pool, student_id).await?;
    for contact_id in existing_contacts {
        let contact = DbContact::get_by_id(pool, contact_id).await?;
        if contact.r#type == student_contact.r#type && contact.value == student_contact.value {
            return Err(Error::InvalidRequest(
                "Contact with the same value already exists".to_string(),
                format!("/students/{student_id}/contacts"),
            ));
        }
    }

    // Insert the new contact
    let new_contact_id = query!(
        "INSERT INTO contacts (type, value, name_th, name_en) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING RETURNING id",
        student_contact.r#type as ContactType,
        student_contact.value,
        student_contact.name.th,
        student_contact.name.en,
    )
    .fetch_one(pool)
    .await?
    .id;

    query!(
        "INSERT INTO person_contacts (person_id, contact_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        student.person_id,
        new_contact_id,
    )
    .execute(pool)
    .await?;

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
