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
    query::{QueryParam, QueryablePlaceholder, SqlSetClause},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ModifyContactsRequest {
    pub name: Option<FlexibleMultiLangString>,
    pub r#type: Option<ContactType>,
    pub value: Option<String>,
}

#[put("/{id}")]
pub async fn modify_contacts(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    contact_id: Path<Uuid>,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<ModifyContactsRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let contact_id = contact_id.into_inner();
    let Some(contact) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/contacts/{contact_id}"),
        ));
    };
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/contacts/{contact_id}")).await?;

    // Check if the contact exists
    let db_contact = DbContact::get_by_id(pool, contact_id).await?;

    authorizer
        .authorize_contact(&db_contact, pool, ActionType::Update)
        .await?;

    let mut qb = SqlSetClause::new();
    qb.push_multilang_update_field("name", contact.name)
        .push_update_field("type", contact.r#type, QueryParam::ContactType)
        .push_update_field("value", contact.value, QueryParam::String);

    let mut qb = qb.into_query_builder("UPDATE contacts");
    qb.push(" WHERE id = ")
        .push_bind(contact_id)
        .build()
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
