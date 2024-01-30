#[macro_export]
macro_rules! impl_id_only_variant_from {
    ($fetch_variant_type: ty, $db_type: ty) => {
        impl From<$db_type> for $fetch_variant_type {
            fn from(db_variant: $db_type) -> Self {
                Self { id: db_variant.id }
            }
        }

        impl_fetch_level_variant_from!($fetch_variant_type, $db_type);
    };
}
