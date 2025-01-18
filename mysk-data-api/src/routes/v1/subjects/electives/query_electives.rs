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
        elective_subject::{
            request::{queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject},
            ElectiveSubject,
        },
        traits::TopLevelQuery as _,
    },
    permissions,
    prelude::*,
};

#[get("")]
pub async fn query_elective_subject(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    request_query: RequestType<ElectiveSubject, QueryableElectiveSubject, SortableElectiveSubject>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();
    let filter = request_query.filter.as_ref();
    let sort = request_query.sort.as_ref();
    let pagination = request_query.pagination.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, "/subjects/electives".to_string()).await?;

    let electives = ElectiveSubject::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &*authorizer,
    )
    .await?;

    let pagination = ElectiveSubject::response_pagination(pool, filter, pagination).await?;
    let response = ResponseType::new(electives, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
