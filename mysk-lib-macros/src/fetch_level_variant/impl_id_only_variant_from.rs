#[macro_export]
macro_rules! impl_id_only_variant_from {
    ($fetch_variant_type: ty, $db_type: ty) => {
        use mysk_lib_macros::impl_fetch_level_variant_from as __mysk_macros_internal_impl_fetch_level_variant_from;

        impl From<$db_type> for $fetch_variant_type {
            fn from(db_variant: $db_type) -> Self {
                Self { id: db_variant.id }
            }
        }

        __mysk_macros_internal_impl_fetch_level_variant_from!($fetch_variant_type, $db_type);
    };
}
