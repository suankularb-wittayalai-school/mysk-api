use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod doc;
pub(crate) mod health;

use doc::ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check);
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
