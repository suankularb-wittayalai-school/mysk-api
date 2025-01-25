use actix_web::web::{scope, ServiceConfig};

pub mod attendance;
pub mod electives;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/attendance").configure(attendance::config));
    cfg.service(scope("/electives").configure(electives::config));
    cfg.service(scope("/attendance").configure(attendance::config));
}
