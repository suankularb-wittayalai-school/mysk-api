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
        requests::{FetchLevel, QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
        string::MultiLangString,
    },
    models::{
        contact::Contact,
        enums::ContactType,
        teacher::{db::DbTeacher, Teacher},
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
struct TeacherContactRequest {
    name: MultiLangString,
    r#type: ContactType,
    value: String,
}

#[post("/{id}/contacts")]
pub async fn create_teacher_contacts(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    teacher_id: Path<Uuid>,
    request_body: Json<
        RequestType<TeacherContactRequest, QueryablePlaceholder, SortablePlaceholder>,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let teacher_id = teacher_id.into_inner();
    let Some(teacher_contact) = &request_body.data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/teachers/{teacher_id}/contacts"),
        ));
    };
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/teachers/{teacher_id}/contacts"))
            .await?;

    // Fetch the DbTeacher instance
    let db_teacher = DbTeacher::get_by_id(pool, teacher_id).await?;

    // Authorize the action
    authorizer
        .authorize_teacher(&db_teacher, pool, ActionType::Create)
        .await?;

    // Check if the teacher exists
    let teacher = match Teacher::get_by_id(
        pool,
        teacher_id,
        Some(&FetchLevel::Detailed),
        Some(&FetchLevel::IdOnly),
        &authorizer,
    )
    .await
    {
        Ok(Teacher::Detailed(teacher, _)) => teacher,
        Err(Error::InternalSeverError(_, _)) => {
            return Err(Error::EntityNotFound(
                "Teacher not found".to_string(),
                format!("/teachers/{teacher_id}/contacts"),
            ));
        }
        _ => unreachable!("Teacher::get_by_id should always return a Detailed variant"),
    };

    // Check for duplicate contact values
    if teacher
        .contacts
        .iter()
        .any(|contact| contact.value == teacher_contact.value)
    {
        return Err(Error::InvalidRequest(
            "Contact with same value already exists".to_string(),
            format!("/teachers/{teacher_id}/contacts"),
        ));
    }

    // Insert the new contact
    let new_contact_id = query!(
        "INSERT INTO contacts (type, value, name_th, name_en) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING RETURNING id",
        teacher_contact.r#type as ContactType,
        teacher_contact.value,
        teacher_contact.name.th,
        teacher_contact.name.en,
    )
    .fetch_one(pool)
    .await?
    .id;

    query!(
        "INSERT INTO person_contacts (person_id, contact_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        teacher.person.as_ref().unwrap().id,
        new_contact_id,
    )
    .execute(pool)
    .await?;

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
