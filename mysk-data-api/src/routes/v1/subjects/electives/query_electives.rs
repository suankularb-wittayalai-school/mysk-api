use crate::{extractors::api_key::ApiKeyHeader, AppState};
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::{
        requests::RequestType,
        response::{MetadataType, ResponseType},
    },
    models::{
        elective_subject::{
            request::{queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject},
            ElectiveSubject,
        },
        traits::TopLevelQuery as _,
    },
    prelude::*,
};

#[get("")]
pub async fn query_elective_subject(
    data: Data<AppState>,
    request_query: RequestType<ElectiveSubject, QueryableElectiveSubject, SortableElectiveSubject>,
    _api_key: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;
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
