use crate::{extractors::ExtractorFuture, AppState};
use actix_web::{dev::Payload, http::header, web::Data, FromRequest, HttpRequest};
use futures::{future, FutureExt as _};
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
            return future::err(Error::MissingToken(
                "Missing authorization token".to_string(),
                "extractors::LoggedIn".to_string(),
            ))
            .boxed();
        };
        let Ok(token_parts) = authorization_header.to_str() else {
            return future::err(Error::InvalidAuthorizationScheme(
                "Internal authorization scheme".to_string(),
                "extractors::LoggedIn".to_string(),
            ))
            .boxed();
        };
        let token_parts: Vec<&str> = token_parts.split(' ').collect();

        let Some(scheme) = token_parts.first() else {
            return future::err(Error::InvalidAuthorizationScheme(
                "Invalid authorization scheme".to_string(),
                "extractors::LoggedIn".to_string(),
            ))
            .boxed();
        };
        if *scheme != "Bearer" {
            return future::err(Error::InvalidAuthorizationScheme(
                "Invalid authorization scheme".to_string(),
                "extractors::LoggedIn".to_string(),
            ))
            .boxed();
        }

        let Some(token) = token_parts.get(1) else {
            return future::err(Error::MissingToken(
                "Missing authorization token".to_string(),
                "extractors::LoggedIn".to_string(),
            ))
            .boxed();
        };
        let Ok(decoded_token) = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ) else {
            return future::err(Error::InvalidToken(
                "Invalid authorization token".to_string(),
                "extractors::LoggedIn".to_string(),
            ))
            .boxed();
        };

        let Ok(user_id) = Uuid::parse_str(&decoded_token.claims.sub) else {
            return future::err(Error::EntityNotFound(
                "User not found".to_string(),
                "extractors::LoggedIn".to_string(),
            ))
            .boxed();
        };

        async move { Ok(LoggedIn(User::get_by_id(&pool, user_id).await?)) }.boxed()
    }
}
