use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{
    common::{
        requests::{EmptyRequestData, RequestType},
        response::ResponseType,
    },
    models::online_teaching_reports::{
        OnlineTeachingReports,
        requests::{
            queryable::QueryableOnlineTeachingReports, sortable::SortableOnlineTeachingReports,
        },
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
    }: RequestType<
        EmptyRequestData,
        QueryableOnlineTeachingReports,
        SortableOnlineTeachingReports,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let authorizer = Authorizer::new(&user, "/subjects/attendance".to_string());

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
