use actix_web::web;


pub mod enroll_electives;
pub mod query_elective_details;
pub mod query_electives;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(query_electives::query_elective_subject);
    cfg.service(enroll_electives::enroll_elective_subject);
    cfg.service(query_elective_details::query_elective_details);
}
