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
    models::elective_trade_offer::{
        ElectiveTradeOffer,
        request::{queryable::QueryableElectiveTradeOffer, sortable::SortableElectiveTradeOffer},
    },
    permissions::Authorizer,
    prelude::*,
};

#[get("")]
pub async fn query_trade_offers(
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
    }: RequestType<(), QueryableElectiveTradeOffer, SortableElectiveTradeOffer>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let authorizer = Authorizer::new(
        &mut conn,
        &user,
        "/subjects/electives/trade-offers".to_string(),
    )
    .await?;

    let (trade_offers, pagination) = ElectiveTradeOffer::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(trade_offers, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
