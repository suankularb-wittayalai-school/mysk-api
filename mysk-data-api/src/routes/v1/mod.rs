use actix_web::web::{scope, ServiceConfig};

pub mod students;
pub mod subjects;
pub mod teachers;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/students").configure(students::config));
    cfg.service(scope("/teachers").configure(teachers::config));
    cfg.service(scope("/subjects").configure(subjects::config));
}
