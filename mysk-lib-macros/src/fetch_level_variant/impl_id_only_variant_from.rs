/// Implements a `IdOnly` fetch level variant for a data model. The arguments to this macro are as
/// follows:
///
/// `table_name`: The name of the table of the data model **in lowercase**.
///
/// `fetch_variant_type`: The type name of the fetch level variant.
///
/// `db_type`: The type name of the data model.
#[macro_export]
macro_rules! impl_id_only_variant_from {
    ($table_name: ident, $fetch_variant_type: ty, $db_type: ty) => {
        impl From<$db_type> for $fetch_variant_type {
            fn from(db_variant: $db_type) -> Self {
                Self { id: db_variant.id }
            }
        }

        ::mysk_lib_macros::impl_fetch_level_variant_from!(
            $table_name,
            IdOnly,
            $fetch_variant_type,
            $db_type
        );
    };
}
