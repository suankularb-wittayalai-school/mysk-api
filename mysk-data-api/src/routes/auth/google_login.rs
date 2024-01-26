use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    post, web, HttpResponse, Responder,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use mysk_lib::{
    error::Error,
    models::{
        auth::oauth::{verify_id_token, GoogleUserResult, TokenClaims},
        common::response::ResponseType,
        user::User,
    },
};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct OAuthRequest {
    pub credential: String,
}
#[derive(Debug, Serialize)]
struct GoogleTokenResponse {
    access_token: String,
    expires_in: i64,
    token_type: String,
    scope: String,
    id_token: String,
}

#[post("/oauth/google")]
async fn google_oauth_handler(
    data: web::Data<AppState>,
    query: web::Json<OAuthRequest>,
) -> Result<impl Responder, actix_web::Error> {
    // dbg!(query);

    let id_token: String = query.credential.to_owned();

    // dbg!(id_token.as_str());

    if id_token.is_empty() {
        return Ok(HttpResponse::Unauthorized().json(Error::InvalidToken(
            "Invalid token".to_string(),
            "/auth/oauth/google".to_string(),
        )));
    }

    // decode id_token to get google user info with jwt and get access_token and verify it with google secret
    let google_id_data = match verify_id_token(&id_token, &data.env).await {
        Ok(data) => data,
        Err(err) => {
            return Ok(HttpResponse::Unauthorized().json(Error::InvalidToken(
                err.to_string(),
                "/auth/oauth/google".to_string(),
            )));
        }
    };

    let google_user = GoogleUserResult::from_token_payload(google_id_data);

    // dbg!(&google_user);

    let user = User::get_by_email(&data.db, &google_user.email).await;

    let user_id = match user {
        Some(user) => user.id,
        None => {
            return Ok(HttpResponse::NotFound().json(Error::EntityNotFound(
                "User not found".to_string(),
                "/auth/oauth/google".to_string(),
            )))
        }
    };

    let jwt_secret = data.env.jwt_secret.to_owned();
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(data.env.jwt_max_age)).timestamp() as usize;
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
            let cookie = Cookie::build("token", token.to_owned())
                .secure(true)
                .http_only(true)
                .max_age(ActixWebDuration::days(30))
                .same_site(actix_web::cookie::SameSite::Strict)
                .finish();

            let response: ResponseType<GoogleTokenResponse> = ResponseType::new(
                GoogleTokenResponse {
                    access_token: token,
                    expires_in: data.env.jwt_max_age * 60,
                    token_type: "Bearer".to_owned(),
                    scope: "email profile".to_owned(),
                    id_token,
                },
                None,
            );

            Ok(HttpResponse::Ok().cookie(cookie).json(response))
        }
        Err(err) => Ok(
            HttpResponse::InternalServerError().json(Error::InternalSeverError(
                err.to_string(),
                "/auth/oauth/google".to_string(),
            )),
        ),
    }
}
