use actix_web::web::{scope, ServiceConfig};

pub(crate) mod students;
pub(crate) mod subjects;
pub(crate) mod teachers;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/students").configure(students::config));
    cfg.service(scope("/teachers").configure(teachers::config));
    cfg.service(scope("/subjects").configure(subjects::config));
}
