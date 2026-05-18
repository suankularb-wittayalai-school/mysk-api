use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};

use mysk_lib::{common::response::ResponseType, models::student::db::DbStudent, prelude::*};
use uuid::Uuid;

use crate::{AppState, extractors::api_key::ApiKeyHeader};

#[get("/{id}/clubs/quota")]
pub async fn query_students_club_quota(
    data: Data<AppState>,
    _: ApiKeyHeader,
    id: Path<Uuid>,
) -> Result<impl Responder> {
    let mut conn = data.db.acquire().await?;
    let student_id = id.into_inner();
    let quota = DbStudent::get_student_club_quota(&mut conn, student_id, None).await?;

    let response = ResponseType::new(quota, None);

    Ok(HttpResponse::Ok().json(response))
}
