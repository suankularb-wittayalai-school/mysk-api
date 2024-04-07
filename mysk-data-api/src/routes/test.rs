use actix_web::{get, web, HttpResponse, Responder};

// use mysk_lib::models::common::requests::FetchLevel;
use mysk_lib::models::common::requests::RequestType;
use mysk_lib::models::common::traits::TopLevelGetById;
use mysk_lib::models::elective_subject::ElectiveSubject;
use mysk_lib::models::*;
use mysk_lib::prelude::*;
// use mysk_lib_macros::traits::db::GetById;
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

    let model_id = Uuid::parse_str("d05c155a-ebe0-456b-b289-7b0eb1487f04").unwrap();

    let fetch_level = request_query.fetch_level.as_ref();

    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    let model = elective_subject::ElectiveSubject::get_by_id(
        pool,
        model_id,
        fetch_level,
        descendant_fetch_level,
    )
    .await?;

    // let model = elective_trade_offer::ElectiveTradeOffer::get_by_id(
    //     pool,
    //     model_id,
    //     fetch_level,
    //     descendant_fetch_level,
    // )
    // .await?;

    let response = common::response::ResponseType::new(model, None);

    Ok(HttpResponse::Ok().json(response))
}
