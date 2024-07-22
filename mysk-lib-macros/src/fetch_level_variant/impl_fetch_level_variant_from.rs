/// Implements a fetch level variant for a data model. The arguments to this macro are as follows:
///
/// `table_name`: The name of the table of the data model **in lowercase**.
///
/// `fetch_level`: The name of the fetch level to be implemented **in pascal case**.
///
/// `fetch_variant_type`: The type name of the fetch level variant.
///
/// `db_type`: The type name of the data model.
#[macro_export]
macro_rules! impl_fetch_level_variant_from {
    ($table_name: ident, $fetch_level: ident, $fetch_variant_type: ty, $db_type: ty) => {
        use crate::permissions::ActionType;
        use async_trait::async_trait as __mysk_macros_internal_async_trait;

        #[__mysk_macros_internal_async_trait]
        impl FetchLevelVariant<$db_type> for $fetch_variant_type {
            async fn from_table(
                pool: &PgPool,
                table: $db_type,
                _: Option<&FetchLevel>,
                authorizer: &Box<dyn crate::permissions::Authorizer>,
            ) -> Result<Self> {
                $crate::paste::paste! {
                    authorizer
                        .[<authorize_ $table_name>](&table, pool, ActionType::[<Read $fetch_level>])
                        .await?;
                }

                Ok(Self::from(table))
            }
        }
    };
}
