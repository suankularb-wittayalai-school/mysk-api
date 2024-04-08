use actix_web::HttpRequest;
use actix_web::{get, web, HttpResponse, Responder};

// use mysk_lib::models::common::requests::FetchLevel;
use mysk_lib::models::common::traits::TopLevelQuery;
use mysk_lib::models::common::{requests::RequestType, response::ResponseType};
use mysk_lib::models::elective_subject::request::{
    queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject,
};
use mysk_lib::models::elective_subject::ElectiveSubject;
use mysk_lib::prelude::*;
// use mysk_lib_macros::traits::db::GetById;

use crate::AppState;

#[get("/")]
pub async fn query_elective_subject(
    data: web::Data<AppState>,
    request: HttpRequest,
) -> Result<impl Responder> {
    let pool: &sqlx::PgPool = &data.db;
    let request_query = serde_qs::from_str::<
        RequestType<ElectiveSubject, QueryableElectiveSubject, SortableElectiveSubject>,
    >(request.query_string());

    let request_query = match request_query {
        Ok(query) => query,
        Err(e) => {
            return Err(Error::InvalidRequest(
                e.to_string(),
                "/v1/subjects/electives/".to_string(),
            ));
        }
    };

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

    let response = ResponseType::new(electives, None);

    Ok(HttpResponse::Ok().json(response))
}
