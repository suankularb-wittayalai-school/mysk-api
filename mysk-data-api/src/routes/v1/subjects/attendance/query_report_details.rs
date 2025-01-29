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
    permissions,
    prelude::*,
    query::QueryablePlaceholder,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_report_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    online_teaching_report_id: Path<Uuid>,
    request_query: RequestType<OnlineTeachingReports, QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let online_teaching_report_id = online_teaching_report_id.into_inner();
    let fetch_level = request_query.fetch_level;
    let descendant_fetch_level = request_query.descendant_fetch_level;
    let authorizer = permissions::get_authorizer(
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
        &*authorizer,
    )
    .await?;
    let response = ResponseType::new(report, None);

    Ok(HttpResponse::Ok().json(response))
}
