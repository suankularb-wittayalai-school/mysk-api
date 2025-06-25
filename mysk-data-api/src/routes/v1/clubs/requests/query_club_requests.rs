use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    AppState,
};
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::{
        requests::RequestType,
        response::{MetadataType, ResponseType},
    },
    models::{
        club_request::{
            request::{queryable::QueryableClubRequest, sortable::SortableClubRequest},
            ClubRequest,
        },
        traits::TopLevelQuery as _,
    },
    permissions::Authorizer,
    prelude::*,
};

#[get("")]
pub async fn query_club_requests(
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
    }: RequestType<ClubRequest, QueryableClubRequest, SortableClubRequest>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let authorizer =
        Authorizer::new(pool, &user, "/clubs/requests".to_string()).await?;

    let (club_requests, pagination) = ClubRequest::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(club_requests, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
