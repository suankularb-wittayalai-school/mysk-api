use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{
    HttpResponse, Responder, put,
    web::{Data, Json, Path},
};
use chrono::NaiveDate;
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
        string::FlexibleMultiLangString,
    },
    models::{
        enums::ShirtSize,
        student::{Student, db::DbStudent},
        traits::{GetById as _, TopLevelGetById as _},
    },
    permissions::{ActionType, Authorizable as _, Authorizer},
    prelude::*,
    query::{QueryParam, QueryablePlaceholder, SqlSetClause},
};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct UpdateStudentRequest {
    person: Option<UpdatePersonInfo>,
}

#[derive(Debug, Deserialize)]
struct UpdatePersonInfo {
    prefix: Option<FlexibleMultiLangString>,
    first_name: Option<FlexibleMultiLangString>,
    last_name: Option<FlexibleMultiLangString>,
    middle_name: Option<FlexibleMultiLangString>,
    nickname: Option<FlexibleMultiLangString>,
    birthdate: Option<NaiveDate>,
    allergies: Option<Vec<String>>,
    shirt_size: Option<ShirtSize>,
    pants_size: Option<String>,
}

#[allow(clippy::too_many_lines)]
#[put("/{id}")]
pub async fn modify_student(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    student_id: Path<Uuid>,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<UpdateStudentRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let student_id = student_id.into_inner();
    let Some(update_data) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/students/{student_id}"),
        ));
    };
    let authorizer = Authorizer::new(&mut conn, &user, format!("students/{student_id}")).await?;

    let db_student = DbStudent::get_by_id(&mut conn, student_id).await?;
    let person_id = db_student.person_id;

    authorizer
        .authorize_student(&db_student, &mut conn, ActionType::Update)
        .await?;

    // NOTE: Person-related updates
    if let Some(pu) = update_data.person {
        let mut person_transaction = pool.begin().await?;

        if let Some(allergies) = pu.allergies {
            query!(
                "DELETE FROM person_allergies WHERE person_id = $1",
                person_id,
            )
            .execute(&mut *person_transaction)
            .await?;

            query!(
                "\
                INSERT INTO person_allergies (person_id, allergy_name)\
                SELECT $1, * FROM UNNEST($2::text[])\
                ",
                person_id,
                &allergies[..],
            )
            .execute(&mut *person_transaction)
            .await?;
        }

        let mut qb = SqlSetClause::new();
        qb.push_multilang_update_field("prefix", pu.prefix)
            .push_multilang_update_field("first_name", pu.first_name)
            .push_multilang_update_field("last_name", pu.last_name)
            .push_multilang_update_field("middle_name", pu.middle_name)
            .push_multilang_update_field("nickname", pu.nickname)
            .push_update_field("birthdate", pu.birthdate, QueryParam::NaiveDate)
            .push_update_field("shirt_size", pu.shirt_size, QueryParam::ShirtSize)
            .push_update_field("pants_size", pu.pants_size, QueryParam::String);

        let mut qb = qb.into_query_builder("UPDATE people");
        qb.push(" WHERE id = ")
            .push_bind(person_id)
            .build()
            .execute(&mut *person_transaction)
            .await?;

        person_transaction.commit().await?;
    }

    let student = Student::get_by_id(
        pool,
        student_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(student, None);

    Ok(HttpResponse::Ok().json(response))
}
