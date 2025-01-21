use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    AppState,
};
use actix_web::{
    delete,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::{EmptyResponseData, ResponseType},
    },
    models::{contact::db::DbContact, traits::GetById as _},
    permissions::{self, ActionType},
    prelude::*,
};
use sqlx::query;
use std::sync::Arc;
use uuid::Uuid;

#[delete("")]
pub async fn delete_contacts(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    request_body: Json<RequestType<Vec<Uuid>, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let Some(contact_ids) = &request_body.data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            "/contacts".to_string(),
        ));
    };
    let authorizer = permissions::get_authorizer(pool, &user, "/contacts".to_string()).await?;

    // Check if the contacts exists
    let db_contacts = DbContact::get_by_ids(pool, contact_ids.clone()).await?;

    let authorizer = Arc::new(authorizer);
    let futures: Vec<_> = db_contacts
        .into_iter()
        .map(|db_contact| {
            let pool = pool.clone();
            let authorizer = authorizer.clone();

            tokio::spawn(async move {
                authorizer
                    .authorize_contact(&db_contact, &pool, ActionType::Delete)
                    .await
            })
        })
        .collect();
    for future in futures {
        future.await??;
    }

    query!("DELETE FROM contacts WHERE id = ANY($1)", contact_ids)
        .execute(pool)
        .await?;

    let response = ResponseType::new(EmptyResponseData {}, None);

    Ok(HttpResponse::Ok().json(response))
}
