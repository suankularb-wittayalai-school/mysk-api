use actix_web::{post, web, HttpResponse, Responder};
use mysk_lib::models::common::traits::TopLevelGetById;
use mysk_lib::models::user;
use mysk_lib::models::user::enums::user_role::UserRole;
use sqlx::types::Uuid;

use mysk_lib::prelude::*;

use mysk_lib::models::common::requests::{
    FetchLevel, QueryablePlaceholder, RequestType, SortablePlaceholder,
};
use mysk_lib::models::elective_subject::ElectiveSubject;

use crate::middlewares::logged_in::LoggedIn;
use crate::middlewares::student::StudentOnly;
use crate::{middlewares::api_key::HaveApiKey, AppState};

#[post("/{id}/enroll")]
pub async fn enroll_elective_subject(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
    student_id: StudentOnly,
    _: HaveApiKey,
    request_query: RequestType<ElectiveSubject, QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = student_id.0;
    let id = id.into_inner();

    // let elective_enroll = ElectiveSubject::enroll();

    // let user = user.0;
    dbg!(&student_id);

    let elective = ElectiveSubject::get_by_id(pool, id, Some(&FetchLevel::Detailed), None).await?;

    // Check if the elective the student is trying to enroll in is available
    match elective {
        ElectiveSubject::Detailed(elective, _) => {
            if elective.class_size == elective.cap_size {
                return Err(Error::InvalidPermission(
                    "The elective is already full".to_string(),
                    format!("/subjects/electives/{id}/enroll"),
                ));
            }
        }
        _ => unreachable!("ElectiveSubject::get_by_id should always return a Compact variant"),
    }

    // check if the student is in a class available for the elective

    // check if the student has already enrolled in the elective before

    Ok(HttpResponse::Created())
}
