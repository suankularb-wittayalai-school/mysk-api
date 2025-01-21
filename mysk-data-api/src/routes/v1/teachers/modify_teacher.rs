use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    AppState,
};
use actix_web::{
    put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use chrono::NaiveDate;
use mysk_lib::{
    common::{
        requests::{QueryParam, QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
        string::FlexibleMultiLangString,
    },
    helpers::date::get_current_academic_year,
    models::{
        enums::ShirtSize,
        teacher::{db::DbTeacher, Teacher},
        traits::{GetById as _, TopLevelGetById as _},
    },
    permissions::{self, ActionType},
    prelude::*,
    query::set_clause::SqlSetClause,
};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct UpdateTeacherRequest {
    person: Option<UpdatePersonInfo>,
    teacher: Option<UpdateTeacherInfo>,
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
#[derive(Debug, Deserialize)]
struct UpdateTeacherInfo {
    subject_group_id: Option<i64>,
    advisor_at: Option<i64>,
}

#[allow(clippy::too_many_lines)]
#[put("/{id}")]
pub async fn modify_teacher(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    teacher_id: Path<Uuid>,
    Json(request_body): Json<
        RequestType<UpdateTeacherRequest, QueryablePlaceholder, SortablePlaceholder>,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let teacher_id = teacher_id.into_inner();
    let Some(update_data) = request_body.data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/teachers/{teacher_id}"),
        ));
    };
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("teachers/{teacher_id}")).await?;

    let db_teacher = DbTeacher::get_by_id(pool, teacher_id).await?;
    let person_id = db_teacher
        .person_id
        .expect("Every teacher should have a person_id");

    authorizer
        .authorize_teacher(&db_teacher, pool, ActionType::Update)
        .await?;

    // NOTE: Teacher-related updates
    if let Some(tu) = update_data.teacher {
        let mut teacher_transaction = pool.begin().await?;
        let current_academic_year = get_current_academic_year(None);

        // Update subject group
        if let Some(subject_group_id) = tu.subject_group_id {
            let new_subject_group = query!(
                "SELECT id FROM subject_groups WHERE id = $1",
                subject_group_id
            )
            .fetch_one(pool)
            .await?;

            let current_subject_group = db_teacher.subject_group_id;

            if current_subject_group != new_subject_group.id {
                query!(
                    "UPDATE teachers SET subject_group_id = $1 WHERE id = $2",
                    new_subject_group.id,
                    teacher_id,
                )
                .execute(&mut *teacher_transaction)
                .await?;
            }
        };

        // Update/insert advisory classroom
        if let Some(class_advisor_at) = tu.advisor_at {
            let new_classroom = query!(
                "SELECT id FROM classrooms WHERE number = $1 AND year = $2 FOR UPDATE",
                class_advisor_at,
                current_academic_year,
            )
            .fetch_one(pool)
            .await?;

            let existing_advisor_at =
                DbTeacher::get_teacher_advisor_at(pool, teacher_id, Some(current_academic_year))
                    .await?;

            if existing_advisor_at != Some(new_classroom.id) {
                match existing_advisor_at {
                    Some(existing_classroom) => {
                        // Change advisory classroom to new classroom
                        query!(
                            "
                            UPDATE classroom_advisors SET classroom_id = $1
                            WHERE teacher_id = $2 AND classroom_id = $3
                            ",
                            new_classroom.id,
                            teacher_id,
                            existing_classroom,
                        )
                        .execute(&mut *teacher_transaction)
                        .await?;
                    }
                    None => {
                        // If the teacher isn't an advisor, add them to a classroom
                        query!(
                            "
                            INSERT INTO classroom_advisors (classroom_id, teacher_id)
                            VALUES ($1, $2)
                            ",
                            new_classroom.id,
                            teacher_id,
                        )
                        .execute(&mut *teacher_transaction)
                        .await?;
                    }
                }
            }
        }
        teacher_transaction.commit().await?;
    }

    // NOTE: Person-related updates
    if let Some(pu) = update_data.person {
        let mut person_transaction = pool.begin().await?;

        // Update allergies on a separate table `person_allergies`
        if let Some(allergies) = pu.allergies {
            query!(
                "DELETE FROM person_allergies WHERE person_id = $1",
                person_id,
            )
            .execute(&mut *person_transaction)
            .await?;

            // See also: https://github.com/launchbadge/sqlx/blob/main/FAQ.md#how-can-i-bind-an-array-to-a-values-clause-how-can-i-do-bulk-inserts
            query!(
                "
                INSERT INTO person_allergies (person_id, allergy_name)
                SELECT $1, * FROM UNNEST($2::text[])
                ",
                person_id,
                &allergies[..],
            )
            .execute(&mut *person_transaction)
            .await?;
        };

        // TODO: Refactor `SqlSetClause` API
        let mut qb = SqlSetClause::new()
            .push_multilang_update_field("prefix", pu.prefix)
            .push_multilang_update_field("first_name", pu.first_name)
            .push_multilang_update_field("last_name", pu.last_name)
            .push_multilang_update_field("middle_name", pu.middle_name)
            .push_multilang_update_field("nickname", pu.nickname)
            .push_update_field("birthdate", pu.birthdate, QueryParam::NaiveDate)
            .push_update_field("shirt_size", pu.shirt_size, QueryParam::ShirtSize)
            .push_update_field("pants_size", pu.pants_size, QueryParam::String)
            .into_query_builder("UPDATE people");

        qb.push(" WHERE id = ")
            .push_bind(person_id)
            .build()
            .execute(&mut *person_transaction)
            .await?;

        person_transaction.commit().await?;
    };

    let teacher = Teacher::get_by_id(
        pool,
        teacher_id,
        fetch_level,
        descendant_fetch_level,
        &*authorizer,
    )
    .await?;
    let response = ResponseType::new(teacher, None);

    Ok(HttpResponse::Ok().json(response))
}
