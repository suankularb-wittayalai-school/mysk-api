use crate::AppState;
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use chrono::{prelude::*, Duration};
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
struct GoogleTokenResponse {
    access_token: String,
    expires_in: i64,
    token_type: String,
    scope: String,
    id_token: String,
}

#[allow(clippy::cast_possible_wrap)]
#[post("/oauth/google")]
async fn google_oauth_handler(
    data: Data<AppState>,
    query: Json<OAuthRequest>,
) -> Result<impl Responder> {
    let id_token: String = query.credential.clone();

    if id_token.is_empty() {
        return Err(Error::InvalidToken(
            "Invalid token".to_string(),
            "/auth/oauth/google".to_string(),
        ));
    }

    // decode id_token to get google user info with jwt and get access_token and verify it with google secret
    let google_id_data = match verify_id_token(&id_token, &data.env).await {
        Ok(data) => data,
        Err(err) => {
            return Err(Error::InvalidToken(
                err.to_string(),
                "/auth/oauth/google".to_string(),
            ));
        }
    };

    let google_user = GoogleUserResult::from_token_payload(google_id_data);

    let user = User::get_by_email(&data.db, &google_user.email).await;

    // let user_id = match user {
    //     Some(user) => Ok(user.id),
    //     None => {
    //         return Err(Error::EntityNotFound(
    //             "User not found".to_string(),
    //             "/auth/oauth/google".to_string(),
    //         ))
    //     }
    // };

    let user_id = match user {
        Ok(Some(user)) => user.id,
        Ok(None) => {
            return Err(Error::EntityNotFound(
                "User not found".to_string(),
                "/auth/oauth/google".to_string(),
            ))
        }
        Err(err) => {
            return Err(Error::InternalSeverError(
                err.to_string(),
                "/auth/oauth/google".to_string(),
            ))
        }
    };

    let jwt_secret = data.env.token_secret.clone();
    let now = Utc::now();
    let iat = usize::try_from(now.timestamp()).unwrap();
    let exp = usize::try_from((now + Duration::minutes(data.env.token_max_age as i64)).timestamp())
        .unwrap();
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
                .max_age(ActixWebDuration::days(30))
                .same_site(actix_web::cookie::SameSite::Strict)
                .finish();

            let response: ResponseType<GoogleTokenResponse> = ResponseType::new(
                GoogleTokenResponse {
                    access_token: token,
                    expires_in: data.env.token_max_age as i64 * 60,
                    token_type: "Bearer".to_owned(),
                    scope: "email profile".to_owned(),
                    id_token,
                },
                None,
            );

            Ok(HttpResponse::Ok().cookie(cookie).json(response))
        }
        Err(err) => Err(Error::InternalSeverError(
            err.to_string(),
            "/auth/oauth/google".to_string(),
        )),
    }
}
