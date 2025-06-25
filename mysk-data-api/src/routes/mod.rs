use actix_web::web::{ServiceConfig, scope, to};

pub mod auth;
pub mod health;
pub mod not_found;
pub mod v1;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(auth::config))
        .service(scope("/v1").configure(v1::config))
        .service(health::health_check)
        .default_service(to(not_found::not_found));
}
