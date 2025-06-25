use crate::AppState;
use actix_web::{HttpResponse, Responder, get, web::Data};
use chrono::{SecondsFormat, Utc};
use mysk_lib::{common::response::ResponseType, prelude::*};
use serde::Serialize;
use sqlx::query;
use std::time;

#[derive(Serialize)]
struct HealthCheckResponse {
    server_time: String,
    database_connection: bool,
    database_response_time: u128,
}

#[get("/health-check")]
pub async fn health_check(data: Data<AppState>) -> Result<impl Responder> {
    let mut conn = data.db.acquire().await?;

    let start = time::Instant::now();
    let database_connection = query("SELECT 1").execute(&mut *conn).await.is_ok();
    let database_response_time = start.elapsed().as_millis();

    let response = ResponseType::new(
        HealthCheckResponse {
            server_time: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            database_connection,
            database_response_time,
        },
        None,
    );

    Ok(HttpResponse::Ok().json(response))
}
