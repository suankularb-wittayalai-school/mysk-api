use actix_web::web::ServiceConfig;

pub mod check_practice_attendance;
pub mod query_practice_period_details;
pub mod query_practice_periods;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(check_practice_attendance::check_practice_attendance)
        .service(query_practice_periods::query_practice_periods)
        .service(query_practice_period_details::query_practice_period_details);
}
