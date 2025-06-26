use crate::models::{
    student::{
        db::DbStudent,
        fetch_levels::{compact::CompactStudent, default::DefaultStudent, id_only::IdOnlyStudent},
    },
    model::Model,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Student = Model<
    DbStudent,
    IdOnlyStudent,
    CompactStudent,
    DefaultStudent,
    DefaultStudent,
>;
