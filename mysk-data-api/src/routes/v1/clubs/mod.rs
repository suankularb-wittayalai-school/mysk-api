use actix_web::web::{ServiceConfig, scope};

pub mod add_club_member;
pub mod create_club_contact;
pub mod get_club_statistics;
pub mod join_club;
pub mod query_club_details;
pub mod query_clubs;
pub mod requests;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/requests").configure(requests::config))
        .service(get_club_statistics::get_club_statistics)
        .service(add_club_member::add_club_member)
        .service(join_club::join_club)
        .service(create_club_contact::create_club_contact)
        .service(query_club_details::query_club_details)
        .service(query_clubs::query_clubs);
}
