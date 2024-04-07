use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};

use mysk_lib::models::common::requests::FetchLevel;
use mysk_lib::models::common::requests::RequestType;
use mysk_lib::models::common::traits::TopLevelGetById;
use mysk_lib::models::elective_subject::ElectiveSubject;
use mysk_lib::models::*;
use mysk_lib::prelude::*;
use mysk_lib_macros::traits::db::GetById;
use uuid::Uuid;

use crate::AppState;

#[utoipa::path(path = "/test", tag = "Global")]
#[get("/test")]
pub async fn test(
    data: web::Data<AppState>,
    request_query: web::Query<
        RequestType<
            ElectiveSubject,
            common::requests::QueryablePlaceholder,
            common::requests::SortablePlaceholder,
        >,
    >,
) -> Result<impl Responder> {
    let pool: &sqlx::PgPool = &data.db;

    let elective_id = Uuid::parse_str("ee921113-12a4-4358-8043-6d0eb5ea64f1").unwrap();

    let fetch_level = request_query.fetch_level.as_ref();

    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    // let elective = elective_subject::ElectiveSubject::get_by_id(
    //     pool,
    //     elective_id,
    //     fetch_level,
    //     descendant_fetch_level,
    // )
    // .await?;

    let trade_offer =
        elective_trade_offer::db::DbElectiveTradeOffer::get_by_id(pool, elective_id).await?;

    let response = common::response::ResponseType::new(trade_offer, None);

    Ok(HttpResponse::Ok().json(response))
}
