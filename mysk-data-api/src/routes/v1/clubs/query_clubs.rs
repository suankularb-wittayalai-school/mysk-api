use crate::{extractors::api_key::ApiKeyHeader, AppState};
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::{
        requests::RequestType,
        response::{MetadataType, ResponseType},
    },
    models::{
        club::{
            request::{queryable::QueryableClub, sortable::SortableClub},
            Club,
        },
        traits::TopLevelQuery as _,
    },
    prelude::*,
};

#[get("")]
pub async fn query_clubs(
    data: Data<AppState>,
    _: ApiKeyHeader,
    request_query: RequestType<Club, QueryableClub, SortableClub>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();
    let filter = request_query.filter.as_ref();
    let sort = request_query.sort.as_ref();
    let pagination = request_query.pagination.as_ref();

    let clubs = Club::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
    )
    .await?;

    let pagination = Club::response_pagination(pool, filter, pagination).await?;
    let response = ResponseType::new(clubs, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
