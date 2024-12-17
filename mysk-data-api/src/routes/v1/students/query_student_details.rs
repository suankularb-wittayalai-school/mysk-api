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
        requests::{QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::{student::Student, traits::TopLevelGetById as _},
    permissions,
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_student_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    id: Path<Uuid>,
    request_query: RequestType<Student, QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let student_id = id.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/students/{student_id}")).await?;

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
