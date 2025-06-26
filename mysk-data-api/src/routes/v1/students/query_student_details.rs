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
        requests::RequestType,
        response::ResponseType,
    },
    models::student::Student,
    permissions::Authorizer,
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_student_details(
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
    let authorizer = Authorizer::new(&mut conn, &user, format!("/students/{student_id}")).await?;

    let student = Student::get_by_id(
        pool,
        student_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(student, None);

    Ok(HttpResponse::Ok().json(response))
}
