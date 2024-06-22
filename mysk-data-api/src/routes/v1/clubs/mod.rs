use actix_web::web::{scope, ServiceConfig};

pub mod add_club_members;
pub mod create_club_contacts;
pub mod get_club_statistics;
pub mod join_clubs;
pub mod query_club_details;
pub mod query_clubs;
pub mod requests;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/requests").configure(requests::config));
    cfg.service(get_club_statistics::get_club_statistics);
    cfg.service(add_club_members::add_club_members);
    cfg.service(join_clubs::join_clubs);
    cfg.service(create_club_contacts::create_club_contacts);
    cfg.service(query_club_details::query_club_details);
    cfg.service(query_clubs::query_clubs);
}
