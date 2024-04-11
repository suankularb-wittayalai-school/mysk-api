use actix_web::web;

mod create_api_key;
mod google_login;
mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(google_login::google_oauth_handler);
    cfg.service(user::get_user);
    cfg.service(create_api_key::create_api_key);
}
