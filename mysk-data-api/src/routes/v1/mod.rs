use actix_web::web::{scope, ServiceConfig};

pub mod clubs;
pub mod students;
pub mod subjects;
pub mod teachers;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/clubs").configure(clubs::config));
    cfg.service(scope("/students").configure(students::config));
    cfg.service(scope("/teachers").configure(teachers::config));
    cfg.service(scope("/subjects").configure(subjects::config));
}
