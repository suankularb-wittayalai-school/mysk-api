use actix_web::web::ServiceConfig;

pub mod create_offer;
pub mod query_offers;
pub mod update_offer;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_offer::create_trade_offer);
    cfg.service(query_offers::query_trade_offers);
    cfg.service(update_offer::update_trade_offer);
}
