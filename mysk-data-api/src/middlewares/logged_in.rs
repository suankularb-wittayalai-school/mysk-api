use actix_web::dev::Payload;
use actix_web::{http, web, FromRequest, HttpRequest};
use futures::Future as FutureTrait;
use jsonwebtoken::{decode, DecodingKey, Validation};
use mysk_lib::error::Error;
use mysk_lib::models::auth::oauth::TokenClaims;
use mysk_lib::models::common::traits::GetById;
use mysk_lib::models::user::User;
use serde::Serialize;
use std::pin::Pin;
use uuid::Uuid;

use crate::AppState;

#[derive(Serialize)]
pub struct LoggedIn(pub User);

impl FromRequest for LoggedIn {
    type Error = Error;
    type Future = Pin<Box<dyn FutureTrait<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let app_state = match req.app_data::<web::Data<AppState>>() {
            Some(state) => state,
            None => {
                return Box::pin(async {
                    Err(Error::InternalSeverError(
                        "App state not found".to_string(),
                        "LoggedIn Middleware".to_string(),
                    ))
                })
            }
        };

        let pool = app_state.db.clone();
        let jwt_secret = app_state.env.jwt_secret.clone();

        let auth_header = req.headers().get(http::header::AUTHORIZATION);

        let token = match auth_header {
            Some(token) => match token.to_str() {
                Ok(token) => token,
                Err(_) => {
                    return Box::pin(async {
                        // return 401 unauthorized if the token is not a string as ResponseType
                        Err(Error::InvalidToken(
                            "Invalid token".to_string(),
                            "LoggedIn Middleware".to_string(),
                        ))
                    });
                }
            },
            None => {
                return Box::pin(async {
                    Err(Error::MissingToken(
                        "Missing token".to_string(),
                        "LoggedIn Middleware".to_string(),
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
                        "LoggedIn Middleware".to_string(),
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
                        "LoggedIn Middleware".to_string(),
                    ))
                })
            }
        };

        Box::pin(async move {
            let user = User::get_by_id(&pool, user_id).await;

            match user {
                Ok(user) => Ok(LoggedIn(user)),
                Err(_) => Err(Error::EntityNotFound(
                    "User not found".to_string(),
                    "LoggedIn Middleware".to_string(),
                )),
            }
        })
    }
}
