#[macro_export]
macro_rules! impl_fetch_level_variant_from {
    ($fetch_variant_type: ty, $db_type: ty) => {
        use async_trait::async_trait as __mysk_macros_internal_async_trait;

        #[__mysk_macros_internal_async_trait]
        impl FetchLevelVariant<$db_type> for $fetch_variant_type {
            async fn from_table(
                _: &PgPool,
                table: $db_type,
                _: Option<&FetchLevel>,
                authorizer: &Box<dyn crate::permissions::Authorizer>,
            ) -> Result<Self> {
                Ok(Self::from(table))
            }
        }
    };
}
