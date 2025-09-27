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
    models::{
        cheer_practice_attendance::{CheerPracticeAttendance, db::DbCheerPracticeAttendance},
        cheer_practice_period::db::DbCheerPracticePeriod,
        classroom::db::DbClassroom,
    },
    permissions::Authorizer,
    prelude::*,
};

use uuid::Uuid;

#[get("/{period_id}/{classroom_id}")]
pub async fn query_classroom_cheer_practice_attendance(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    ids: Path<(Uuid, Uuid)>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = pool.acquire().await?;
    let practice_period_id = ids.0;
    let classroom_id = ids.1;
    let authorizer = Authorizer::new(
        &mut conn,
        &user,
        format!("/attendance/cheer/{practice_period_id}/{classroom_id}"),
    )
    .await?;

    if !DbCheerPracticePeriod::get_classroom_ids(&mut conn, practice_period_id)
        .await?
        .contains(&classroom_id)
    {
        return Err(Error::InvalidRequest(
            "Requested classroom is not a part of the current cheer practice period".to_string(),
            format!("/attendance/cheer/{practice_period_id}/{classroom_id}"),
        ));
    }

    let classroom_members = DbClassroom::get_classroom_students(&mut conn, classroom_id).await?;
    let attendance_ids =
        DbCheerPracticeAttendance::get_by_student_ids(&mut conn, classroom_members).await?;

    let attendances = CheerPracticeAttendance::get_by_ids(
        pool,
        &attendance_ids,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;

    let response = ResponseType::new(attendances, None);

    Ok(HttpResponse::Ok().json(response))
}
