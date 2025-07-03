use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use mysk_lib::{
    common::{requests::RequestType, response::ResponseType},
    models::cheer_practice_period::CheerPracticePeriod,
    permissions::Authorizer,
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_practice_period_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    practice_period_id: Path<Uuid>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let practice_period_id = practice_period_id.into_inner();
    let authorizer = Authorizer::new(&mut conn, &user, format!("/attendance/cheer/periods/{practice_period_id}")).await?;

    let practice_period = CheerPracticePeriod::get_by_id(
        pool,
        practice_period_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    
    let response = ResponseType::new(practice_period, None);

    Ok(HttpResponse::Ok().json(response))
}