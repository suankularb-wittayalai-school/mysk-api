use crate::{extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn}, AppState};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
    }, models::{club::Club, traits::TopLevelGetById as _}, permissions::roles::get_authorizer, prelude::*
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_club_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    club_id: Path<Uuid>,
    request_query: RequestType<Club, QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user_id = user.0;
    let club_id = club_id.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();
    let authorizer = get_authorizer(&user_id);

    let club = Club::get_by_id(pool, club_id, fetch_level, descendant_fetch_level, &authorizer).await?;
    let response = ResponseType::new(club, None);

    Ok(HttpResponse::Ok().json(response))
}
