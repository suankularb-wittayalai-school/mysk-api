use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{
    common::{
        requests::{EmptyRequestData, RequestType},
        response::{MetadataType, ResponseType},
    },
    models::contact::{
        Contact,
        request::{queryable::QueryableContact, sortable::SortableContact},
    },
    permissions::Authorizer,
    prelude::*,
};

#[get("")]
pub async fn query_contacts(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    RequestType {
        pagination,
        filter,
        sort,
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<EmptyRequestData, QueryableContact, SortableContact>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let authorizer = Authorizer::new(&mut conn, &user, "/contacts".to_string()).await?;

    let (contacts, pagination) = Contact::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(contacts, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
