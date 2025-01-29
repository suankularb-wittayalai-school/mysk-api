use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, teacher::LoggedInTeacher},
    AppState,
};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    models::{
        enums::UserRole, online_teaching_reports::db::DbOnlineTeachingReports, traits::GetById as _,
    },
    prelude::*,
};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION},
    Client,
};
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;
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
    let pool = &data.db;
    let report_id = report_id.into_inner();

    let class_report = DbOnlineTeachingReports::get_by_id(pool, report_id)
        .await
        .map_err(|e| match e {
            SqlxError::RowNotFound => Error::EntityNotFound(
                "Class report not found".to_string(),
                format!("/subjects/attendance/{report_id}"),
            ),
            _ => e.into(),
        })?;

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
        .await?;

    if !image_request.status().is_success() {
        return Err(Error::InternalSeverError(
            "Internal server error".to_string(),
            format!("/subjects/attendance/image/{report_id}"),
        ));
    }
    let signed_url = image_request.json::<SignedUrl>().await.unwrap().signed_url;

    Ok(HttpResponse::Ok().json(ReportImageResponse {
        image_url: format!("{}/storage/v1{}", data.env.supabase_uri, signed_url),
    }))
}
