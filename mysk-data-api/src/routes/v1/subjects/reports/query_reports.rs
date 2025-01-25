use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    AppState,
};
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::{requests::RequestType, response::ResponseType},
    models::{
        online_teaching_reports::{
            requests::{
                queryable::QueryableOnlineTeachingReports, sortable::SortableOnlineTeachingReports,
            },
            OnlineTeachingReports,
        },
        traits::TopLevelQuery,
    },
    permissions,
    prelude::*,
};

#[get("")]
pub async fn query_reports(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    request_query: RequestType<
        OnlineTeachingReports,
        QueryableOnlineTeachingReports,
        SortableOnlineTeachingReports,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();
    let filter = request_query.filter.as_ref();
    let sort = request_query.sort.as_ref();
    let pagination = request_query.pagination.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, "/subjects/reports".to_string()).await?;

    let reports = OnlineTeachingReports::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &*authorizer,
    )
    .await?;

    let response = ResponseType::new(reports, None);

    Ok(HttpResponse::Ok().json(response))
}
