use crate::models::{
    contact::db::DbContact,
    contact::fetch_levels::{default::DefaultContact, id_only::IdOnlyContact},
    model::Model,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Contact = Model<DbContact, IdOnlyContact, IdOnlyContact, DefaultContact, DefaultContact>;
