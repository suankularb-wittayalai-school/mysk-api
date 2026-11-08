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
    models::certificate::{
        Certificate,
        request::{queryable::QueryableCertificate, sortable::SortableCertificate},
    },
    permissions::Authorizer,
    prelude::*,
};

#[get("")]
pub async fn query_certificates(
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
    }: RequestType<EmptyRequestData, QueryableCertificate, SortableCertificate>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let authorizer = Authorizer::new(&user, "/certificates".to_string());

    let (certificates, pagination) = Certificate::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &authorizer,
    )
    .await?;

    let response = ResponseType::new(certificates, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}