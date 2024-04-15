use super::ExtractorFuture;
use crate::{extractors::logged_in::LoggedIn, AppState};
use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use futures::future;
use mysk_lib::{
    models::{student::Student, user::enums::user_role::UserRole},
    prelude::*,
};
use serde::Serialize;
use uuid::Uuid;

/// Extractor to allow only clients that are logged in as students.
#[derive(Serialize)]
pub struct LoggedInStudent(pub Uuid);

impl FromRequest for LoggedInStudent {
    type Error = Error;
    type Future = ExtractorFuture<Self>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let Some(app_state) = req.app_data::<Data<AppState>>() else {
            return Box::pin(future::err(Error::InternalSeverError(
                "App state not found".to_string(),
                "extractors::StudentOnly".to_string(),
            )));
        };

        let pool = app_state.db.clone();

        let req = req.clone();
        let user = LoggedIn::from_request(&req, payload);

        Box::pin(async move {
            let user = user.await?.0;
            if !matches!(user.role, UserRole::Student) {
                return Err(Error::InvalidPermission(
                    "User is not a student".to_string(),
                    req.path().to_string(),
                ));
            }

            let student_id = Student::get_student_from_user_id(&pool, user.id)
                .await?
                .ok_or(Error::EntityNotFound(
                    "Student not found".to_string(),
                    "extractors::StudentOnly".to_string(),
                ))?;

            Ok(LoggedInStudent(student_id))
        })
    }
}
