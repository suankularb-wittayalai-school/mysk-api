use actix_web::web::{ServiceConfig, scope};

pub mod cheer;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/cheer").configure(cheer::config));
}
