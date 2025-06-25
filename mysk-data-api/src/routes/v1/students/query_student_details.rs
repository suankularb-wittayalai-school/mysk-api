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
    models::{student::Student, traits::TopLevelGetById as _},
    permissions::Authorizer,
    prelude::*,
    query::QueryablePlaceholder,
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
    }: RequestType<(), QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = id.into_inner();
    let authorizer =
        Authorizer::new(pool, &user, format!("/students/{student_id}")).await?;

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
