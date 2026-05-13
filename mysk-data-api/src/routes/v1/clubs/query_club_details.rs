use crate::{AppState, extractors::api_key::ApiKeyHeader};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use mysk_lib::{
    common::{requests::RequestType, response::ResponseType},
    models::club::Club,
    permissions::{Authorizer, roles::AdminRole},
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_club_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    club_id: Path<Uuid>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType,
) -> Result<impl Responder> {
    let pool = &data.db;
    let club_id = club_id.into_inner();
    let authorizer = Authorizer::Admin(AdminRole);

    let club = Club::get_by_id(
        pool,
        club_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(club, None);

    Ok(HttpResponse::Ok().json(response))
}
