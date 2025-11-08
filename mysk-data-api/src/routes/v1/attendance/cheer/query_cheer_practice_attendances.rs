use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{
    common::{
        requests::{EmptyRequestData, RequestType},
        response::{MetadataType, ResponseType},
    },
    models::cheer_practice_attendance::{
        CheerPracticeAttendance,
        request::{
            queryable::QueryableCheerPracticeAttendance, sortable::SortableCheerPracticeAttendance,
        },
    },
    permissions::Authorizer,
    prelude::*,
};
use sqlx::query_scalar;
#[get("")]
pub async fn query_cheer_practice_attendances(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    RequestType {
        pagination,
        filter,
        sort,
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<
        EmptyRequestData,
        QueryableCheerPracticeAttendance,
        SortableCheerPracticeAttendance,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let authorizer = Authorizer::new(&mut conn, &user, "/attendance/cheer".to_string()).await?;

    // TODO: Using `practice_period_id` and `classroom_id` filters separately or none at all may
    // overload the FetchVariant's `.from_relation` logic, causing unstable behaviour and/or pool
    // crashes
    if let Some(ref f) = filter
        && let Some(fd) = &f.data
        && let (Some(practice_period_id), Some(classroom_id)) =
            (fd.practice_period_id, fd.classroom_id)
    {
        let is_valid = query_scalar!(
                    "SELECT EXISTS(SELECT 1 FROM cheer_practice_period_classrooms WHERE practice_period_id = $1 AND classroom_id = $2)", 
                    practice_period_id,
                    classroom_id
                )
                .fetch_one(&mut *conn)
                .await?
                .unwrap_or(false);

        if !is_valid {
            return Err(Error::InvalidRequest(
                "Requested classroom is not a part of the current cheer practice period"
                    .to_string(),
                format!("/attendance/cheer/{practice_period_id}/{classroom_id}"),
            ));
        }
    }

    let (attendances, pagination) = CheerPracticeAttendance::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(attendances, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
