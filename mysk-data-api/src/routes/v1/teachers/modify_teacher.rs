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
        requests::{QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
        string::FlexibleMultiLangString,
    },
    helpers::date::get_current_academic_year,
    models::{
        enums::ShirtSize,
        teacher::{db::DbTeacher, Teacher},
        traits::TopLevelGetById,
    },
    permissions::{self, ActionType},
    prelude::*,
};
use mysk_lib_macros::traits::db::GetById as _;
use serde::Deserialize;
use sqlx::{query, Postgres, QueryBuilder};
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

    let db_teacher = DbTeacher::get_by_id(pool, teacher_id).await?;
    let person_id = db_teacher
        .person_id
        .expect("Every teacher should have a person_id");

    authorizer
        .authorize_teacher(&db_teacher, pool, ActionType::Update)
        .await?;

    // NOTE: Teacher-related updates
    if let Some(teacher_update) = &update_data.teacher {
        let mut teacher_transaction = pool.begin().await?;
        let current_academic_year = get_current_academic_year(None);

        // Update subject group
        if let Some(subject_group) = &teacher_update.subject_group_id {
            let new_subject_group =
                query!("SELECT id FROM subject_groups WHERE id = $1", subject_group)
                    .fetch_one(pool)
                    .await?;

            let current_subject_group = db_teacher.subject_group_id;

            if current_subject_group != new_subject_group.id {
                query!(
                    "UPDATE teachers
                    SET subject_group_id = $1
                    WHERE id = $2",
                    new_subject_group.id,
                    teacher_id
                )
                .execute(&mut *teacher_transaction)
                .await?;
            }
        };

        // Update/insert advisory classroom
        if let Some(class_advisor_at) = &teacher_update.advisor_at {
            let new_classroom = query!(
                "SELECT id FROM classrooms WHERE number = $1 AND year = $2 FOR UPDATE",
                class_advisor_at,
                current_academic_year
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
                            "UPDATE classroom_advisors
                            SET classroom_id = $1
                            WHERE teacher_id = $2 AND classroom_id = $3",
                            new_classroom.id,
                            teacher_id,
                            existing_classroom
                        )
                        .execute(&mut *teacher_transaction)
                        .await?;
                    }
                    None => {
                        // If the teacher isn't an advisor, add them to a classroom
                        query!(
                            "INSERT INTO classroom_advisors (classroom_id, teacher_id) 
                            VALUES ($1, $2)",
                            new_classroom.id,
                            teacher_id
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
    if let Some(person_update) = &update_data.person {
        let mut person_transaction = pool.begin().await?;
        let mut updates = Vec::new();
        let mut bindings = Vec::new();
        let mut coalesce_flags = Vec::new();

        // Update allergies on a separate table `person_allergies`, remember to remove debug traces!!!
        if let Some(allergies) = &person_update.allergies {
            query!(
                "DELETE FROM person_allergies WHERE person_id = $1",
                person_id
            )
            .execute(&mut *person_transaction)
            .await?;

            for allergy in allergies {
                query!(
                    "INSERT INTO person_allergies (person_id, allergy_name) VALUES ($1, $2)",
                    person_id,
                    allergy
                )
                .execute(&mut *person_transaction)
                .await?;
            }
        };

        macro_rules! add_update_field {
            // NOTE: This handles `Option<FlexibleMultiLangString>`
            (multilang: $field:ident, $value:expr) => {
                if let Some(new_value) = $value {
                    if let Some(th) = &new_value.th {
                        updates.push(stringify!($field).to_owned() + "_th");
                        bindings.push(th.clone());
                        coalesce_flags.push(true);
                    }

                    if let Some(en) = &new_value.en {
                        updates.push(stringify!($field).to_owned() + "_en");
                        bindings.push(en.clone());
                        coalesce_flags.push(true);
                    }
                };
            };

            ($field:ident, $value:expr) => {
                if let Some(new_value) = $value {
                    updates.push(stringify!($field).to_owned());
                    bindings.push(new_value.to_string()); // FIXME: This casts everything to a string
                    coalesce_flags.push(false);
                }
            };
        }

        add_update_field!(multilang: prefix, &person_update.prefix);
        add_update_field!(multilang: first_name, &person_update.first_name);
        add_update_field!(multilang: last_name, &person_update.last_name);
        add_update_field!(multilang: middle_name, &person_update.middle_name );
        add_update_field!(multilang: nickname, &person_update.nickname);
        add_update_field!(birthdate, &person_update.birthdate);
        add_update_field!(shirt_size, &person_update.shirt_size);
        add_update_field!(pants_size, &person_update.pants_size);

        if !updates.is_empty() {
            let mut query = QueryBuilder::new("UPDATE people SET ");
            for (i, (field, use_coalesce)) in updates.iter().zip(coalesce_flags.iter()).enumerate()
            {
                if i > 0 {
                    query.push(", ");
                }

                if *use_coalesce {
                    query.push(format!("{field} = COALESCE("));
                    query.push_bind(&bindings[i]);
                    query.push(", ");
                    query.push(format!("{field})"));
                } else {
                    query.push(format!("{field} = "));
                    query.push_bind(&bindings[i]);
                }
            }

            query.push(" WHERE id = ");
            query.push_bind(person_id);

            query.push(" RETURNING id");

            // println!("{}", query.into_sql());
            query.build().execute(&mut *person_transaction).await?;
        }

        person_transaction.commit().await?;
    };

    let teacher = Teacher::get_by_id(
        pool,
        teacher_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;

    let response = ResponseType::new(teacher, None);

    Ok(HttpResponse::Ok().json(response))
}
