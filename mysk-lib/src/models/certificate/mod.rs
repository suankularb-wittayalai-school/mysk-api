use crate::models::{
    certificate::{
        db::DbCertificate,
        fetch_levels::{default::DefaultCertificate, id_only::IdOnlyCertificate},
    },
    model::Model,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Certificate = Model<
    DbCertificate,
    IdOnlyCertificate,
    DefaultCertificate,
    DefaultCertificate,
    DefaultCertificate,
>;
