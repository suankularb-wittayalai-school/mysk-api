use crate::{extractors::logged_in::LoggedIn, extractors::ExtractorFuture, AppState};
use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use futures::FutureExt as _;
use mysk_lib::{
    models::{enums::UserRole, teacher::db::DbTeacher},
    prelude::*,
};
use serde::Serialize;
use uuid::Uuid;

/// Extractor to allow only clients that are logged in as teachers.
#[derive(Serialize)]
pub struct LoggedInTeacher(pub Uuid);

impl FromRequest for LoggedInTeacher {
    type Error = Error;
    type Future = ExtractorFuture<Self>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let app_state = req
            .app_data::<Data<AppState>>()
            .expect("Irrecoverable error, AppState is None");
        let pool = app_state.db.clone();
        let source = req.path().to_string();
        let user = LoggedIn::from_request(req, payload);

        async move {
            let user = user.await?.0;
            match user.role {
                UserRole::Teacher | UserRole::Management => (),
                _ => {
                    return Err(Error::InvalidPermission(
                        "User is not a teacher".to_string(),
                        source,
                    ));
                }
            }

            let teacher_id = DbTeacher::get_teacher_from_user_id(&pool, user.id)
                .await?
                .ok_or(Error::InvalidPermission(
                    "User is not a teacher".to_string(),
                    source,
                ))?;

            Ok(LoggedInTeacher(teacher_id))
        }
        .boxed_local()
    }
}
