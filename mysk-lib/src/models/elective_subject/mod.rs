use self::{
    db::DbElectiveSubject,
    fetch_levels::{
        compact::CompactElectiveSubject, default::DefaultElectiveSubject,
        detailed::DetailedElectiveSubject, id_only::IdOnlyElectiveSubject,
    },
    request::{queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject},
};
use super::common::{
    requests::{FetchLevel, FilterConfig, PaginationConfig, SortingConfig},
    response::PaginationType,
    top_level_variant::TopLevelVariant,
    traits::{QueryDb, TopLevelQuery},
};
use crate::models::common::traits::TopLevelFromTable;
use crate::prelude::*;

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type ElectiveSubject = TopLevelVariant<
    DbElectiveSubject,
    IdOnlyElectiveSubject,
    CompactElectiveSubject,
    DefaultElectiveSubject,
    DetailedElectiveSubject,
>;

impl TopLevelQuery<QueryableElectiveSubject, SortableElectiveSubject> for ElectiveSubject {
    async fn query(
        pool: &sqlx::PgPool,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
        filter: Option<&FilterConfig<QueryableElectiveSubject>>,
        sort: Option<&SortingConfig<SortableElectiveSubject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>> {
        let models = DbElectiveSubject::query(pool, filter, sort, pagination).await?;

        let mut result = vec![];

        for variant in models {
            result
                .push(Self::from_table(pool, variant, fetch_level, descendant_fetch_level).await?);
        }

        Ok(result)
    }

    async fn response_pagination(
        pool: &sqlx::PgPool,
        filter: Option<&FilterConfig<QueryableElectiveSubject>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType> {
        DbElectiveSubject::response_pagination(pool, filter, pagination).await
    }
}
