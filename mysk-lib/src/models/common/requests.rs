use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FetchLevel {
    IdOnly,
    Compact,
    Default,
    Detailed,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct FilterConfig<T> {
    pub data: Option<T>,
    pub q: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct SortingConfig<T> {
    pub by: Vec<T>,
    pub ascending: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PaginationConfig {
    pub p: u32,
    pub size: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct RequestType<T, Queryable, Sortable> {
    pub data: Option<T>,
    pub pagination: Option<PaginationConfig>,
    pub filter: Option<FilterConfig<Queryable>>,
    pub sorting: Option<SortingConfig<Sortable>>,
    pub fetch_level: Option<FetchLevel>,
    pub descendant_fetch_level: Option<FetchLevel>,
}
