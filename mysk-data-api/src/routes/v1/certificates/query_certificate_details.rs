use crate::{
    AppState, 
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn}
};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use mysk_lib::{
    common::{requests::RequestType, response::ResponseType},
    models::certificate::Certificate,
    permissions::Authorizer,
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_certificate_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    certificate_id: Path<Uuid>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType,
) -> Result<impl Responder> {
    let pool = &data.db;
    let certificate_id = certificate_id.into_inner();
    let authorizer = Authorizer::new(&user, format!("/certificates/{certificate_id}"));

    let certificate = Certificate::get_by_id(
        pool,
        certificate_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(certificate, None);

    Ok(HttpResponse::Ok().json(response))
}
