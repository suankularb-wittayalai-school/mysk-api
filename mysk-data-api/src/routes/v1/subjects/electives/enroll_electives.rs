use actix_web::{post, web, HttpResponse, Responder};

use mysk_lib::models::common::response::ResponseType;
use mysk_lib::models::common::traits::TopLevelQuery;
use mysk_lib::models::elective_subject::request::{
    queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject,
};
use mysk_lib::prelude::*;

use mysk_lib::models::common::requests::RequestType;
use mysk_lib::models::elective_subject::ElectiveSubject;
use uuid::Uuid;

use crate::{middlewares::api_key::HaveApiKey, AppState};

#[post("/{id}/enroll")]
pub async fn enroll_elective_subject(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
    _: HaveApiKey,
    request_query: RequestType<ElectiveSubject, QueryableElectiveSubject, SortableElectiveSubject>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let elective_id = id.into_inner();
    
    Ok(HttpResponse::Ok())
}
