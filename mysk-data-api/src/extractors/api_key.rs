use super::ExtractorFuture;
use crate::AppState;
use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use futures::future;
use mysk_lib::{
    auth::key::{ApiKey, PrefixedApiKey},
    prelude::*,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::query_as;

/// Extractor to allow only clients with a valid API key.
#[derive(Serialize)]
pub struct ApiKeyHeader(ApiKey);

impl FromRequest for ApiKeyHeader {
    type Error = Error;
    type Future = ExtractorFuture<Self>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let app_state = match req.app_data::<Data<AppState>>() {
            Some(state) => state,
            None => {
                return Box::pin(future::err(Error::InternalSeverError(
                    "App state not found".to_string(),
                    "HaveApiKey Middleware".to_string(),
                )))
            }
        };

        let pool = app_state.db.clone();
        let x_api_key_header = req.headers().get("X-API-KEY");

        let token = match x_api_key_header {
            Some(token) => match token.to_str() {
                Ok(token) => token,
                Err(_) => {
                    return Box::pin(future::err(Error::InvalidApiKey(
                        "Invalid API Key".to_string(),
                        "HaveApiKey Middleware".to_string(),
                    )));
                }
            },
            None => {
                return Box::pin(async {
                    Err(Error::MissingApiKey(
                        "Missing API Key".to_string(),
                        "HaveApiKey Middleware".to_string(),
                    ))
                })
            }
        };

        let token = PrefixedApiKey::from(token.to_string());

        let mut hasher = Sha256::new();
        hasher.update(token.get_long_token().as_bytes());
        let hash = bs58::encode(hasher.finalize()).into_string();

        // let hash = match hash {
        //     Ok(hash) => hash,
        //     Err(_) => {
        //         return Box::pin(async {
        //             Err(Error::InvalidApiKey(
        //                 "Invalid API Key".to_string(),
        //                 "HaveApiKey Middleware".to_string(),
        //             ))
        //         })
        //     }
        // };

        Box::pin(async move {
            let api_key = match query_as!(
                ApiKey,
                r#"
                SELECT * FROM user_api_keys
                WHERE long_token_hash = $1 AND short_token = $2
                AND (expire_at > NOW() OR expire_at IS NULL)
                "#,
                hash,
                token.get_short_token(),
            )
            .fetch_one(&pool)
            .await
            {
                Ok(api_key) => api_key,
                Err(err) => match err {
                    sqlx::Error::RowNotFound => {
                        return Err(Error::InvalidApiKey(
                            "Invalid API Key".to_string(),
                            "HaveApiKey Middleware".to_string(),
                        ))
                    }
                    _ => {
                        return Err(Error::InternalSeverError(
                            "Internal server error".to_string(),
                            "HaveApiKey Middleware".to_string(),
                        ))
                    }
                },
            };

            Ok(ApiKeyHeader(api_key))
        })
    }
}
