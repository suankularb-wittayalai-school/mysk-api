use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::online_teaching_reports::OnlineTeachingReports,
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
    let mut conn = data.db.acquire().await?;
    let online_teaching_report_id = online_teaching_report_id.into_inner();
    let authorizer = Authorizer::new(
        &mut conn,
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
