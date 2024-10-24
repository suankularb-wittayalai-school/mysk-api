use actix_web::web::{scope, ServiceConfig};

pub mod enroll_electives;
pub mod get_previously_enrolled;
pub mod in_enrollment_period;
pub mod modify_electives;
pub mod query_elective_details;
pub mod query_electives;
pub mod trade_offers;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/trade-offers").configure(trade_offers::config));
    cfg.service(get_previously_enrolled::get_previously_enrolled);
    cfg.service(in_enrollment_period::in_enrollment_period);
    cfg.service(enroll_electives::enroll_elective_subject);
    cfg.service(modify_electives::modify_elective_subject);
    cfg.service(query_elective_details::query_elective_details);
    cfg.service(query_electives::query_elective_subject);
}
