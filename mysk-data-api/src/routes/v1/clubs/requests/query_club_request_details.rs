use crate::{extractors::api_key::ApiKeyHeader, AppState};
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
    models::{club_request::ClubRequest, traits::TopLevelGetById as _},
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_club_request_details(
    data: Data<AppState>,
    club_id: Path<Uuid>,
    request_query: RequestType<ClubRequest, QueryablePlaceholder, SortablePlaceholder>,
    _: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;
    let club_id = club_id.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    let club_request =
        ClubRequest::get_by_id(pool, club_id, fetch_level, descendant_fetch_level).await?;
    let response = ResponseType::new(club_request, None);

    Ok(HttpResponse::Ok().json(response))
}
