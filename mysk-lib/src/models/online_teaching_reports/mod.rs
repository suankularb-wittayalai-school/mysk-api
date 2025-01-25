use crate::models::{
    online_teaching_reports::{
        db::DbOnlineTeachingReports,
        fetch_levels::{
            default::DefaultOnlineTeachingReports, id_only::IdOnlyOnlineTeachingReports,
        },
        requests::{
            queryable::QueryableOnlineTeachingReports, sortable::SortableOnlineTeachingReports,
        },
    },
    top_level_variant::TopLevelVariant,
    traits::TopLevelQuery,
};

pub mod db;
pub mod fetch_levels;
pub mod requests;

pub type OnlineTeachingReports = TopLevelVariant<
    DbOnlineTeachingReports,
    IdOnlyOnlineTeachingReports,
    DefaultOnlineTeachingReports,
    DefaultOnlineTeachingReports,
    DefaultOnlineTeachingReports,
>;

impl
    TopLevelQuery<
        DbOnlineTeachingReports,
        QueryableOnlineTeachingReports,
        SortableOnlineTeachingReports,
    > for OnlineTeachingReports
{
}
