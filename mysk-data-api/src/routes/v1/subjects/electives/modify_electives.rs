use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
    AppState,
};
use actix_web::{
    put,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{FetchLevel, QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    helpers::date::{get_current_academic_year, get_current_semester},
    models::{
        classroom::Classroom, elective_subject::ElectiveSubject, student::Student,
        traits::TopLevelGetById as _,
    },
    prelude::*,
};
use sqlx::query;

#[put("/{session_code}/enroll")]
async fn modify_elective_subject(
    data: Data<AppState>,
    session_code: Path<i64>,
    student_id: LoggedInStudent,
    request_query: RequestType<ElectiveSubject, QueryablePlaceholder, SortablePlaceholder>,
    _api_key: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = student_id.0;
    let session_code = session_code.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    // Checks if the elective the student is trying to enroll in is available
    let elective = match ElectiveSubject::get_by_session_code(
        pool,
        session_code,
        Some(&FetchLevel::Detailed),
        None,
    )
    .await
    {
        Ok(ElectiveSubject::Detailed(elective, _)) => {
            if elective.class_size == elective.cap_size {
                return Err(Error::InvalidPermission(
                    "The elective is already full".to_string(),
                    format!("/subjects/electives/{session_code}/enroll"),
                ));
            }

            elective
        }
        Err(Error::InternalSeverError(_, _)) => {
            return Err(Error::InvalidRequest(
                "Elective subject not found".to_string(),
                format!("/subjects/electives/{session_code}/enroll"),
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
                    format!("/subjects/electives/{session_code}/enroll"),
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
                        "Student not in a class available for the elective".to_string(),
                        format!("/subjects/electives/{session_code}/enroll"),
                    ));
                }
            }
        },
        _ => unreachable!("Student::get_by_id should always return a Default variant"),
    }

    // Check if the student already has an elective subject if they don't then throw an error
    let student_elective_subject = query!(
        r#"
        SELECT elective_subject_id FROM student_elective_subjects WHERE student_id = $1 and year = $2 AND semester = $3
        "#,
        student_id,
        get_current_academic_year(None),
        get_current_semester(None),
    )
    .fetch_optional(pool)
    .await?;

    if student_elective_subject.is_none() {
        return Err(Error::InvalidPermission(
            "Student does not have an elective subject".to_string(),
            format!("/subjects/electives/{session_code}/enroll"),
        ));
    }

    // Checks if the student has already enrolled in the elective before
    let enroll_count = query!(
        r#"
        SELECT COUNT(*) FROM student_elective_subjects
        WHERE student_id = $1 AND elective_subject_id = $2
        "#,
        student_id,
        elective.id,
    )
    .fetch_one(pool)
    .await?;

    let enroll_count: i64 = enroll_count.count.unwrap_or(0);
    if enroll_count > 0 {
        return Err(Error::InvalidPermission(
            "Student has already enrolled in this elective before".to_string(),
            format!("/subjects/electives/{session_code}/enroll"),
        ));
    }

    query!(
        r#"
        UPDATE student_elective_subjects SET elective_subject_id = $1 WHERE student_id = $2 AND year = $3 AND semester = $4
        "#,
        elective.id,
        student_id,
        get_current_academic_year(None),
        get_current_semester(None),
    )
    .execute(pool)
    .await?;

    // Get the updated elective to return to the client
    let elective = ElectiveSubject::get_by_session_code(
        pool,
        session_code,
        fetch_level,
        descendant_fetch_level,
    )
    .await?;

    let response = ResponseType::new(elective, None);
    Ok(HttpResponse::Ok().json(response))
}
