use crate::{AppState, routes::auth::gsi_login::GoogleTokenResponse};
use actix_web::{
    HttpResponse, Responder,
    cookie::{Cookie, time::Duration as ActixWebDuration},
    get,
    web::{Data, Query, Redirect},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use mysk_lib::{
    auth::oauth::{
        GoogleUserResult, TokenClaims, exchange_oauth_code, generate_oauth_init_url,
        verify_id_token,
    },
    common::response::ResponseType,
    models::user::{User, UserMeta},
    prelude::*,
};
use serde::Deserialize;

#[get("/oauth/init")]
pub async fn oauth_initiator(data: Data<AppState>) -> Result<impl Responder> {
    let (redirect_url, state) = generate_oauth_init_url(
        &data.env.google_oauth_client_id,
        format!("{}/auth/oauth/google", &data.env.root_uri).as_str(),
    )?;

    {
        let mut guard = data.oauth_states.lock();
        let oauth_states = &mut *guard;
        oauth_states.insert(state);
    }

    Ok(Redirect::to(redirect_url))
}

#[derive(Deserialize)]
struct GoogleOAuthCodeRequest {
    code: String,
    state: String,
}

#[allow(clippy::cast_possible_wrap)]
#[get("/oauth/google")]
pub async fn google_oauth_handler(
    data: Data<AppState>,
    Query(GoogleOAuthCodeRequest {
        ref code,
        ref state,
    }): Query<GoogleOAuthCodeRequest>,
) -> Result<impl Responder> {
    {
        let mut guard = data.oauth_states.lock();
        let oauth_states = &mut *guard;

        if !oauth_states.contains(state) {
            return Err(Error::InvalidToken(
                "OAuth state mismatch".to_string(),
                "/auth/oauth/google".to_string(),
            ));
        }

        oauth_states.remove(state);
    }

    let id_token = exchange_oauth_code(
        code,
        &data.env.google_oauth_client_id,
        &data.env.google_oauth_client_secret,
        format!("{}/auth/oauth/google", &data.env.root_uri).as_str(),
    )
    .await?;

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
    let user = User::get_by_email(&mut *(data.db.acquire().await?), &google_user.email).await?;

    let now = Utc::now();
    let iat = usize::try_from(now.timestamp())
        .expect("Irrecoverable error, i64 is out of range for usize");
    let exp = usize::try_from((now + Duration::minutes(data.env.token_max_age as i64)).timestamp())
        .expect("Irrecoverable error, i64 is out of range for usize");
    let claims = TokenClaims {
        sub: user.id,
        mta: match user.meta {
            Some(UserMeta::Student { student_id: id } | UserMeta::Teacher { teacher_id: id }) => {
                Some(id)
            }
            _ => None,
        },
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
            scope: "openid email profile",
            id_token,
        },
        None,
    );

    Ok(HttpResponse::Ok().cookie(cookie).json(response))
}
