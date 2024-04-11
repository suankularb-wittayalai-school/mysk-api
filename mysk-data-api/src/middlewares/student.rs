use crate::{middlewares::logged_in::LoggedIn, AppState};
use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use futures::Future as FutureTrait;
use mysk_lib::{
    models::{student::Student, user::enums::user_role::UserRole},
    prelude::*,
};
use serde::Serialize;
use std::pin::Pin;
use uuid::Uuid;

#[derive(Serialize)]
pub struct StudentOnly(pub Uuid);

impl FromRequest for StudentOnly {
    type Error = Error;
    type Future = Pin<Box<dyn FutureTrait<Output = Result<Self>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let app_state = match req.app_data::<Data<AppState>>() {
            Some(state) => state,
            None => {
                return Box::pin(async {
                    Err(Error::InternalSeverError(
                        "App state not found".to_string(),
                        "LoggedIn Middleware".to_string(),
                    ))
                })
            }
        };

        let pool = app_state.db.clone();

        let req = req.clone();
        let user = LoggedIn::from_request(&req, payload);

        Box::pin(async move {
            let user = user.await?.0;

            // Check if the user is a student
            match user.role {
                UserRole::Student => {}
                _ => {
                    return Err(Error::InvalidPermission(
                        "User is not a student".to_string(),
                        req.path().to_string(),
                    ));
                }
            }

            // Get the student
            let student_id = Student::get_student_from_user_id(&pool, user.id)
                .await?
                .ok_or(Error::InternalSeverError(
                    "Student not found".to_string(),
                    "Student Middleware".to_string(),
                ))?;

            Ok(StudentOnly(student_id))
        })
    }
}
