use actix_web::web::{scope, ServiceConfig};

pub mod create_club_contacts;
pub mod query_club_details;
pub mod query_clubs;
pub mod requests;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/requests").configure(requests::config));
    cfg.service(create_club_contacts::create_club_contacts);
    cfg.service(query_club_details::query_club_details);
    cfg.service(query_clubs::query_clubs);
}
