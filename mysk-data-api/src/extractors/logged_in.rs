use crate::{extractors::ExtractorFuture, AppState};
use actix_web::{dev::Payload, http::header, web::Data, FromRequest, HttpRequest};
use futures::future;
use jsonwebtoken::{decode, DecodingKey, Validation};
use mysk_lib::{auth::oauth::TokenClaims, models::user::User, prelude::*};
use serde::Serialize;
use uuid::Uuid;

/// Extractor to allow only clients that are logged in.
#[derive(Serialize)]
pub struct LoggedIn(pub User);

impl FromRequest for LoggedIn {
    type Error = Error;
    type Future = ExtractorFuture<Self>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let app_state = req
            .app_data::<Data<AppState>>()
            .expect("Irrecoverable error, AppState is None");
        let pool = app_state.db.clone();
        let jwt_secret = app_state.env.token_secret.clone();
        let Some(authorization_header) = req.headers().get(header::AUTHORIZATION) else {
            return Box::pin(future::err(Error::MissingToken(
                "Missing authorization token".to_string(),
                "extractors::LoggedIn".to_string(),
            )));
        };
        let Ok(token_parts) = authorization_header.to_str() else {
            return Box::pin(future::err(Error::InternalSeverError(
                "Internal server error".to_string(),
                "extractors::LoggedIn".to_string(),
            )));
        };
        let token_parts: Vec<&str> = token_parts.split(' ').collect();

        let Some(scheme) = token_parts.first() else {
            return Box::pin(future::err(Error::InvalidAuthorizationScheme(
                "Invalid authorization scheme".to_string(),
                "extractors::LoggedIn".to_string(),
            )));
        };
        if *scheme != "Bearer" {
            return Box::pin(future::err(Error::InvalidAuthorizationScheme(
                "Invalid authorization scheme".to_string(),
                "extractors::LoggedIn".to_string(),
            )));
        }

        let Some(token) = token_parts.get(1) else {
            return Box::pin(future::err(Error::MissingToken(
                "Missing authorization token".to_string(),
                "extractors::LoggedIn".to_string(),
            )));
        };
        let Ok(decoded_token) = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ) else {
            return Box::pin(future::err(Error::InvalidToken(
                "Invalid authorization token".to_string(),
                "extractors::LoggedIn".to_string(),
            )));
        };

        let Ok(user_id) = Uuid::parse_str(&decoded_token.claims.sub) else {
            return Box::pin(future::err(Error::EntityNotFound(
                "User not found".to_string(),
                "extractors::LoggedIn".to_string(),
            )));
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
