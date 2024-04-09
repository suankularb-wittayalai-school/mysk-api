use actix_web::web;

pub(crate) mod students;
pub(crate) mod subjects;
pub(crate) mod teachers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/students").configure(students::config));

    cfg.service(web::scope("/teachers").configure(teachers::config));

    cfg.service(web::scope("/subjects").configure(subjects::config));
}
