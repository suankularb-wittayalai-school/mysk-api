use actix_web::web::{ServiceConfig, scope};

pub mod periods;
pub mod query_classroom_cheer_practice_attendance;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/periods").configure(periods::config))
        .service(
            query_classroom_cheer_practice_attendance::query_classroom_cheer_practice_attendance,
        );
}
