use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{
    common::{
        requests::RequestType,
        response::{MetadataType, ResponseType},
    },
    models::{
        club_request::{
            ClubRequest,
            request::{queryable::QueryableClubRequest, sortable::SortableClubRequest},
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
    let mut conn = data.db.acquire().await?;
    let authorizer = Authorizer::new(&mut conn, &user, "/clubs/requests".to_string()).await?;

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
