use actix_web::web::{ServiceConfig, scope};

pub mod in_jaturamitr_period;
pub mod periods;
pub mod query_cheer_practice_attendances;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/periods").configure(periods::config))
        .service(query_cheer_practice_attendances::query_cheer_practice_attendances)
        .service(in_jaturamitr_period::in_jaturamitr_period);
}
