use crate::models::{
    cheer_practice_attendance::{
        db::DbCheerPracticeAttendance,
        fetch_levels::{
            default::DefaultCheerPracticeAttendance, id_only::IdOnlyCheerPracticeAttendance,
        },
    },
    model::Model,
};

pub mod db;
pub mod fetch_levels;

pub type CheerPracticeAttendance = Model<
    DbCheerPracticeAttendance,
    IdOnlyCheerPracticeAttendance,
    DefaultCheerPracticeAttendance,
    DefaultCheerPracticeAttendance,
    DefaultCheerPracticeAttendance,
>;
