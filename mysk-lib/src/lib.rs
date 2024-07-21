#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::borrowed_box,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

pub mod auth;
pub mod common;
pub mod error;
pub mod helpers;
pub mod models;
pub mod permissions;
pub mod prelude;
