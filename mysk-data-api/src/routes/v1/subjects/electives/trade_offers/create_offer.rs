use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
    AppState,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{FetchLevel, QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::{
        elective_subject::{db::DbElectiveSubject, ElectiveSubject},
        elective_trade_offer::ElectiveTradeOffer,
        enums::SubmissionStatus,
        traits::TopLevelGetById as _,
    },
    prelude::*,
};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ElectiveTradeOfferRequest {
    pub receiver_id: Uuid,
}

#[allow(clippy::too_many_lines)]
#[post("")]
async fn create_trade_offer(
    data: Data<AppState>,
    _: ApiKeyHeader,
    student_id: LoggedInStudent,
    request_body: Json<
        RequestType<ElectiveTradeOfferRequest, QueryablePlaceholder, SortablePlaceholder>,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let receiver_student_id = match &request_body.data {
        Some(request_data) => request_data.receiver_id,
        None => {
            return Err(Error::InvalidRequest(
                "Json deserialize error: field `data` can not be empty".to_string(),
                "/subjects/electives/trade-offers".to_string(),
            ));
        }
    };
    let sender_student_id = student_id.0;
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();

    // Checks if the student is "blacklisted" from enrolling in an elective
    if DbElectiveSubject::is_student_blacklisted(pool, sender_student_id).await? {
        return Err(Error::InvalidPermission(
            "Student is blacklisted from enrolling in electives".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    }

    // Check if the current time is within the elective's enrollment period
    if !DbElectiveSubject::is_enrollment_period(pool, sender_student_id).await? {
        return Err(Error::InvalidPermission(
            "The elective's enrollment period has ended".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    }

    // Check if the sending student has already enrolled in an elective in the current semester
    let Some(sender_elective_subject_id) =
        DbElectiveSubject::is_currently_enrolled(pool, sender_student_id).await?
    else {
        return Err(Error::InvalidPermission(
            "Student has not enrolled in an elective this semester".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    };

    // Check if the receiving student has already enrolled in an elective in the current semester
    let Some(receiver_elective_subject_id) =
        DbElectiveSubject::is_currently_enrolled(pool, receiver_student_id).await?
    else {
        return Err(Error::InvalidPermission(
            "Receiving student has not enrolled in an elective this semester".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    };

    // Gets the elective subject of the receiver, and also checks whether they're in a classroom
    let receiver_elective_subject = match ElectiveSubject::get_by_id(
        pool,
        receiver_elective_subject_id,
        Some(&FetchLevel::Compact),
        None,
    )
    .await
    {
        Ok(ElectiveSubject::Compact(receiver_elective_subject, _)) => receiver_elective_subject,
        Err(Error::InvalidPermission(err, _)) => {
            return Err(Error::InvalidPermission(
                err,
                "/subjects/electives/trade-offers".to_string(),
            ));
        }
        Err(Error::EntityNotFound(err, _)) => {
            return Err(Error::EntityNotFound(
                err,
                "/subjects/electives/trade-offers".to_string(),
            ));
        }
        Err(Error::InternalSeverError(err, _)) => {
            return Err(Error::InternalSeverError(
                err,
                "/subjects/electives/trade-offers".to_string(),
            ));
        }
        _ => unreachable!(),
    };

    // Checks if the sender is eligible to enroll in the receiver's elective session, also checks
    // whether they're in a classroom
    match DbElectiveSubject::is_student_eligible(
        pool,
        receiver_elective_subject.id,
        sender_student_id,
    )
    .await
    {
        Ok(true) => (),
        Ok(false) => {
            return Err(Error::InvalidPermission(
                "Student is not eligible to enroll in this elective".to_string(),
                "/subjects/electives/trade-offers".to_string(),
            ));
        }
        Err(Error::InvalidPermission(err, _)) => {
            return Err(Error::InvalidPermission(
                err,
                "/subjects/electives/trade-offers".to_string(),
            ));
        }
        _ => unreachable!(),
    };

    // Gets the elective subject of the sender
    let sender_elective_subject = match ElectiveSubject::get_by_id(
        pool,
        sender_elective_subject_id,
        Some(&FetchLevel::Compact),
        None,
    )
    .await
    {
        Ok(ElectiveSubject::Compact(sender_elective_subject, _)) => sender_elective_subject,
        Err(Error::EntityNotFound(err, _)) => {
            return Err(Error::EntityNotFound(
                err,
                "/subjects/electives/trade-offers".to_string(),
            ));
        }
        Err(Error::InternalSeverError(err, _)) => {
            return Err(Error::InternalSeverError(
                err,
                "/subjects/electives/trade-offers".to_string(),
            ));
        }
        _ => unreachable!(),
    };

    // Checks if the receiver is eligible to enroll in the sender's elective session
    match DbElectiveSubject::is_student_eligible(
        pool,
        sender_elective_subject.id,
        receiver_student_id,
    )
    .await
    {
        Ok(true) => (),
        Ok(false) => {
            return Err(Error::InvalidPermission(
                "Receiving student is not eligible to enroll in this elective".to_string(),
                "/subjects/electives/trade-offers".to_string(),
            ));
        }
        _ => unreachable!(),
    };

    // Check if the elective subjects are the same
    if (sender_elective_subject.id == receiver_elective_subject.id)
        && (sender_elective_subject.session_code == receiver_elective_subject.session_code)
    {
        return Err(Error::InvalidRequest(
            "Both the sender and receiver has the same elective subjects".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    }

    // Check if a trade offer with same receiving student and same elective subject already exists
    let trade_offer_already_exists = query!(
        "
        SELECT EXISTS (
            SELECT FROM elective_subject_trade_offers
            WHERE sender_id = $1 AND receiver_id = $2 AND status = $3
            AND sender_elective_subject_session_id = $4
            AND receiver_elective_subject_session_id = $5
        )
        ",
        sender_student_id,
        receiver_student_id,
        SubmissionStatus::Pending as SubmissionStatus,
        sender_elective_subject.id,
        receiver_elective_subject.id,
    )
    .fetch_one(pool)
    .await?
    .exists
    .unwrap_or(false);
    if trade_offer_already_exists {
        return Err(Error::InvalidRequest(
            "Trade offer with the receiving student already exists".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    }

    // check if sender have more than 3 pending trade offers
    let pending_trade_offers_count = query!(
        "
        SELECT COUNT(*) FROM elective_subject_trade_offers
        WHERE (sender_id = $1 OR receiver_id = $1) AND status = $2
        ",
        sender_student_id,
        SubmissionStatus::Pending as SubmissionStatus,
    )
    .fetch_one(pool)
    .await?
    .count
    .unwrap_or(0);

    if pending_trade_offers_count >= 3 {
        return Err(Error::InvalidPermission(
            "Student has reached the maximum number of pending trade offers".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    }

    // check if receiver have more than 3 pending trade offers
    let pending_trade_offers_count = query!(
        "
        SELECT COUNT(*) FROM elective_subject_trade_offers
        WHERE (sender_id = $1 OR receiver_id = $1) AND status = $2
        ",
        receiver_student_id,
        SubmissionStatus::Pending as SubmissionStatus,
    )
    .fetch_one(pool)
    .await?
    .count
    .unwrap_or(0);

    if pending_trade_offers_count >= 3 {
        return Err(Error::InvalidPermission(
            "Receiving student has reached the maximum number of pending trade offers".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    }

    let trade_offer_id = query!(
        "
        INSERT INTO elective_subject_trade_offers (
            sender_id,
            receiver_id,
            status,
            sender_elective_subject_session_id,
            receiver_elective_subject_session_id
        ) VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        ",
        sender_student_id,
        receiver_student_id,
        SubmissionStatus::Pending as SubmissionStatus,
        sender_elective_subject.id,
        receiver_elective_subject.id,
    )
    .fetch_one(pool)
    .await?
    .id;

    let elective_trade_offer =
        ElectiveTradeOffer::get_by_id(pool, trade_offer_id, fetch_level, descendant_fetch_level)
            .await?;
    let response = ResponseType::new(elective_trade_offer, None);

    Ok(HttpResponse::Ok().json(response))
}
