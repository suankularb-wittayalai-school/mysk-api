use crate::models::{
    model::Model,
    student::{
        db::DbStudent,
        fetch_levels::{compact::CompactStudent, default::DefaultStudent, id_only::IdOnlyStudent},
    },
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Student = Model<DbStudent, IdOnlyStudent, CompactStudent, DefaultStudent, DefaultStudent>;
