use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{
    common::{
        requests::{EmptyRequestData, FetchLevel, RequestType},
        response::{MetadataType, ResponseType},
    },
    models::cheer_practice_period::{
        CheerPracticePeriod,
        request::{queryable::QueryableCheerPracticePeriod, sortable::SortableCheerPracticePeriod},
    },
    permissions::Authorizer,
    prelude::*,
};

#[get("")]
pub async fn query_practice_period(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    RequestType {
        pagination,
        filter,
        sort,
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<EmptyRequestData, QueryableCheerPracticePeriod, SortableCheerPracticePeriod>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let authorizer =
        Authorizer::new(&mut conn, &user, "/attendance/cheer/periods".to_string()).await?;

    // TODO: Remove this later
    if !matches!(
        descendant_fetch_level,
        FetchLevel::IdOnly | FetchLevel::Compact
    ) {
        return Err(Error::InvalidRequest(
            "Requested descendant_fetch_level is too resource intensive. Use `IdOnly` or `Compact` instead."
                .to_string(),
            "/attendance/cheer/periods".to_string()
        ));
    }

    let (practice_periods, pagination) = CheerPracticePeriod::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(practice_periods, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
