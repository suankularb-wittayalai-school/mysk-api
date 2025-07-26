use actix_web::web::{ServiceConfig, scope};

pub mod query_practice_period;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/requests").configure(requests::config))
        .service(query_practice_period::query_practice_period);
}
