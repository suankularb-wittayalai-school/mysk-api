use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, student::LoggedInStudent},
};
use actix_web::{
    HttpResponse, Responder, put,
    web::{Data, Json, Path},
};
use mysk_lib::{
    common::{
        requests::RequestType,
        response::ResponseType,
    },
    helpers::date::{get_current_academic_year, get_current_semester},
    models::{
        elective_subject::db::DbElectiveSubject,
        elective_trade_offer::{ElectiveTradeOffer, db::DbElectiveTradeOffer},
        enums::SubmissionStatus,
        traits::GetById,
    },
    permissions::Authorizer,
    prelude::*,
};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct UpdatableElectiveOffer {
    pub status: SubmissionStatus,
}

#[allow(clippy::too_many_lines)]
#[put("/{id}")]
async fn update_trade_offer(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    LoggedInStudent(client_student_id): LoggedInStudent,
    trade_offer_id: Path<Uuid>,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<UpdatableElectiveOffer>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut transaction = data.db.begin().await?;
    let trade_offer_id = trade_offer_id.into_inner();
    let trade_offer_status = if matches!(request_data.status, SubmissionStatus::Pending) {
        return Err(Error::InvalidRequest(
            "Status must be either `approved` or `declined`".to_string(),
            format!("/subjects/electives/trade-offers/{trade_offer_id}"),
        ));
    } else {
        request_data.status
    };

    let authorizer = Authorizer::new(
        &mut transaction,
        &user,
        format!("/subjects/electives/trade-offers/{trade_offer_id}"),
    )
    .await?;

    // Checks if the student is "blacklisted" from enrolling in an elective
    if DbElectiveSubject::is_student_blacklisted(&mut transaction, client_student_id).await? {
        return Err(Error::InvalidPermission(
            "Student is blacklisted from enrolling in electives".to_string(),
            format!("/subjects/electives/trade-offers/{trade_offer_id}"),
        ));
    }

    // Check if the current time is within the elective's enrollment period
    if !DbElectiveSubject::is_enrollment_period(&mut transaction, client_student_id).await? {
        return Err(Error::InvalidPermission(
            "The elective enrollment period has ended".to_string(),
            format!("/subjects/electives/trade-offers/{trade_offer_id}"),
        ));
    }

    let trade_offer = DbElectiveTradeOffer::get_by_id(&mut transaction, trade_offer_id).await?;

    // Check if trade offer is already approved or declined
    if matches!(
        trade_offer.status,
        SubmissionStatus::Approved | SubmissionStatus::Declined,
    ) {
        return Err(Error::InvalidPermission(
            format!("Trade offer has already been {}", trade_offer.status),
            format!("/subjects/electives/trade-offers/{trade_offer_id}"),
        ));
    }

    let (updated_status, other_student_id) = if client_student_id == trade_offer.sender_id {
        // Disallow approved status if the client is a sending student
        if matches!(trade_offer_status, SubmissionStatus::Approved) {
            return Err(Error::InvalidPermission(
                "Student is not allowed to approve own trade offer".to_string(),
                format!("/subjects/electives/trade-offers/{trade_offer_id}"),
            ));
        }

        (trade_offer_status, trade_offer.receiver_id)
    } else if client_student_id == trade_offer.receiver_id {
        // Allows both approved and declined if the client is a receiving student
        (trade_offer_status, trade_offer.sender_id)
    } else {
        // Insufficient permissions if student is neither the sender or receiver
        return Err(Error::InvalidPermission(
            "Insufficient permissions to perform this action".to_string(),
            format!("/subjects/electives/trade-offers/{trade_offer_id}"),
        ));
    };

    if matches!(updated_status, SubmissionStatus::Approved) {
        // Set the status of all the other trade offers of the sending and receiving students to
        // "declined"
        query!(
            "\
            UPDATE elective_subject_trade_offers SET status = $1 \
            WHERE id != $2 AND status = $3 AND\
                (sender_id = $4 OR sender_id = $5 OR receiver_id = $4 OR receiver_id = $5)\
            ",
            SubmissionStatus::Declined as SubmissionStatus,
            trade_offer_id,
            SubmissionStatus::Pending as SubmissionStatus,
            client_student_id,
            other_student_id,
        )
        .execute(&mut *transaction)
        .await?;

        // Swap the elective subjects of the sending and receiving students
        // https://dba.stackexchange.com/a/131128
        query!(
            "\
            UPDATE elective_subject_session_enrolled_students AS esses \
            SET updated_at = now(), elective_subject_session_id = CASE student_id WHEN $1 THEN (\
                SELECT elective_subject_session_id \
                FROM elective_subject_session_enrolled_students AS esses \
                JOIN elective_subject_sessions AS ess \
                    ON ess.id = esses.elective_subject_session_id \
                WHERE student_id = $2 AND year = $3 AND semester = $4\
            ) WHEN $2 THEN (\
                SELECT elective_subject_session_id \
                FROM elective_subject_session_enrolled_students AS esses \
                JOIN elective_subject_sessions AS ess \
                    ON ess.id = esses.elective_subject_session_id \
                WHERE student_id = $1 AND year = $3 AND semester = $4\
            ) END FROM elective_subject_sessions AS ess \
            WHERE student_id IN ($1, $2) AND ess.id = esses.elective_subject_session_id \
            AND ess.year = $3 AND ess.semester = $4\
            ",
            client_student_id,
            other_student_id,
            get_current_academic_year(None),
            get_current_semester(None),
        )
        .execute(&mut *transaction)
        .await?;
    }

    // Accept or decline the trade offer
    query!(
        "UPDATE elective_subject_trade_offers SET status = $1 WHERE id = $2",
        updated_status as SubmissionStatus,
        trade_offer_id,
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    let elective_trade_offer = ElectiveTradeOffer::get_by_id(
        pool,
        trade_offer_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(elective_trade_offer, None);

    Ok(HttpResponse::Ok().json(response))
}
