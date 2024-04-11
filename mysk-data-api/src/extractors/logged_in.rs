use super::ExtractorFuture;
use crate::AppState;
use actix_web::{dev::Payload, http::header, web::Data, FromRequest, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use mysk_lib::{auth::oauth::TokenClaims, models::user::User, prelude::*};
use mysk_lib_macros::traits::db::GetById;
use serde::Serialize;
use uuid::Uuid;

/// Extractor to allow only clients that are logged in.
#[derive(Serialize)]
pub struct LoggedIn(pub User);

impl FromRequest for LoggedIn {
    type Error = Error;
    type Future = ExtractorFuture<Self>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let app_state = match req.app_data::<Data<AppState>>() {
            Some(state) => state,
            None => {
                return Box::pin(async {
                    Err(Error::InternalSeverError(
                        "App state not found".to_string(),
                        "extractors::LoggedIn".to_string(),
                    ))
                })
            }
        };

        let pool = app_state.db.clone();
        let jwt_secret = app_state.env.token_secret.clone();

        let auth_header = req.headers().get(header::AUTHORIZATION);

        let token = match auth_header {
            Some(token) => match token.to_str() {
                Ok(token) => token,
                Err(_) => {
                    return Box::pin(async {
                        // return 401 unauthorized if the token is not a string as ResponseType
                        Err(Error::InvalidToken(
                            "Invalid token".to_string(),
                            "extractors::LoggedIn".to_string(),
                        ))
                    });
                }
            },
            None => {
                return Box::pin(async {
                    Err(Error::MissingToken(
                        "Missing token".to_string(),
                        "extractors::LoggedIn".to_string(),
                    ))
                })
            }
        };

        let token = token.trim_start_matches("Bearer ");

        let claims = match decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(claims) => claims,
            Err(_) => {
                return Box::pin(async {
                    Err(Error::InvalidToken(
                        "Invalid token".to_string(),
                        "extractors::LoggedIn".to_string(),
                    ))
                })
            }
        };

        let user_id = match Uuid::parse_str(&claims.claims.sub) {
            Ok(user_id) => user_id,
            Err(_) => {
                return Box::pin(async {
                    Err(Error::EntityNotFound(
                        "User not found".to_string(),
                        "extractors::LoggedIn".to_string(),
                    ))
                })
            }
        };

        Box::pin(async move {
            match User::get_by_id(&pool, user_id).await {
                Ok(user) => Ok(LoggedIn(user)),
                Err(_) => Err(Error::EntityNotFound(
                    "User not found".to_string(),
                    "extractors::LoggedIn".to_string(),
                )),
            }
        })
    }
}
