use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, student::LoggedInStudent},
};
use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};
use mysk_lib::{
    common::{requests::RequestType, response::ResponseType},
    models::{
        elective_subject::db::DbElectiveSubject, elective_trade_offer::ElectiveTradeOffer,
        enums::SubmissionStatus, traits::GetById,
    },
    permissions::Authorizer,
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
pub async fn create_trade_offer(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    LoggedInStudent(client_student_id): LoggedInStudent,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<ElectiveTradeOfferRequest>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut transaction = data.db.begin().await?;
    let other_student_id = request_data.receiver_id;
    let authorizer = Authorizer::new(&user, "/subjects/electives/trade-offers".to_string());

    // Checks if the student is "blacklisted" from enrolling in an elective
    if DbElectiveSubject::is_student_blacklisted(&mut transaction, client_student_id).await? {
        return Err(Error::InvalidPermission(
            "Student is blacklisted from enrolling in electives".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    }

    // Check if the current time is within the elective's enrollment period
    if !DbElectiveSubject::is_enrollment_period(&mut transaction, client_student_id).await? {
        return Err(Error::InvalidPermission(
            "The elective enrollment period has ended".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    }

    // Check if the sending student has already enrolled in an elective in the current semester
    let Some(client_elective_subject_id) =
        DbElectiveSubject::is_currently_enrolled(&mut transaction, client_student_id).await?
    else {
        return Err(Error::InvalidPermission(
            "Student has not enrolled in an elective this semester".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    };

    // Check if the receiving student has already enrolled in an elective in the current semester
    let Some(other_elective_subject_id) =
        DbElectiveSubject::is_currently_enrolled(&mut transaction, other_student_id).await?
    else {
        return Err(Error::InvalidPermission(
            "Receiving student has not enrolled in an elective this semester".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    };

    // Gets the elective subject of the receiver, and also checks whether they're in a classroom
    let other_elective_subject =
        DbElectiveSubject::get_by_id(&mut transaction, other_elective_subject_id).await?;

    // Checks if the sender is eligible to enroll in the receiver's elective session, also checks
    // whether they're in a classroom
    if !DbElectiveSubject::is_student_eligible(
        &mut transaction,
        other_elective_subject.id,
        client_student_id,
    )
    .await?
    {
        return Err(Error::InvalidPermission(
            "Student is not eligible to enroll in this elective".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    }

    // Gets the elective subject of the sender
    let client_elective_subject =
        DbElectiveSubject::get_by_id(&mut transaction, client_elective_subject_id).await?;

    // Checks if the receiver is eligible to enroll in the sender's elective session
    if !DbElectiveSubject::is_student_eligible(
        &mut transaction,
        client_elective_subject.id,
        other_student_id,
    )
    .await?
    {
        return Err(Error::InvalidPermission(
            "Receiving student is not eligible to enroll in this elective".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    }

    // Check if the elective subjects are the same
    if (client_elective_subject.id == other_elective_subject.id)
        && (client_elective_subject.session_code == other_elective_subject.session_code)
    {
        return Err(Error::InvalidRequest(
            "Both the sender and receiver has the same elective subjects".to_string(),
            "/subjects/electives/trade-offers".to_string(),
        ));
    }

    // Check if a trade offer with same receiving student and same elective subject already exists
    let trade_offer_already_exists = query!(
        "\
        SELECT EXISTS (\
            SELECT FROM elective_subject_trade_offers \
            WHERE sender_id = $1 AND receiver_id = $2 AND status = $3 \
            AND sender_elective_subject_session_id = $4 \
            AND receiver_elective_subject_session_id = $5\
        )\
        ",
        client_student_id,
        other_student_id,
        SubmissionStatus::Pending as SubmissionStatus,
        client_elective_subject.id,
        other_elective_subject.id,
    )
    .fetch_one(&mut *transaction)
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
        "\
        SELECT COUNT(*) FROM elective_subject_trade_offers \
        WHERE (sender_id = $1 OR receiver_id = $1) AND status = $2\
        ",
        client_student_id,
        SubmissionStatus::Pending as SubmissionStatus,
    )
    .fetch_one(&mut *transaction)
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
        "\
        SELECT COUNT(*) FROM elective_subject_trade_offers \
        WHERE (sender_id = $1 OR receiver_id = $1) AND status = $2\
        ",
        other_student_id,
        SubmissionStatus::Pending as SubmissionStatus,
    )
    .fetch_one(&mut *transaction)
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
        "\
        INSERT INTO elective_subject_trade_offers (\
            sender_id, receiver_id, status, sender_elective_subject_session_id,\
            receiver_elective_subject_session_id\
        ) VALUES ($1, $2, $3, $4, $5) RETURNING id\
        ",
        client_student_id,
        other_student_id,
        SubmissionStatus::Pending as SubmissionStatus,
        client_elective_subject.id,
        other_elective_subject.id,
    )
    .fetch_one(&mut *transaction)
    .await?
    .id;

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
