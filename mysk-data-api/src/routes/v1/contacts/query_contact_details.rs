use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    AppState,
};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::{contact::Contact, traits::TopLevelGetById as _},
    permissions,
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_contact_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    contact_id: Path<Uuid>,
    request_query: RequestType<Contact, QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let contact_id = contact_id.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/contacts/{contact_id}")).await?;

    let contact = Contact::get_by_id(
        pool,
        contact_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(contact, None);

    Ok(HttpResponse::Ok().json(response))
}
