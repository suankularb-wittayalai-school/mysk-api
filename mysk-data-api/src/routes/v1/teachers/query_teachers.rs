use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    AppState,
};
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::{
        requests::RequestType,
        response::{MetadataType, ResponseType},
    },
    models::{
        teacher::{
            request::{queryable::QueryableTeacher, sortable::SortableTeacher},
            Teacher,
        },
        traits::TopLevelQuery as _,
    },
    permissions,
    prelude::*,
};

#[get("")]
pub async fn query_teachers(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    request_query: RequestType<Teacher, QueryableTeacher, SortableTeacher>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let fetch_level = request_query.fetch_level;
    let descendant_fetch_level = request_query.descendant_fetch_level;
    let filter = request_query.filter;
    let sort = request_query.sort;
    let pagination = request_query.pagination;
    let authorizer = permissions::get_authorizer(pool, &user, "/teachers".to_string()).await?;

    let (teacher, pagination) = Teacher::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &*authorizer,
    )
    .await?;
    let response = ResponseType::new(teacher, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
