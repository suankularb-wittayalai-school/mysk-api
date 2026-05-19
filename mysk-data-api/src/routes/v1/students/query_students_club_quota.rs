use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};

use mysk_lib::{
    common::{requests::FetchLevel, response::ResponseType},
    models::student::{Student, db::DbStudent},
    permissions::Authorizer,
    prelude::*,
};
use uuid::Uuid;

use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};

#[get("/{id}/clubs/quota")]
pub async fn query_students_club_quota(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    id: Path<Uuid>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let student_id = id.into_inner();
    let authorizer = Authorizer::new(&user, format!("/v1/students/{student_id}/clubs/quota"));

    // Checks if the student exists
    let Student::IdOnly(student, _) = Student::get_by_id(
        pool,
        student_id,
        FetchLevel::IdOnly,
        FetchLevel::IdOnly,
        &authorizer,
    )
    .await?
    else {
        unreachable!("Student should always be an IdOnly variant")
    };

    let quota = DbStudent::get_student_club_quota(&mut conn, student.id, None).await?;

    let response = ResponseType::new(quota, None);

    Ok(HttpResponse::Ok().json(response))
}
