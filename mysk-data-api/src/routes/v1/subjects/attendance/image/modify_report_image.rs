use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, teacher::LoggedInTeacher},
    AppState,
};
use actix_web::{
    put,
    web::{Bytes, Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::{
        online_teaching_reports::{db::DbOnlineTeachingReports, OnlineTeachingReports},
        traits::{GetById as _, TopLevelGetById as _},
    },
    permissions::Authorizer,
    prelude::*,
    query::QueryablePlaceholder,
};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ModifyReportImageRequest {
    file_extension: String,
}

#[put("/{id}")]
pub async fn modify_report_image(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    LoggedInTeacher(teacher_id): LoggedInTeacher,
    report_id: Path<Uuid>,
    RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<ModifyReportImageRequest, QueryablePlaceholder, SortablePlaceholder>,
    request_body: Bytes,
) -> Result<impl Responder> {
    let pool = &data.db;
    let report_id = report_id.into_inner();
    let Some(update_data) = request_data else {
        return Err(Error::InvalidRequest(
            "Query deserialize error: field `data` can not be empty".to_string(),
            format!("/subjects/attendance/image/{report_id}"),
        ));
    };
    let authorizer = Authorizer::new(
        pool,
        &user,
        format!("/subjects/attendance/image/{report_id}"),
    )
    .await?;

    let class_report = DbOnlineTeachingReports::get_by_id(pool, report_id).await?;

    // Check if the report is owned by the teacher
    if class_report.teacher_id != teacher_id {
        return Err(Error::EntityNotFound(
            "Entity not found".to_string(),
            format!("/subjects/attendance/image/{report_id}"),
        ));
    }

    // Check if the report has an image
    if !class_report.has_image || class_report.image_ext.is_none() {
        return Err(Error::InvalidRequest(
            "Class report is incomplete (missing image)".to_string(),
            format!("/subjects/attendance/image/{report_id}"),
        ));
    }

    // Check if file extension is valid
    match update_data.file_extension.as_str() {
        "png" | "jpg" | "jpeg" | "webp" => (),
        _ => {
            return Err(Error::InvalidRequest(
                "Invalid file extension provided".to_string(),
                format!("/subjects/attendance/image/{report_id}"),
            ));
        }
    }

    let supabase_authorization = format!("Bearer {}", data.env.supabase_secret_key);
    let client = Client::new();

    // Delete the old image
    client
        .delete(format!(
            "{}/storage/v1/object/online_teaching_reports/{}.{}",
            data.env.supabase_uri,
            report_id,
            class_report.image_ext.unwrap(),
        ))
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&supabase_authorization).unwrap(),
        )
        .send()
        .await?
        .error_for_status()?;

    client
        .post(format!(
            "{}/storage/v1/object/online_teaching_reports/{}.{}",
            data.env.supabase_uri, report_id, update_data.file_extension,
        ))
        .body(request_body)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&supabase_authorization).unwrap(),
        )
        .header(
            CONTENT_TYPE,
            HeaderValue::from_str(&format!("image/{}", update_data.file_extension)).unwrap(),
        )
        .send()
        .await?
        .error_for_status()?;

    query!(
        "UPDATE online_teaching_reports SET image_ext = COALESCE($1, image_ext) WHERE id = $2",
        update_data.file_extension,
        report_id,
    )
    .execute(pool)
    .await?;

    let class_report = OnlineTeachingReports::get_by_id(
        pool,
        report_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(class_report, None);

    Ok(HttpResponse::Ok().json(response))
}
