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
        requests::{RequestType, SortablePlaceholder},
        response::{EmptyResponseData, ResponseType},
    },
    models::{contact::db::DbContact, traits::GetById as _},
    permissions::{ActionType, Authorizable as _, Authorizer},
    prelude::*,
    query::QueryablePlaceholder,
};
use sqlx::query;
use uuid::Uuid;

#[delete("")]
pub async fn delete_contacts(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    Json(RequestType {
        data: request_data, ..
    }): Json<RequestType<Vec<Uuid>, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let Some(contact_ids) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            "/contacts".to_string(),
        ));
    };
    let authorizer = Authorizer::new(&mut conn, &user, "/contacts".to_string()).await?;

    // Check if the contacts exists
    let db_contacts = DbContact::get_by_ids(&mut conn, contact_ids.clone()).await?;

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
