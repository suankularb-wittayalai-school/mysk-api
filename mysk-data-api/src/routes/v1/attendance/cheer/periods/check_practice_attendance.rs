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
        cheer_practice_attendance::CheerPracticeAttendance,
        cheer_practice_period::db::DbCheerPracticePeriod,
        enums::{
            CheerPracticeAttendanceType,
            UserRole::{self},
        },
        user::{User, UserMeta},
    },
    permissions::Authorizer,
    prelude::*,
};
use serde::Deserialize;
use sqlx::query_scalar;
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
        &user,
        format!("/attendance/cheer/periods/{practice_period_id}/check"),
    );

    match user {
        User {
            role: UserRole::Student,
            meta: Some(UserMeta::Student {
                student_id: s_checker_id,
            }),
            ..
        } => {
            if !DbCheerPracticePeriod::is_student_cheer_staff(&data.cache, s_checker_id) {
                return Err(Error::InvalidPermission(
                    "Student must be a staff member to update attendances".to_string(),
                    format!("/attendance/cheer/periods/{practice_period_id}/check"),
                ));
            }

            if s_checker_id == request_data.student_id {
                return Err(Error::InvalidPermission(
                    "Cheer staff cannot take their own cheer practice attendance".to_string(),
                    format!("/attendance/cheer/periods/{practice_period_id}/check"),
                ));
            }
        }
        User {
            role: UserRole::Teacher,
            meta: Some(UserMeta::Teacher {
                teacher_id: t_checker_id,
            }),
            ..
        } => {
            // Only teachers in `cheer_practice_teachers` can take attendance of any classroom
            // unless that day is Jaturamitr day
            if !DbCheerPracticePeriod::in_jaturamitr_period(practice_period_id)
                && !DbCheerPracticePeriod::is_teacher_cheer_staff(&data.cache, t_checker_id)
            {
                return Err(Error::InvalidPermission(
                    "Teacher is not allowed to take attendance on this period".to_string(),
                    format!("/attendance/cheer/periods/{practice_period_id}/check"),
                ));
            }
        }
        _ => {
            return Err(Error::InvalidPermission(
                "Insufficient permissions to perform this action".to_string(),
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

        // `presence_at_end` can only be either `present` or `deserted`
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

    // Check if student is valid for period and classroom
    let is_student_id_valid = query_scalar!(
        "\
        SELECT EXISTS (\
            SELECT FROM cheer_practice_attendances_with_detail_view AS cpa \
            WHERE cpa.student_id = $1 AND cpa.practice_period_id = $2 AND cpa.disabled = FALSE \
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

    let is_presence_unset = request_data.presence.is_none();

    // Upsert with coalescing if the presence given isn't null (not unset by the client)
    let practice_attendance_id = query_scalar!(
        "\
        INSERT INTO cheer_practice_attendances AS cpa (\
            practice_period_id, student_id, checker_id, presence, absence_reason\
        ) VALUES ($1, $2, $3, $4, $5)\
        ON CONFLICT(practice_period_id, student_id) DO UPDATE SET \
            checker_id = $3,\
            presence = CASE WHEN $7 THEN (\
                CASE WHEN $6 THEN NULL ELSE COALESCE($4, cpa.presence) END)\
                ELSE cpa.presence END,\
            absence_reason = $5,\
            presence_at_end = CASE WHEN (NOT $7) THEN (\
                CASE WHEN $6 THEN NULL ELSE COALESCE($4, cpa.presence_at_end) END)\
                ELSE cpa.presence_at_end END \
        RETURNING id\
        ",
        practice_period_id,
        request_data.student_id,
        user.id,
        request_data.presence as Option<CheerPracticeAttendanceType>,
        request_data.absence_reason,
        is_presence_unset,
        request_data.is_start,
    )
    .fetch_one(&mut *transaction)
    .await?;

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
