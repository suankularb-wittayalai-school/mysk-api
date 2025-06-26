use crate::models::{
    model::Model,
    online_teaching_reports::{
        db::DbOnlineTeachingReports,
        fetch_levels::{
            default::DefaultOnlineTeachingReports, id_only::IdOnlyOnlineTeachingReports,
        },
    },
};

pub mod db;
pub mod fetch_levels;
pub mod requests;

pub type OnlineTeachingReports = Model<
    DbOnlineTeachingReports,
    IdOnlyOnlineTeachingReports,
    DefaultOnlineTeachingReports,
    DefaultOnlineTeachingReports,
    DefaultOnlineTeachingReports,
>;
