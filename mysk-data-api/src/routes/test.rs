use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};

use mysk_lib::models::{
    common::{
        response::ResponseType,
        traits::{CombineFromTable, GetById},
    },
    student::{db::DbStudent, Student},
};

use crate::AppState;

#[utoipa::path(path = "/test", tag = "Global")]
#[get("/test")]
pub async fn test(
    data: web::Data<AppState>,
    request: HttpRequest,
) -> Result<impl Responder, Error> {
    let pool: &sqlx::PgPool = &data.db;

    let student_id = request.query_string().split('=').collect::<Vec<&str>>()[1];

    // dbg!(student_id);

    let student = DbStudent::get_by_id(pool, student_id.parse().unwrap())
        .await
        .unwrap();

    let student = Student::combine_from_table(pool, student, None, None)
        .await
        .unwrap();

    let response = ResponseType::new(student, None);

    Ok(HttpResponse::Ok().json(response))
}
