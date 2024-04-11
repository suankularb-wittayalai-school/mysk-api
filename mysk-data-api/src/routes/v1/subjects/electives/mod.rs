use actix_web::web::ServiceConfig;

pub mod enroll_electives;
pub mod modify_electives;
pub mod query_elective_details;
pub mod query_electives;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(enroll_electives::enroll_elective_subject);
    cfg.service(modify_electives::modify_elective_subject);
    cfg.service(query_elective_details::query_elective_details);
    cfg.service(query_electives::query_elective_subject);
}
