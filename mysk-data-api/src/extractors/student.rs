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

/// Extractor to allow only clients that are logged in as students.
#[derive(Serialize)]
pub struct LoggedInStudent(pub Uuid);

impl FromRequest for LoggedInStudent {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let source = req.path().to_string();
        let user = LoggedIn::from_request(req, payload);

        async move {
            let user = user.await?.0;
            match user {
                User {
                    role: UserRole::Student,
                    meta: Some(UserMeta::Student { student_id }),
                    ..
                } => Ok(LoggedInStudent(student_id)),
                _ => Err(Error::InvalidPermission(
                    "User is not a student".to_string(),
                    source,
                )),
            }
        }
        .boxed_local()
    }
}
