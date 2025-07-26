use crate::models::{
    model::Model,
    subject_group::{db::DbSubjectGroup, fetch_levels::default::DefaultSubjectGroup},
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
