use actix_web::HttpRequest;
use actix_web::{get, web, HttpResponse, Responder};

// use mysk_lib::models::common::requests::FetchLevel;
use mysk_lib::models::common::requests::RequestType;
use mysk_lib::models::common::traits::QueryDb;
use mysk_lib::models::common::traits::TopLevelGetById;
use mysk_lib::models::elective_subject::request::queryable::QueryableElectiveSubject;
use mysk_lib::models::elective_subject::request::sortable::SortableElectiveSubject;
use mysk_lib::models::elective_subject::ElectiveSubject;
use mysk_lib::models::*;
use mysk_lib::prelude::*;
// use mysk_lib_macros::traits::db::GetById;
use uuid::Uuid;

use crate::AppState;

#[utoipa::path(path = "/test", tag = "Global")]
#[get("/test")]
pub async fn test(data: web::Data<AppState>, request: HttpRequest) -> Result<impl Responder> {
    let pool: &sqlx::PgPool = &data.db;
    let request_query = serde_qs::from_str::<
        RequestType<ElectiveSubject, QueryableElectiveSubject, SortableElectiveSubject>,
    >(request.query_string())
    .unwrap();

    let model_id = Uuid::parse_str("d05c155a-ebe0-456b-b289-7b0eb1487f04").unwrap();

    let fetch_level = request_query.fetch_level.as_ref();

    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    let filter = request_query.filter.as_ref();
    let sort = request_query.sort.as_ref();
    let pagination = request_query.pagination.as_ref();
    // let model = elective_subject::ElectiveSubject::get_by_id(
    //     pool,
    //     model_id,
    //     fetch_level,
    //     descendant_fetch_level,
    // )
    // .await?;

    dbg!(&pagination);

    let model =
        elective_subject::db::DbElectiveSubject::query(pool, filter, sort, pagination).await?;

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
