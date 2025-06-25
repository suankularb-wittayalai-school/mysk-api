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
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
        string::MultiLangString,
    },
    models::{
        contact::{db::DbContact, Contact},
        enums::ContactType,
        student::db::DbStudent,
        traits::{GetById as _, TopLevelGetById as _},
    },
    permissions::Authorizer,
    prelude::*,
    query::QueryablePlaceholder,
};
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
    LoggedIn(user): LoggedIn,
    student_id: Path<Uuid>,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<StudentContactRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = student_id.into_inner();
    let Some(student_contact) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/students/{student_id}/contacts"),
        ));
    };
    let authorizer =
        Authorizer::new(pool, &user, format!("/students/{student_id}/contacts"))
            .await?;

    // Check if client is student
    let student = DbStudent::get_by_id(pool, student_id).await?;

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

    let mut transaction = pool.begin().await?;

    let new_contact_id = query!(
        "INSERT INTO contacts (type, value, name_th, name_en) VALUES ($1, $2, $3, $4) RETURNING id",
        student_contact.r#type as ContactType,
        student_contact.value,
        student_contact.name.th,
        student_contact.name.en,
    )
    .fetch_one(&mut *transaction)
    .await?
    .id;

    query!(
        "INSERT INTO person_contacts (person_id, contact_id) VALUES ($1, $2)",
        student.person_id,
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
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(new_contact, None);

    Ok(HttpResponse::Ok().json(response))
}
