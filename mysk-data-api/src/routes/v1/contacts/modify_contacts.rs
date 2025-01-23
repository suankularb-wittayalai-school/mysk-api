use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    AppState,
};
use actix_web::{
    put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
        string::FlexibleMultiLangString,
    },
    models::{
        contact::{db::DbContact, Contact},
        enums::ContactType,
        traits::{GetById as _, TopLevelGetById as _},
    },
    permissions::{self, ActionType},
    prelude::*,
    query::QueryablePlaceholder,
};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ModifyContactsRequest {
    pub name: Option<FlexibleMultiLangString>,
    pub r#type: Option<ContactType>,
    pub value: Option<String>,
    pub include_students: Option<bool>,
    pub include_teachers: Option<bool>,
    pub include_parents: Option<bool>,
}

#[put("/{id}")]
pub async fn modify_contacts(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    contact_id: Path<Uuid>,
    request_body: Json<
        RequestType<ModifyContactsRequest, QueryablePlaceholder, SortablePlaceholder>,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let contact_id = contact_id.into_inner();
    let Some(contact) = &request_body.data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/contacts/{contact_id}"),
        ));
    };
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/contacts/{contact_id}")).await?;

    // Check if the contact exists
    let db_contact = DbContact::get_by_id(pool, contact_id).await?;

    authorizer
        .authorize_contact(&db_contact, pool, ActionType::Update)
        .await?;
    query!(
        "
        UPDATE contacts SET
            name_en = COALESCE($1, name_en),
            name_th = COALESCE($2, name_th),
            type = COALESCE($3, type),
            value = COALESCE($4, value),
            include_students = COALESCE($5, include_students),
            include_teachers = COALESCE($6, include_teachers),
            include_parents  = COALESCE($7, include_parents)
        WHERE id = $8
        ",
        if contact.name.is_some() {
            contact.name.clone().unwrap().en
        } else {
            None
        },
        if contact.name.is_some() {
            contact.name.clone().unwrap().th
        } else {
            None
        },
        contact.r#type as Option<ContactType>,
        contact.value,
        contact.include_students,
        contact.include_teachers,
        contact.include_parents,
        contact_id,
    )
    .execute(pool)
    .await?;

    let updated_contact = Contact::get_by_id(
        pool,
        contact_id,
        fetch_level,
        descendant_fetch_level,
        &*authorizer,
    )
    .await?;
    let response = ResponseType::new(updated_contact, None);

    Ok(HttpResponse::Ok().json(response))
}
