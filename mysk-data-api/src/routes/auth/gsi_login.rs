use crate::AppState;
use actix_web::{
    HttpResponse, Responder,
    cookie::{Cookie, time::Duration as ActixWebDuration},
    post,
    web::{Data, Json},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use mysk_lib::{
    auth::oauth::{GoogleUserResult, TokenClaims, verify_id_token},
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
pub struct GoogleTokenResponse<'a> {
    pub access_token: &'a str,
    pub expires_in: u64,
    pub token_type: &'static str,
    pub scope: &'static str,
    pub id_token: String,
}

#[allow(clippy::cast_possible_wrap)]
#[post("/oauth/gsi")]
pub async fn gsi_handler(
    data: Data<AppState>,
    Json(query): Json<OAuthRequest>,
) -> Result<impl Responder> {
    let id_token = query.credential;
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
    let user_id = User::get_by_email(&mut *(data.db.acquire().await?), &google_user.email)
        .await?
        .id;

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

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.token_secret.as_bytes()),
    )?;

    let cookie = Cookie::build("token", &token)
        .secure(true)
        .http_only(true)
        .max_age(ActixWebDuration::minutes(data.env.token_max_age as i64))
        .same_site(actix_web::cookie::SameSite::Strict)
        .finish();

    let response: ResponseType<GoogleTokenResponse> = ResponseType::new(
        GoogleTokenResponse {
            access_token: &token,
            expires_in: data.env.token_max_age * 60,
            token_type: "Bearer",
            scope: "email profile",
            id_token,
        },
        None,
    );

    Ok(HttpResponse::Ok().cookie(cookie).json(response))
}
