use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    AppState,
};
use actix_web::{
    put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
        string::FlexibleMultiLangString,
    },
    models::{
        person::Person,
        teacher::{db::DbTeacher, Teacher},
        traits::TopLevelGetById,
    },
    permissions::{self, ActionType},
    prelude::*,
};
use mysk_lib_macros::traits::db::GetById as _;
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct UpdateTeacherRequest {
    prefix: FlexibleMultiLangString,
    first_name: FlexibleMultiLangString,
    last_name: FlexibleMultiLangString,
    middle_name: FlexibleMultiLangString,
    nickname: FlexibleMultiLangString,
    // subject_group: Option<i64>,
    // advisor_at: Option<String>,
    // birthdate: Option<NaiveDate>,
    // allergies: Vec<String>,
    // shirt_size: Option<ShirtSize>,
    // pants_size: Option<String>,
    // TODO: Teacher-related fields
}

#[put("/{id}")]
pub async fn modify_teacher(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    teacher_id: Path<Uuid>,
    request_body: Json<
        RequestType<UpdateTeacherRequest, QueryablePlaceholder, SortablePlaceholder>,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let teacher_id = teacher_id.into_inner();
    let Some(update_data) = &request_body.data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/teachers/{teacher_id}"),
        ));
    };
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("teachers/{teacher_id}")).await?;

    // Authorize update action
    let db_teacher = DbTeacher::get_by_id(pool, teacher_id).await?;

    authorizer
        .authorize_teacher(&db_teacher, pool, ActionType::Update)
        .await?;
    let person_id = match &db_teacher.person_id {
        Some(id) => id,
        None => {
            return Err(Error::EntityNotFound(
                "person_id not found for teacher".to_string(),
                format!("/teachers/{teacher_id}"),
            ));
        }
    };

    let person = Person::get_by_id(pool, *person_id).await?;

    let _ = query!(
        r#"
            UPDATE people
            SET
                prefix_th = COALESCE($1, prefix_th),
                prefix_en = COALESCE($2, prefix_en),
                first_name_th = COALESCE($3, first_name_th),
                first_name_en = COALESCE($4, first_name_en),
                last_name_th = COALESCE($5, last_name_th),
                last_name_en = COALESCE($6, last_name_en),
                middle_name_th = COALESCE($7, middle_name_th),
                middle_name_en = COALESCE($8, middle_name_en),
                nickname_th = COALESCE($9, nickname_th),
                nickname_en = COALESCE($10, nickname_en)
            WHERE
              id = $11
        "#,
        update_data.prefix.th.as_ref(),
        update_data.prefix.en.as_ref(),
        update_data.first_name.th.as_ref(),
        update_data.first_name.en.as_ref(),
        update_data.last_name.th.as_ref(),
        update_data.last_name.en.as_ref(),
        update_data.middle_name.th.as_ref(),
        update_data.middle_name.en.as_ref(),
        update_data.nickname.th.as_ref(),
        update_data.nickname.en.as_ref(),
        person.id
    )
    .execute(pool)
    .await?;

    let updated_person = Person::get_by_id(pool, person.id).await?;

    let response = ResponseType::new(updated_person, None);

    Ok(HttpResponse::Ok().json(response))
}
