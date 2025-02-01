use crate::models::{
    subject_group::{db::DbSubjectGroup, fetch_levels::default::DefaultSubjectGroup},
    top_level_variant::TopLevelVariant,
};

pub mod db;
pub mod fetch_levels;

pub type SubjectGroup = TopLevelVariant<
    DbSubjectGroup,
    DefaultSubjectGroup,
    DefaultSubjectGroup,
    DefaultSubjectGroup,
    DefaultSubjectGroup,
>;
