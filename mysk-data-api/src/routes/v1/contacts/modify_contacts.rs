use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{
    HttpResponse, Responder, put,
    web::{Data, Json, Path},
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
        string::FlexibleMultiLangString,
    },
    models::{
        contact::{Contact, db::DbContact},
        enums::ContactType,
        traits::{GetById as _, TopLevelGetById as _},
    },
    permissions::{ActionType, Authorizable as _, Authorizer},
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
    let mut conn = data.db.acquire().await?;
    let contact_id = contact_id.into_inner();
    let Some(contact) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/contacts/{contact_id}"),
        ));
    };
    let authorizer = Authorizer::new(&mut conn, &user, format!("/contacts/{contact_id}")).await?;

    // Check if the contact exists
    let db_contact = DbContact::get_by_id(&mut conn, contact_id).await?;

    authorizer
        .authorize_contact(&db_contact, &mut conn, ActionType::Update)
        .await?;

    let mut qb = SqlSetClause::new();
    qb.push_multilang_update_field("name", contact.name)
        .push_update_field("type", contact.r#type, QueryParam::ContactType)
        .push_update_field("value", contact.value, QueryParam::String);

    let mut qb = qb.into_query_builder("UPDATE contacts");
    qb.push(" WHERE id = ")
        .push_bind(contact_id)
        .build()
        .execute(&mut *conn)
        .await?;

    let updated_contact = Contact::get_by_id(
        pool,
        contact_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(updated_contact, None);

    Ok(HttpResponse::Ok().json(response))
}
