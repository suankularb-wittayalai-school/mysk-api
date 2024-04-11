use actix_web::web::{scope, ServiceConfig};

pub mod electives;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/electives").configure(electives::config));
}
