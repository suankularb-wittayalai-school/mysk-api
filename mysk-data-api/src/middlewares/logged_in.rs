use actix_web::error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized};
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpRequest};
use futures::Future as FutureTrait;
use jsonwebtoken::{decode, DecodingKey, Validation};
use mysk_lib::error::Error;
use mysk_lib::models::auth::TokenClaims;
use mysk_lib::models::common::traits::GetById;
use mysk_lib::models::user::User;
use std::pin::Pin;
use uuid::Uuid;

use crate::AppState;

pub struct LoggedIn(User);

impl FromRequest for LoggedIn {
    type Error = ActixWebError;
    type Future = Pin<Box<dyn FutureTrait<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let app_state = match req.app_data::<web::Data<AppState>>() {
            Some(state) => state,
            None => {
                return Box::pin(async {
                    Err(ErrorInternalServerError::<Error>(
                        Error::InternalSeverError(
                            "App state not found".to_string(),
                            "LoggedIn Middleware".to_string(),
                        )
                        .into(),
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
                        Err(ErrorUnauthorized::<Error>(
                            Error::InvalidToken(
                                "Invalid token".to_string(),
                                "LoggedIn Middleware".to_string(),
                            )
                            .into(),
                        ))
                    });
                }
            },
            None => {
                return Box::pin(async {
                    Err(ErrorUnauthorized::<Error>(
                        Error::MissingToken(
                            "Missing token".to_string(),
                            "LoggedIn Middleware".to_string(),
                        )
                        .into(),
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
                    Err(ErrorUnauthorized::<Error>(
                        Error::InvalidToken(
                            "Invalid token".to_string(),
                            "LoggedIn Middleware".to_string(),
                        )
                        .into(),
                    ))
                })
            }
        };

        let user_id = match Uuid::parse_str(&claims.claims.sub) {
            Ok(user_id) => user_id,
            Err(_) => {
                return Box::pin(async {
                    Err(ErrorNotFound::<Error>(
                        Error::EntityNotFound(
                            "User not found".to_string(),
                            "LoggedIn Middleware".to_string(),
                        )
                        .into(),
                    ))
                })
            }
        };

        Box::pin(async move {
            let user = User::get_by_id(&pool, user_id).await;

            match user {
                Ok(user) => Ok(LoggedIn(user)),
                Err(_) => Err(ErrorNotFound::<Error>(
                    Error::EntityNotFound(
                        "User not found".to_string(),
                        "LoggedIn Middleware".to_string(),
                    )
                    .into(),
                )),
            }
        })
    }
}
