use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use mysk_lib::{
    common::{requests::RequestType, response::ResponseType},
    models::contact::Contact,
    permissions::Authorizer,
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_contact_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    contact_id: Path<Uuid>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let contact_id = contact_id.into_inner();
    let authorizer = Authorizer::new(&mut conn, &user, format!("/contacts/{contact_id}")).await?;

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
