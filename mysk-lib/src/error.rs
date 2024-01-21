use actix_web::HttpResponse;
use serde::Serialize;
use uuid::Uuid;

use crate::models::common::response::{ErrorResponseType, ErrorType};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    EntityNotFound(String, String), // String is the detail
}

impl From<Error> for HttpResponse {
    fn from(error: Error) -> Self {
        match error {
            Error::EntityNotFound(detail, source) => {
                HttpResponse::NotFound().json(ErrorResponseType::new(
                    ErrorType {
                        id: Uuid::new_v4().to_string(),
                        code: 404,
                        error_type: "entity_not_found".to_string(),
                        detail,
                        source,
                    },
                    None,
                ))
            }
        }
    }
}
