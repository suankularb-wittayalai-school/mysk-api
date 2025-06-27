use actix_web::web::{ServiceConfig, scope};

pub mod enroll_electives;
pub mod get_previously_enrolled;
pub mod in_enrollment_period;
pub mod modify_electives;
pub mod query_elective_details;
pub mod query_electives;
pub mod trade_offers;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/trade-offers").configure(trade_offers::config))
        .service(get_previously_enrolled::get_previously_enrolled)
        .service(in_enrollment_period::in_enrollment_period)
        .service(enroll_electives::enroll_elective_subject)
        .service(modify_electives::modify_elective_subject)
        .service(query_elective_details::query_elective_details)
        .service(query_electives::query_elective_subject);
}
