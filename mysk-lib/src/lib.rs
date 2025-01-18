#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_without_default
)]

pub mod auth;
pub mod common;
pub mod error;
pub mod helpers;
pub mod models;
pub mod permissions;
pub mod prelude;
pub mod query;
