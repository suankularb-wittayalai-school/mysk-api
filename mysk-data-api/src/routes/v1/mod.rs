use actix_web::web::{ServiceConfig, scope};

pub mod certificates;
pub mod clubs;
pub mod contacts;
pub mod students;
pub mod subjects;
pub mod teachers;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/certificates").configure(certificates::config))
        .service(scope("/clubs").configure(clubs::config))
        .service(scope("/contacts").configure(contacts::config))
        .service(scope("/students").configure(students::config))
        .service(scope("/teachers").configure(teachers::config))
        .service(scope("/subjects").configure(subjects::config));
}
