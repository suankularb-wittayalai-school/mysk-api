use actix_web::web;

mod create_api_key;
mod google_oauth_login;
mod gsi_login;
mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_api_key::create_api_key)
        .service(google_oauth_login::google_oauth_handler)
        .service(google_oauth_login::oauth_initiator)
        .service(gsi_login::gsi_handler)
        .service(user::get_user);
}
