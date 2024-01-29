pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;
pub use mysk_lib_macros::{impl_fetch_level_variant_from, impl_id_only_variant_from};
