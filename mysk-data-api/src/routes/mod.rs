use actix_web::web::{scope, ServiceConfig};

pub mod auth;
pub mod health;
pub mod v1;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(auth::config));
    cfg.service(health::health_check);
    cfg.service(scope("/v1").configure(v1::config));
}
