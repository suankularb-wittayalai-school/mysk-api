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
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::{club_request::ClubRequest, traits::TopLevelGetById as _},
    permissions,
    prelude::*,
    query::QueryablePlaceholder,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_club_request_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    club_request_id: Path<Uuid>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<(), QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let club_request_id = club_request_id.into_inner();
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/clubs/requests/{club_request_id}"))
            .await?;

    let club_request = ClubRequest::get_by_id(
        pool,
        club_request_id,
        fetch_level,
        descendant_fetch_level,
        &*authorizer,
    )
    .await?;
    let response = ResponseType::new(club_request, None);

    Ok(HttpResponse::Ok().json(response))
}
