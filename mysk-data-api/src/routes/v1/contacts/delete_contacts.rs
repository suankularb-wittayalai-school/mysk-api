use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{
    HttpResponse, Responder, delete,
    web::{Data, Json},
};
use futures::future;
use mysk_lib::{
    common::{
        requests::RequestType,
        response::{EmptyResponseData, ResponseType},
    },
    models::{contact::db::DbContact, traits::GetById as _},
    permissions::{ActionType, Authorizable as _, Authorizer},
    prelude::*,
};
use sqlx::query;
use uuid::Uuid;

#[delete("")]
pub async fn delete_contacts(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    Json(RequestType {
        data: contact_ids, ..
    }): Json<RequestType<Vec<Uuid>>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let authorizer = Authorizer::new(&mut conn, &user, "/contacts".to_string()).await?;

    // Check if the contacts exists
    let db_contacts = DbContact::get_by_ids(pool, &contact_ids).await?;

    let futures = db_contacts
        .iter()
        .map(async |db_contact| {
            authorizer
                .authorize_contact(
                    db_contact,
                    &mut *(pool.acquire().await?),
                    ActionType::Delete,
                )
                .await
        })
        .collect::<Vec<_>>();
    future::try_join_all(futures).await?;

    query!("DELETE FROM contacts WHERE id = ANY($1)", &contact_ids[..])
        .execute(&mut *conn)
        .await?;

    let response = ResponseType::new(EmptyResponseData {}, None);

    Ok(HttpResponse::Ok().json(response))
}
