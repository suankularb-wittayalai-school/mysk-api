use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
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
    },
    helpers::date::{get_current_academic_year, get_current_semester},
    models::{
        elective_subject::db::DbElectiveSubject,
        elective_trade_offer::{db::DbElectiveTradeOffer, ElectiveTradeOffer},
        enums::SubmissionStatus,
        traits::TopLevelGetById as _,
    },
    prelude::*,
};
use serde::Deserialize;
use sqlx::{query, query_as};
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
    student_id: LoggedInStudent,
    trade_offer_id: Path<Uuid>,
    request_body: Json<
        RequestType<UpdatableElectiveOffer, QueryablePlaceholder, SortablePlaceholder>,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let client_student_id = student_id.0;
    let trade_offer_id = trade_offer_id.into_inner();
    let trade_offer_status = match &request_body.data {
        Some(request_data) => match request_data.status {
            SubmissionStatus::Approved | SubmissionStatus::Declined => request_data.status,
            SubmissionStatus::Pending => {
                return Err(Error::InvalidRequest(
                    "Status must be either 'approved' or 'declined'".to_string(),
                    format!("/subjects/electives/trade-offers/{trade_offer_id}"),
                ));
            }
        },
        None => {
            return Err(Error::InvalidRequest(
                "Json deserialize error: field `data` can not be empty".to_string(),
                format!("/subjects/electives/trade-offers/{trade_offer_id}"),
            ));
        }
    };
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();

    // Checks if the student is "blacklisted" from enrolling in an elective
    if DbElectiveSubject::is_student_blacklisted(pool, client_student_id).await? {
        return Err(Error::InvalidPermission(
            "Student is blacklisted from enrolling in electives".to_string(),
            format!("/subjects/electives/trade-offers/{trade_offer_id}"),
        ));
    }

    // Check if the current time is within the elective's enrollment period
    if !DbElectiveSubject::is_enrollment_period(pool, client_student_id).await? {
        return Err(Error::InvalidPermission(
            "The elective's enrollment period has ended".to_string(),
            format!("/subjects/electives/trade-offers/{trade_offer_id}"),
        ));
    }

    let Some(trade_offer) = query_as!(
        DbElectiveTradeOffer,
        r#"
        SELECT
            id,
            created_at,
            sender_id,
            receiver_id,
            status AS "status: SubmissionStatus",
            sender_elective_subject_session_id,
            receiver_elective_subject_session_id
        FROM elective_subject_trade_offers WHERE id = $1
        "#,
        trade_offer_id,
    )
    .fetch_optional(pool)
    .await?
    else {
        return Err(Error::EntityNotFound(
            "Trade offer not found".to_string(),
            format!("/subjects/electives/trade-offers/{trade_offer_id}"),
        ));
    };

    // Check if trade offer is already approved or declined
    match trade_offer.status {
        SubmissionStatus::Approved | SubmissionStatus::Declined => {
            return Err(Error::InvalidPermission(
                format!("Trade offer has already been {}", trade_offer.status),
                format!("/subjects/electives/trade-offers/{trade_offer_id}"),
            ));
        }
        SubmissionStatus::Pending => (),
    }

    let mut updated_status: Option<SubmissionStatus> = None;
    let mut other_student_id: Option<Uuid> = None;

    if client_student_id == trade_offer.sender_id {
        // Runs if the client is a sending student
        match trade_offer_status {
            SubmissionStatus::Declined => {
                updated_status = Some(SubmissionStatus::Declined);
                other_student_id = Some(trade_offer.receiver_id);
            }
            SubmissionStatus::Approved => {
                return Err(Error::InvalidPermission(
                    "Student is not allowed to approve own trade offer".to_string(),
                    format!("/subjects/electives/trade-offers/{trade_offer_id}"),
                ));
            }
            SubmissionStatus::Pending => unreachable!(),
        }
    } else if client_student_id == trade_offer.receiver_id {
        // Runs if the client is a receiving student
        match trade_offer_status {
            SubmissionStatus::Approved => {
                updated_status = Some(SubmissionStatus::Approved);
                other_student_id = Some(trade_offer.sender_id);
            }
            SubmissionStatus::Declined => {
                updated_status = Some(SubmissionStatus::Declined);
                other_student_id = Some(trade_offer.sender_id);
            }
            SubmissionStatus::Pending => unreachable!(),
        }
    }

    // Checks if student is neither the sender or receiver
    if updated_status.is_none() || other_student_id.is_none() {
        return Err(Error::InvalidPermission(
            "Student is not allowed to interact with this trade offer".to_string(),
            format!("/subjects/electives/trade-offers/{trade_offer_id}"),
        ));
    }

    if let SubmissionStatus::Approved = updated_status.unwrap() {
        // Set the status of all the other trade offers of the sending and receiving students to
        // "declined"
        query!(
            "
            UPDATE elective_subject_trade_offers SET status = $1
            WHERE
                id != $2 AND status = $3 AND
                (sender_id = $4 OR sender_id = $5 OR receiver_id = $4 OR receiver_id = $5)
            ",
            SubmissionStatus::Declined as SubmissionStatus,
            trade_offer_id,
            SubmissionStatus::Pending as SubmissionStatus,
            client_student_id,
            other_student_id,
        )
        .execute(pool)
        .await?;

        // Swap the elective subjects of the sending and receiving students
        // https://dba.stackexchange.com/a/131128
        query!(
            "
            UPDATE elective_subject_session_enrolled_students
                SET updated_at = now(), elective_subject_session_id = CASE student_id
                    WHEN $1 THEN (
                        SELECT elective_subject_session_id
                        FROM
                            elective_subject_session_enrolled_students AS esses
                            JOIN elective_subject_sessions AS ess
                            ON ess.id = esses.elective_subject_session_id
                        WHERE student_id = $2 AND year = $3 AND semester = $4
                    )
                    WHEN $2 THEN (
                        SELECT elective_subject_session_id
                        FROM
                            elective_subject_session_enrolled_students AS esses
                            JOIN elective_subject_sessions AS ess
                            ON ess.id = esses.elective_subject_session_id
                        WHERE student_id = $1 AND year = $3 AND semester = $4
                    )
                END
            WHERE student_id IN ($1, $2)
            ",
            client_student_id,
            other_student_id,
            get_current_academic_year(None),
            get_current_semester(None),
        )
        .execute(pool)
        .await?;
    }

    // Accept or decline the trade offer
    query!(
        "UPDATE elective_subject_trade_offers SET status = $1 WHERE id = $2",
        updated_status.unwrap() as SubmissionStatus,
        trade_offer_id,
    )
    .execute(pool)
    .await?;

    let elective_trade_offer =
        ElectiveTradeOffer::get_by_id(pool, trade_offer_id, fetch_level, descendant_fetch_level)
            .await?;
    let response = ResponseType::new(elective_trade_offer, None);

    Ok(HttpResponse::Ok().json(response))
}
