use actix_web::{post, web, HttpResponse, Responder};
use mysk_lib::models::classroom::Classroom;
use mysk_lib::models::common::response::ResponseType;
use mysk_lib::models::common::traits::TopLevelGetById;
use mysk_lib::models::student::Student;
use sqlx::query;
use sqlx::types::Uuid;

use mysk_lib::prelude::*;

use mysk_lib::helpers::date::{get_current_academic_year, get_current_semester};
use mysk_lib::models::common::requests::{
    FetchLevel, QueryablePlaceholder, RequestType, SortablePlaceholder,
};
use mysk_lib::models::elective_subject::ElectiveSubject;

use crate::middlewares::student::StudentOnly;
use crate::{middlewares::api_key::HaveApiKey, AppState};

#[post("/{id}/enroll")]
pub async fn enroll_elective_subject(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
    student_id: StudentOnly,
    _: HaveApiKey,
    request_query: web::Json<
        RequestType<ElectiveSubject, QueryablePlaceholder, SortablePlaceholder>,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = student_id.0;
    let id = id.into_inner();
    let request_query = request_query.into_inner();

    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    // let elective_enroll = ElectiveSubject::enroll();

    // let user = user.0;
    dbg!(&student_id);

    let elective = ElectiveSubject::get_by_id(pool, id, Some(&FetchLevel::Detailed), None).await?;

    // Check if the elective the student is trying to enroll in is available
    let elective = match elective {
        ElectiveSubject::Detailed(elective, _) => {
            if elective.class_size == elective.cap_size {
                return Err(Error::InvalidPermission(
                    "The elective is already full".to_string(),
                    format!("/subjects/electives/{id}/enroll"),
                ));
            }
            elective
        }
        _ => unreachable!("ElectiveSubject::get_by_id should always return a Detailed variant"),
    };

    // check if the student is in a class available for the elective
    let student = Student::get_by_id(pool, student_id, Some(&FetchLevel::Default), None).await?;
    match student {
        Student::Default(student, _) => match student.classroom {
            None => {
                return Err(Error::InvalidPermission(
                    "Student is not in a class".to_string(),
                    format!("/subjects/electives/{id}/enroll"),
                ));
            }
            Some(classroom) => {
                if !elective
                    .applicable_classrooms
                    .iter()
                    .any(|c| match (c, &classroom) {
                        (Classroom::IdOnly(c, _), Classroom::IdOnly(classroom, _)) => {
                            c.id == classroom.id
                        }
                        _ => false,
                    })
                {
                    return Err(Error::InvalidPermission(
                        "Student is not in a class available for the elective".to_string(),
                        format!("/subjects/electives/{id}/enroll"),
                    ));
                }
            }
        },
        _ => unreachable!("Student::get_by_id should always return a Default variant"),
    }

    // check if the student has already enrolled in the elective before
    let enroll_count = query!(
        r#"
        SELECT COUNT(*) FROM student_elective_subjects
        WHERE student_id = $1 AND elective_subject_id = $2
        "#,
        student_id,
        id
    )
    .fetch_one(pool)
    .await?;

    let enroll_count: i64 = enroll_count.count.unwrap_or(0);
    if enroll_count > 0 {
        return Err(Error::InvalidPermission(
            "Student has already enrolled in the elective".to_string(),
            format!("/subjects/electives/{id}/enroll"),
        ));
    }

    // enroll the student in the elective
    let _ = query!(
        r#"
        INSERT INTO student_elective_subjects (student_id, elective_subject_id, year, semester) VALUES ($1, $2, $3, $4)
        "#,
        student_id,
        id,
        get_current_academic_year(None),
        get_current_semester(None)
    ).execute(pool).await?;

    let elective =
        ElectiveSubject::get_by_id(pool, id, fetch_level, descendant_fetch_level).await?;
    let response = ResponseType::new(elective, None);

    Ok(HttpResponse::Created().json(response))
}
