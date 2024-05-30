use actix_web::web::{scope, ServiceConfig};

pub mod query_club_details;
pub mod requests;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/requests").configure(requests::config));
    cfg.service(query_club_details::query_club_details);
}
