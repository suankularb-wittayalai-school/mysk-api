use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use futures::Future as FutureTrait;
use mysk_lib::error::Error;
use mysk_lib::models::auth::key::{ApiKey, PrefixedApiKey};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::Error as SqlxError;
use std::pin::Pin;

use crate::AppState;

#[derive(Serialize)]
pub struct HaveApiKey(ApiKey);

impl FromRequest for HaveApiKey {
    type Error = Error;
    type Future = Pin<Box<dyn FutureTrait<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let app_state = match req.app_data::<web::Data<AppState>>() {
            Some(state) => state,
            None => {
                return Box::pin(async {
                    Err(Error::InternalSeverError(
                        "App state not found".to_string(),
                        "HaveApiKey Middleware".to_string(),
                    ))
                })
            }
        };

        let pool = app_state.db.clone();

        let x_api_key_header = req.headers().get("X-API-KEY");

        let token = match x_api_key_header {
            Some(token) => match token.to_str() {
                Ok(token) => token,
                Err(_) => {
                    return Box::pin(async {
                        // return 401 unauthorized if the token is not a string as ResponseType
                        Err(Error::InvalidApiKey(
                            "Invalid API Key".to_string(),
                            "HaveApiKey Middleware".to_string(),
                        ))
                    });
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
        let hashed_token = hasher.finalize();
        // let hash = String::from_utf8(hashed_token.to_vec());
        let hash = bs58::encode(hashed_token).into_string();

        // dbg!(&token, &hash);

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
            let api_key = match sqlx::query_as!(
                ApiKey,
                r#"
                SELECT * FROM user_api_keys WHERE long_token_hash = $1 AND short_token = $2 AND (expire_at > NOW() OR expire_at IS NULL)
                "#,
                hash,
                token.get_short_token()
            )
            .fetch_one(&pool)
            .await
            {
                Ok(api_key) => api_key,
                Err(err) => match err {
                    SqlxError::RowNotFound => {
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

            Ok(HaveApiKey(api_key))
        })
    }
}
