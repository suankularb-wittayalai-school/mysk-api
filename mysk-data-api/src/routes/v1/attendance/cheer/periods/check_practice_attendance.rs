use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json, Path},
};
use mysk_lib::{
    common::{requests::RequestType, response::ResponseType},
    models::{
        cheer_practice_attendance::{CheerPracticeAttendance, db::DbCheerPracticeAttendance},
        cheer_practice_period::db::DbCheerPracticePeriod,
        enums::{
            CheerPracticeAttendanceType,
            UserRole::{Student, Teacher},
        },
        student::db::DbStudent,
        teacher::db::DbTeacher,
    },
    permissions::Authorizer,
    prelude::*,
};
use serde::Deserialize;
use sqlx::{query, query_scalar};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CheckPracticeAttendanceRequest {
    is_start: bool,
    student_id: Uuid,
    presence: Option<CheerPracticeAttendanceType>,
    absence_reason: Option<String>,
}

#[allow(clippy::too_many_lines)]
#[post("/{id}/check")]
pub async fn check_practice_attendance(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    practice_period_id: Path<Uuid>,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<CheckPracticeAttendanceRequest>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut transaction = data.db.begin().await?;
    let practice_period_id = practice_period_id.into_inner();

    let authorizer = Authorizer::new(
        &mut transaction,
        &user,
        format!("/attendance/cheer/periods/{practice_period_id}/check"),
    )
    .await?;

    match user.role {
        Student => {
            let s_checker_id = DbStudent::get_student_from_user_id(&mut transaction, user.id)
                .await?
                .ok_or(Error::InvalidPermission(
                    "User is not a student".to_string(),
                    format!("/attendance/cheer/periods/{practice_period_id}/check"),
                ))?;

            // Check if the checking student is a cheer staff
            if !DbCheerPracticePeriod::get_cheer_staffs(&mut transaction)
                .await?
                .contains(&s_checker_id)
            {
                return Err(Error::InvalidPermission(
                    "Student must be a staff member to update attendances".to_string(),
                    format!("/attendance/cheer/periods/{practice_period_id}/check"),
                ));
            }

            if s_checker_id == request_data.student_id {
                return Err(Error::InternalServerError(
                    "Cheer staff cannot take their own cheer practice attendance".to_string(),
                    format!("/attendance/cheer/periods/{practice_period_id}/check"),
                ));
            }
        }
        Teacher => {
            let t_checker_id = DbTeacher::get_teacher_from_user_id(&mut transaction, user.id)
                .await?
                .ok_or(Error::InvalidPermission(
                    "User is not a teacher".to_string(),
                    format!("/attendance/cheer/periods/{practice_period_id}/check"),
                ))?;

            // Advisors can only take attendance of their own advisory classroom
            let is_classroom_valid = query_scalar!(
                "SELECT EXISTS (
                    SELECT FROM classroom_students cs
                    JOIN classroom_advisors ca ON cs.classroom_id = ca.classroom_id
                    WHERE cs.student_id = $1
                    AND ca.teacher_id = $2
                )",
                request_data.student_id,
                t_checker_id
            )
            .fetch_one(&mut *transaction)
            .await?
            .unwrap_or(false);
            if !is_classroom_valid {
                return Err(Error::InvalidPermission(
                    "Teacher is not the advisor of this classroom".to_string(),
                    format!("/attendance/cheer/periods/{practice_period_id}/check"),
                ));
            }
        }
        _ => {
            return Err(Error::InvalidPermission(
                "Logged in UserRole not permitted to perform this action".to_string(),
                format!("/attendance/cheer/periods/{practice_period_id}/check"),
            ));
        }
    }

    if let Some(presence) = request_data.presence {
        // Check if `absence_reason` matches with the correct `presence` enum
        if !matches!(
            presence,
            CheerPracticeAttendanceType::AbsentWithLeave
                | CheerPracticeAttendanceType::AbsentWithoutLeave
        ) && request_data.absence_reason.is_some()
        {
            return Err(Error::InvalidRequest(
                "Absence reason was specified for a presence type that forbids a reason"
                    .to_string(),
                format!("/attendance/cheer/periods/{practice_period_id}/check"),
            ));
        }

        // `presence_at_end` can only be Present or Deserted
        if !request_data.is_start
            && !matches!(
                presence,
                CheerPracticeAttendanceType::Present | CheerPracticeAttendanceType::Deserted
            )
        {
            return Err(Error::InvalidRequest(
                "Invalid presence type for the current attendance-taking phase".to_string(),
                format!("/attendance/cheer/periods/{practice_period_id}/check"),
            ));
        }
    }

    let is_blacklisted = query_scalar!(
        "SELECT EXISTS (
            SELECT FROM cheer_practice_blacklisted_students WHERE student_id = $1
        )",
        request_data.student_id
    )
    .fetch_one(&mut *transaction)
    .await?
    .unwrap_or(false);
    if is_blacklisted {
        return Err(Error::InvalidRequest(
            "Student is exempt from participating in cheer practice".to_string(),
            format!("/attendance/cheer/periods/{practice_period_id}/check"),
        ));
    }

    // Check if student is valid for period and classroom
    let is_student_id_valid = query_scalar!(
        "\
        SELECT EXISTS (\
            SELECT FROM classroom_students AS cs \
                JOIN cheer_practice_period_classrooms AS c ON c.classroom_id = cs.classroom_id \
            WHERE cs.student_id = $1 AND c.practice_period_id = $2\
        )\
        ",
        request_data.student_id,
        practice_period_id,
    )
    .fetch_one(&mut *transaction)
    .await?
    .unwrap_or(false);
    if !is_student_id_valid {
        return Err(Error::InvalidPermission(
            "Insufficient permissions to perform this action".to_string(),
            format!("/attendance/cheer/periods/{practice_period_id}/check"),
        ));
    }

    let practice_attendance_id = if request_data.is_start {
        let presence_at_end = request_data.presence.and_then(|presence| match presence {
            CheerPracticeAttendanceType::AbsentWithoutLeave
            | CheerPracticeAttendanceType::AbsentWithLeave
            | CheerPracticeAttendanceType::Deserted => Some(presence),
            _ => None,
        });

        query_scalar!(
            "\
            INSERT INTO cheer_practice_attendances\
                (practice_period_id, student_id, checker_id, presence, absence_reason, presence_at_end)\
                VALUES ($1, $2, $3, $4, $5, $6)\
            ON CONFLICT(practice_period_id, student_id)\
                DO UPDATE SET checker_id = $3, presence = $4, absence_reason = $5, presence_at_end = $6 \
            RETURNING id\
            ",
            practice_period_id,
            request_data.student_id,
            user.id,
            request_data.presence as Option<CheerPracticeAttendanceType>,
            request_data.absence_reason,
            presence_at_end as Option<CheerPracticeAttendanceType>
        )
        .fetch_one(&mut *transaction)
        .await?
    } else {
        let practice_attendance_id = DbCheerPracticeAttendance::get_by_period_id_and_student_id(
            &mut transaction,
            practice_period_id,
            request_data.student_id,
        )
        .await?;

        query!(
            "\
            UPDATE cheer_practice_attendances \
            SET checker_id = $1, presence_at_end = $2, absence_reason = $3 \
            WHERE id = $4\
            ",
            user.id,
            request_data.presence as Option<CheerPracticeAttendanceType>,
            request_data.absence_reason,
            practice_attendance_id,
        )
        .execute(&mut *transaction)
        .await?;

        practice_attendance_id
    };

    transaction.commit().await?;

    let practice_attendance = CheerPracticeAttendance::get_by_id(
        pool,
        practice_attendance_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(practice_attendance, None);

    Ok(HttpResponse::Ok().json(response))
}
