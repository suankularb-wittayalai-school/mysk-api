use actix_web::web::ServiceConfig;

pub mod delete_club_requests;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(delete_club_requests::delete_club_requests);
}
