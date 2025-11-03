use crate::{AppState, extractors::api_key::ApiKeyHeader};
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
    permissions::{Authorizer, roles::AdminRole},
    prelude::*,
};

#[get("")]
pub async fn query_practice_periods(
    data: Data<AppState>,
    _: ApiKeyHeader,
    // LoggedIn(user): LoggedIn,
    RequestType {
        pagination,
        filter,
        sort,
        fetch_level,
        ..
    }: RequestType<EmptyRequestData, QueryableCheerPracticePeriod, SortableCheerPracticePeriod>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let authorizer = Authorizer::Admin(AdminRole);
    // let authorizer = Authorizer::new(
    //     &mut *(data.db.acquire().await?),
    //     &user,
    //     "/attendance/cheer/periods".to_string(),
    // )
    // .await?;

    let (practice_periods, pagination) = CheerPracticePeriod::query(
        pool,
        fetch_level,
        FetchLevel::IdOnly,
        filter,
        sort,
        pagination,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(practice_periods, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
