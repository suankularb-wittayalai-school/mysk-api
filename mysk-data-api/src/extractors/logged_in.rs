use crate::AppState;
#[cfg(debug_assertions)]
use actix_web::http::Method;
use actix_web::{FromRequest, HttpRequest, dev::Payload, http::header, web::Data};
use futures::{
    FutureExt as _,
    future::{self, LocalBoxFuture},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
#[cfg(debug_assertions)]
use mysk_lib::{auth::oauth::TokenClaims, models::user::User, prelude::*};
use serde::Serialize;
#[cfg(debug_assertions)]
use uuid::Uuid;

#[cfg(debug_assertions)]
#[derive(Clone, Copy, Debug)]
enum ImpersonationHeader {
    Absent,
    Invalid,
    Target(Uuid),
}

/// Extractor to allow only clients that are logged in.
#[derive(Serialize)]
pub struct LoggedIn(pub User);

impl FromRequest for LoggedIn {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self>>;

    #[allow(clippy::too_many_lines)]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let app_state = req
            .app_data::<Data<AppState>>()
            .expect("Irrecoverable error, AppState is None")
            .clone();
        let source = req.path().to_string();
        let Some(authorization_header) = req.headers().get(header::AUTHORIZATION) else {
            return future::err(Error::MissingToken(
                "Missing authorization token".to_string(),
                source,
            ))
            .boxed();
        };
        let Ok(token_parts) = authorization_header.to_str() else {
            return future::err(Error::InvalidAuthorizationScheme(
                "Internal authorization scheme".to_string(),
                source,
            ))
            .boxed();
        };
        let token_parts: Vec<&str> = token_parts.split(' ').collect();

        let Some(scheme) = token_parts.first() else {
            return future::err(Error::InvalidAuthorizationScheme(
                "Invalid authorization scheme".to_string(),
                source,
            ))
            .boxed();
        };
        if *scheme != "Bearer" {
            return future::err(Error::InvalidAuthorizationScheme(
                "Invalid authorization scheme".to_string(),
                source,
            ))
            .boxed();
        }

        let Some(token) = token_parts.get(1) else {
            return future::err(Error::MissingToken(
                "Missing authorization token".to_string(),
                source,
            ))
            .boxed();
        };
        let Ok(decoded_token) = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(app_state.env.token_secret.as_bytes()),
            &Validation::default(),
        ) else {
            return future::err(Error::InvalidToken(
                "Invalid authorization token".to_string(),
                source,
            ))
            .boxed();
        };

        #[cfg(debug_assertions)]
        let impersonation = {
            match req.headers().get("x-impersonate-user") {
                None => ImpersonationHeader::Absent,
                Some(_) if req.method() != Method::GET => ImpersonationHeader::Invalid,
                Some(value) => value
                    .to_str()
                    .ok()
                    .and_then(|raw| Uuid::parse_str(raw.trim()).ok())
                    .map_or(ImpersonationHeader::Invalid, ImpersonationHeader::Target),
            }
        };

        async move {
            let mut conn = app_state.db.acquire().await?;
            let actor = User::get_by_id(
                &mut conn,
                decoded_token.claims.sub,
                decoded_token.claims.mta,
            )
            .await?;

            #[cfg(debug_assertions)]
            {
                match impersonation {
                    _ if !actor.is_admin => {
                        return Err(Error::InvalidPermission(
                            "Impersonation requires admin privileges".to_string(),
                            source,
                        ));
                    }

                    ImpersonationHeader::Absent => {}
                    ImpersonationHeader::Invalid => {
                        return Err(Error::InvalidRequest(
                            "Invalid X-Impersonate-User value".to_string(),
                            source,
                        ));
                    }
                    ImpersonationHeader::Target(target_id) => {
                        if target_id == actor.id {
                            return Err(Error::InvalidPermission(
                                "Cannot impersonate yourself".to_string(),
                                source,
                            ));
                        }

                        let target = User::get_by_id(&mut conn, target_id, None).await?;

                        if target.is_admin {
                            return Err(Error::InvalidPermission(
                                "Cannot impersonate another administrator".to_string(),
                                source,
                            ));
                        }

                        tracing::warn!(
                            actor_id = %actor.id,
                            target_id = %target.id,
                            target_role = %target.role,
                            source = %source,
                            "Admin impersonation active"
                        );

                        return Ok(LoggedIn(target));
                    }
                }
            }

            Ok(LoggedIn(actor))
        }
        .boxed()
    }
}
