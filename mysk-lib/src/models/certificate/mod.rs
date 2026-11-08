use crate::models::{
    certificate::{
        db::DbCertificate, 
        fetch_levels::{
            default::DefaultCertificate
        },
    }, 
    model::Model,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Certificate = Model<DbCertificate, DefaultCertificate, DefaultCertificate, DefaultCertificate, DefaultCertificate>;