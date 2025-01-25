//! Custom extractors that sometimes also functions as middlewares.

use futures::Future;
use mysk_lib::prelude::*;
use std::pin::Pin;

pub mod api_key;
pub mod logged_in;
pub mod student;
pub mod teacher;

/// Convenient `Future` type for extractors.
pub type ExtractorFuture<SelfT> = Pin<Box<dyn Future<Output = Result<SelfT>>>>;
