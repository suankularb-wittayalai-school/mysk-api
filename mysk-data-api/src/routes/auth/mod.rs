use actix_web::web;

mod create_api_key;
mod google_oauth_login;
mod gsi_login;
mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_api_key::create_api_key);
    cfg.service(google_oauth_login::oauth_initiator);
    cfg.service(google_oauth_login::google_oauth_handler);
    cfg.service(gsi_login::gsi_handler);
    cfg.service(user::get_user);
}
