use actix_web::web::{ServiceConfig, scope};

pub mod periods;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/periods").configure(periods::config));
}
