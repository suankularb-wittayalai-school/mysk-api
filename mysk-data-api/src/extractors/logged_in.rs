use super::ExtractorFuture;
use crate::AppState;
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
        let Some(app_state) = req.app_data::<Data<AppState>>() else {
            return Box::pin(future::err(Error::InternalSeverError(
                "App state not found".to_string(),
                "extractors::LoggedIn".to_string(),
            )));
        };

        let pool = app_state.db.clone();
        let jwt_secret = app_state.env.token_secret.clone();

        let Some(token) = req.headers().get(header::AUTHORIZATION) else {
            return Box::pin(future::err(Error::MissingToken(
                "Missing token".to_string(),
                "extractors::LoggedIn".to_string(),
            )));
        };
        let Ok(token) = token.to_str() else {
            return Box::pin(future::err(Error::InvalidToken(
                "Invalid token".to_string(),
                "extractors::LoggedIn".to_string(),
            )));
        };

        let Ok(claims) = decode::<TokenClaims>(
            token.trim_start_matches("Bearer "),
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ) else {
            return Box::pin(future::err(Error::InvalidToken(
                "Invalid token".to_string(),
                "extractors::LoggedIn".to_string(),
            )));
        };

        let Ok(user_id) = Uuid::parse_str(&claims.claims.sub) else {
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
