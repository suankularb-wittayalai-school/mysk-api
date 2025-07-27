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
    models::cheer_practice_attendance::{CheerPracticeAttendance, db::DbCheerPracticeAttendance},
    permissions::Authorizer,
    prelude::*,
};

use uuid::Uuid;

#[get("/{id}/attendance/cheer")]
pub async fn query_cheer_practice_attendance(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    id: Path<Uuid>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let student_id = id.into_inner();
    let authorizer = Authorizer::new(
        &mut conn,
        &user,
        format!("/students/{student_id}/attendance/cheer"),
    )
    .await?;

    let ids = DbCheerPracticeAttendance::get_by_student_id(&mut conn, student_id).await?;
    let cheer_practice_attendances = CheerPracticeAttendance::get_by_ids(
        pool,
        &ids,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;

    let response = ResponseType::new(cheer_practice_attendances, None);

    Ok(HttpResponse::Ok().json(response))
}
