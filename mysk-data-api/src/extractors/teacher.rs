use crate::{extractors::logged_in::LoggedIn, extractors::ExtractorFuture, AppState};
use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
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
        let req = req.clone();
        let user = LoggedIn::from_request(&req, payload);

        Box::pin(async move {
            let user = user.await?.0;
            if !matches!(user.role, UserRole::Teacher) {
                return Err(Error::InvalidPermission(
                    "User is not a teacher".to_string(),
                    req.path().to_string(),
                ));
            }

            let teacher_id = DbTeacher::get_teacher_from_user_id(&pool, user.id)
                .await?
                .ok_or(Error::EntityNotFound(
                    "Teacher not found".to_string(),
                    "extractors::LoggedInTeacher".to_string(),
                ))?;

            Ok(LoggedInTeacher(teacher_id))
        })
    }
}
