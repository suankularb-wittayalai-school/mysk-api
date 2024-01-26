pub(crate) mod create_api_key;
pub(crate) mod google_login;
pub(crate) mod user;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(google_login::google_oauth_handler);
    cfg.service(user::get_user);
    cfg.service(create_api_key::create_api_key);
}
