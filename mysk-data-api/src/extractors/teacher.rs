use crate::extractors::logged_in::LoggedIn;
use actix_web::{FromRequest, HttpRequest, dev::Payload};
use futures::{FutureExt as _, future::LocalBoxFuture};
use mysk_lib::{
    models::{
        enums::UserRole,
        user::{User, UserMeta},
    },
    prelude::*,
};
use serde::Serialize;
use uuid::Uuid;

/// Extractor to allow only clients that are logged in as teachers.
#[derive(Serialize)]
pub struct LoggedInTeacher(pub Uuid);

impl FromRequest for LoggedInTeacher {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let source = req.path().to_string();
        let user = LoggedIn::from_request(req, payload);

        async move {
            let user = user.await?.0;
            match user {
                User {
                    role: UserRole::Teacher,
                    meta: Some(UserMeta::Teacher { teacher_id }),
                    ..
                } => Ok(LoggedInTeacher(teacher_id)),
                _ => Err(Error::InvalidPermission(
                    "User is not a teacher".to_string(),
                    source,
                )),
            }
        }
        .boxed_local()
    }
}
