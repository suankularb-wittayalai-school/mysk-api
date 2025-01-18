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
        elective_trade_offer::{
            request::{
                queryable::QueryableElectiveTradeOffer, sortable::SortableElectiveTradeOffer,
            },
            ElectiveTradeOffer,
        },
        traits::TopLevelQuery as _,
    },
    permissions,
    prelude::*,
};

#[get("")]
pub async fn query_trade_offers(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    request_body: RequestType<
        ElectiveTradeOffer,
        QueryableElectiveTradeOffer,
        SortableElectiveTradeOffer,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();
    let filter = request_body.filter.as_ref();
    let sort = request_body.sort.as_ref();
    let pagination = request_body.pagination.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, "/subjects/electives/trade-offers".to_string())
            .await?;

    let trade_offers = ElectiveTradeOffer::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &*authorizer,
    )
    .await?;

    let pagination = ElectiveTradeOffer::response_pagination(pool, filter, pagination).await?;
    let response = ResponseType::new(trade_offers, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
