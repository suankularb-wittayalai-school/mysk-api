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
        club::{
            request::{queryable::QueryableClub, sortable::SortableClub},
            Club,
        },
        traits::TopLevelQuery as _,
    },
    permissions,
    prelude::*,
};

#[get("")]
pub async fn query_clubs(
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
    }: RequestType<(), QueryableClub, SortableClub>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let authorizer = permissions::get_authorizer(pool, &user, "/clubs".to_string()).await?;

    let (clubs, pagination) = Club::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &*authorizer,
    )
    .await?;
    let response = ResponseType::new(clubs, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
