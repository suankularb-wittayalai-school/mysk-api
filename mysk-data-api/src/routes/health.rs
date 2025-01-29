use crate::AppState;
use actix_web::{get, web::Data, HttpResponse, Responder};
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
    let pool = &data.db;

    let start = time::Instant::now();
    let database_connection = query("SELECT 1").execute(pool).await.is_ok();
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
