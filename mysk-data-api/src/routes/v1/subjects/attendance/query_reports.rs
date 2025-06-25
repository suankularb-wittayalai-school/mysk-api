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
    permissions::Authorizer,
    prelude::*,
};

#[get("")]
pub async fn query_reports(
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
    }: RequestType<(), QueryableOnlineTeachingReports, SortableOnlineTeachingReports>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let authorizer =
        Authorizer::new(pool, &user, "/subjects/attendance".to_string()).await?;

    let reports = OnlineTeachingReports::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &authorizer,
    )
    .await?;

    let response = ResponseType::new(reports, None);

    Ok(HttpResponse::Ok().json(response))
}
