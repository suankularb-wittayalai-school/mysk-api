use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, teacher::LoggedInTeacher},
};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use mysk_lib::{
    models::{
        enums::UserRole, online_teaching_reports::db::DbOnlineTeachingReports, traits::GetById as _,
    },
    prelude::*,
};
use reqwest::{
    Client,
    header::{AUTHORIZATION, HeaderValue},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExpiresIn {
    expires_in: i64,
}

#[derive(Debug, Deserialize)]
struct SignedUrl {
    #[serde(rename = "signedURL")]
    signed_url: String,
}

#[derive(Debug, Serialize)]
struct ReportImageResponse {
    image_url: String,
}

#[get("/{id}")]
pub async fn get_report_image(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    LoggedInTeacher(teacher_id): LoggedInTeacher,
    report_id: Path<Uuid>,
) -> Result<impl Responder> {
    let mut conn = data.db.acquire().await?;
    let report_id = report_id.into_inner();

    let class_report = DbOnlineTeachingReports::get_by_id(&mut conn, report_id).await?;

    // Check if the report is owned by the teacher
    if !matches!(user.role, UserRole::Management) && class_report.teacher_id != teacher_id {
        return Err(Error::EntityNotFound(
            "Class report not found".to_string(),
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

    let supabase_authorization = format!("Bearer {}", data.env.supabase_secret_key);
    let image_request = Client::new()
        .post(format!(
            "{}/storage/v1/object/sign/online_teaching_reports/{}.{}",
            data.env.supabase_uri,
            report_id,
            class_report.image_ext.unwrap(),
        ))
        .json(&ExpiresIn { expires_in: 3600 }) // Expires in 1 hour
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&supabase_authorization).unwrap(),
        )
        .send()
        .await?
        .error_for_status()?;

    let signed_url = image_request.json::<SignedUrl>().await?.signed_url;

    Ok(HttpResponse::Ok().json(ReportImageResponse {
        image_url: format!("{}/storage/v1{}", data.env.supabase_uri, signed_url),
    }))
}
