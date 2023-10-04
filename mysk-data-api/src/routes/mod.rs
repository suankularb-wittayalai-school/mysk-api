use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod doc;
pub(crate) mod health;
pub(crate) mod test;

use doc::ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check);
    cfg.service(test::test);
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
