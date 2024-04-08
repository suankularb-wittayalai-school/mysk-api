use actix_web::{get, web, HttpResponse, Responder};

use mysk_lib::prelude::*;

use mysk_lib::models::common::response::MetadataType;
use mysk_lib::models::common::traits::TopLevelQuery;
use mysk_lib::models::common::{requests::RequestType, response::ResponseType};
use mysk_lib::models::elective_subject::request::{
    queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject,
};
use mysk_lib::models::elective_subject::ElectiveSubject;

use crate::middlewares::api_key::HaveApiKey;
use crate::AppState;

#[get("/")]
pub async fn query_elective_subject(
    data: web::Data<AppState>,
    request_query: RequestType<ElectiveSubject, QueryableElectiveSubject, SortableElectiveSubject>,
    _: HaveApiKey,
) -> Result<impl Responder> {
    let pool: &sqlx::PgPool = &data.db;

    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();
    let filter = request_query.filter.as_ref();
    let sort = request_query.sort.as_ref();
    let pagination = request_query.pagination.as_ref();

    let electives = ElectiveSubject::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
    )
    .await?;

    let pagination = ElectiveSubject::response_pagination(pool, filter, pagination).await?;

    let response = ResponseType::new(electives, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
