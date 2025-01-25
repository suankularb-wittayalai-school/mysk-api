use actix_web::web::{scope, ServiceConfig};

pub mod electives;
pub mod reports;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/electives").configure(electives::config));
    cfg.service(scope("/reports").configure(reports::config));
}
