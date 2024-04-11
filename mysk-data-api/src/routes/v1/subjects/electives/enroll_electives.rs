use crate::{
    middlewares::{api_key::HaveApiKey, student::StudentOnly},
    AppState,
};
use actix_web::{
    post,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    helpers::date::{get_current_academic_year, get_current_semester},
    models::{
        classroom::Classroom,
        common::{
            requests::{FetchLevel, QueryablePlaceholder, RequestType, SortablePlaceholder},
            response::ResponseType,
            traits::TopLevelGetById as _,
        },
        elective_subject::ElectiveSubject,
        student::Student,
    },
    prelude::*,
};
use sqlx::query;
use uuid::Uuid;

#[post("/{id}/enroll")]
pub async fn enroll_elective_subject(
    data: Data<AppState>,
    id: Path<Uuid>,
    student_id: StudentOnly,
    request_query: RequestType<ElectiveSubject, QueryablePlaceholder, SortablePlaceholder>,
    _: HaveApiKey,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = student_id.0;
    let elective_id = id.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    // Checks if the elective the student is trying to enroll in is available
    let elective =
        match ElectiveSubject::get_by_id(pool, elective_id, fetch_level, descendant_fetch_level)
            .await
        {
            Ok(ElectiveSubject::Detailed(elective, _)) => {
                if elective.class_size == elective.cap_size {
                    return Err(Error::InvalidPermission(
                        "The elective is already full".to_string(),
                        format!("/subjects/electives/{elective_id}/enroll"),
                    ));
                }

                elective
            }
            Err(Error::InternalSeverError(_, _)) => {
                return Err(Error::InvalidRequest(
                    "Elective subject not found".to_string(),
                    format!("/subjects/electives/{elective_id}/enroll"),
                ));
            }
            _ => unreachable!("ElectiveSubject::get_by_id should always return a Detailed variant"),
        };

    // Checks if the student is in a class available for the elective
    let student = Student::get_by_id(pool, student_id, Some(&FetchLevel::Default), None).await?;
    match student {
        Student::Default(student, _) => match student.classroom {
            None => {
                return Err(Error::InvalidPermission(
                    "Student is not in a class".to_string(),
                    format!("/subjects/electives/{elective_id}/enroll"),
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
                        format!("/subjects/electives/{elective_id}/enroll"),
                    ));
                }
            }
        },
        _ => unreachable!("Student::get_by_id should always return a Default variant"),
    }

    // Checks if the student has already enrolled in the elective before
    let enroll_count = query!(
        r#"
        SELECT COUNT(*) FROM student_elective_subjects
        WHERE student_id = $1 AND elective_subject_id = $2
        "#,
        student_id,
        elective_id
    )
    .fetch_one(pool)
    .await?;
    let enroll_count: i64 = enroll_count.count.unwrap_or(0);
    if enroll_count > 0 {
        return Err(Error::InvalidPermission(
            "Student has already enrolled in this elective before".to_string(),
            format!("/subjects/electives/{elective_id}/enroll"),
        ));
    }

    // Checks if the student is already enrolled in an elective this semester
    let has_enrolled = query!(
        r#"
        SELECT EXISTS (
            SELECT FROM student_elective_subjects
            WHERE student_id = $1 AND year = $2 AND semester = $3
        )
        "#,
        student_id,
        get_current_academic_year(None),
        get_current_semester(None),
    )
    .fetch_one(pool)
    .await?;
    let has_enrolled = has_enrolled.exists.unwrap_or(false);
    if has_enrolled {
        return Err(Error::InvalidPermission(
            "Student has already enrolled in an elective this semester".to_string(),
            format!("/subjects/electives/{elective_id}/enroll"),
        ));
    }

    query!(
        r#"
        INSERT INTO student_elective_subjects (
            student_id, elective_subject_id, year, semester
        ) VALUES ($1, $2, $3, $4)
        "#,
        student_id,
        elective_id,
        get_current_academic_year(None),
        get_current_semester(None),
    )
    .execute(pool)
    .await?;

    let response = ResponseType::new(elective, None);

    Ok(HttpResponse::Ok().json(response))
}
