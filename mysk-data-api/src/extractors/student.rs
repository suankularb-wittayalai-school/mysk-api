use crate::{extractors::logged_in::LoggedIn, extractors::ExtractorFuture, AppState};
use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use mysk_lib::{
    models::{enums::UserRole, student::Student},
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
        let app_state = req.app_data::<Data<AppState>>().unwrap();
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
                    "extractors::LoggedInStudent".to_string(),
                ))?;

            Ok(LoggedInStudent(student_id))
        })
    }
}
