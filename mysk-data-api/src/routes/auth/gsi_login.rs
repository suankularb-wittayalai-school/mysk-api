use crate::AppState;
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use mysk_lib::{
    auth::oauth::{verify_id_token, GoogleUserResult, TokenClaims},
    common::response::ResponseType,
    error::Error,
    models::user::User,
    prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OAuthRequest {
    pub credential: String,
}
#[derive(Debug, Serialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub scope: String,
    pub id_token: String,
}

#[allow(clippy::cast_possible_wrap)]
#[post("/oauth/gsi")]
async fn gsi_handler(data: Data<AppState>, query: Json<OAuthRequest>) -> Result<impl Responder> {
    let id_token: String = query.credential.clone();

    if id_token.is_empty() {
        return Err(Error::InvalidToken(
            "Invalid token".to_string(),
            "/auth/oauth/gsi".to_string(),
        ));
    }

    // decode id_token to get google user info with jwt and get access_token and verify it with
    // google secret
    let google_id_data = match verify_id_token(&id_token, &data.env).await {
        Ok(data) => data,
        Err(err) => {
            return Err(Error::InvalidToken(
                err.to_string(),
                "/auth/oauth/gsi".to_string(),
            ));
        }
    };

    let google_user = GoogleUserResult::from_token_payload(google_id_data);
    let user_id = User::get_by_email(&data.db, &google_user.email).await?.id;

    let jwt_secret = data.env.token_secret.clone();
    let now = Utc::now();
    let iat = usize::try_from(now.timestamp())
        .expect("Irrecoverable error, i64 is out of range for usize");
    let exp = usize::try_from((now + Duration::minutes(data.env.token_max_age as i64)).timestamp())
        .expect("Irrecoverable error, i64 is out of range for usize");
    let claims = TokenClaims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    );

    match token {
        Ok(token) => {
            let cookie = Cookie::build("token", token.clone())
                .secure(true)
                .http_only(true)
                .max_age(ActixWebDuration::minutes(data.env.token_max_age as i64))
                .same_site(actix_web::cookie::SameSite::Strict)
                .finish();

            let response: ResponseType<GoogleTokenResponse> = ResponseType::new(
                GoogleTokenResponse {
                    access_token: token,
                    expires_in: data.env.token_max_age * 60,
                    token_type: "Bearer".to_string(),
                    scope: "email profile".to_string(),
                    id_token,
                },
                None,
            );

            Ok(HttpResponse::Ok().cookie(cookie).json(response))
        }
        Err(err) => Err(Error::InternalSeverError(
            err.to_string(),
            "/auth/oauth/gsi".to_string(),
        )),
    }
}
