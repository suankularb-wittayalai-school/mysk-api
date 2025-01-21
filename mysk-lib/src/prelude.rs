pub use crate::{error::Error, impl_fetch_level_variant_from, impl_id_only_variant_from};

pub type Result<T, E = Error> = core::result::Result<T, E>;
