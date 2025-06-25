use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    AppState,
};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::{online_teaching_reports::OnlineTeachingReports, traits::TopLevelGetById},
    permissions::Authorizer,
    prelude::*,
    query::QueryablePlaceholder,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_report_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    online_teaching_report_id: Path<Uuid>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<OnlineTeachingReports, QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let online_teaching_report_id = online_teaching_report_id.into_inner();
    let authorizer = Authorizer::new(
        pool,
        &user,
        format!("/subjects/attendance/{online_teaching_report_id}"),
    )
    .await?;

    let report = OnlineTeachingReports::get_by_id(
        pool,
        online_teaching_report_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(report, None);

    Ok(HttpResponse::Ok().json(response))
}
