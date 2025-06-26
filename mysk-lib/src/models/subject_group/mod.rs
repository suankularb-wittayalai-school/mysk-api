use crate::models::{
    subject_group::{db::DbSubjectGroup, fetch_levels::default::DefaultSubjectGroup},
    model::Model,
};

pub mod db;
pub mod fetch_levels;

pub type SubjectGroup = Model<
    DbSubjectGroup,
    DefaultSubjectGroup,
    DefaultSubjectGroup,
    DefaultSubjectGroup,
    DefaultSubjectGroup,
>;
