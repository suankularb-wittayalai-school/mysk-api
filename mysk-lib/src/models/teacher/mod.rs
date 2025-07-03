use crate::models::{
    model::Model,
    teacher::{
        db::DbTeacher,
        fetch_levels::{
            compact::CompactTeacher, default::DefaultTeacher, detailed::DetailedTeacher,
            id_only::IdOnlyTeacher,
        },
    },
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Teacher = Model<DbTeacher, IdOnlyTeacher, CompactTeacher, DefaultTeacher, DetailedTeacher>;
