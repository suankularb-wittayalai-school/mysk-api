#[macro_export]
macro_rules! impl_fetch_level_variant_from {
    ($fetch_variant_type: ty, $db_type: ty) => {
        use async_trait::async_trait as __async_trait;

        #[__async_trait]
        impl FetchLevelVariant<$db_type> for $fetch_variant_type {
            async fn from_table(
                _pool: &PgPool,
                table: $db_type,
                _descendant_fetch_level: Option<&FetchLevel>,
            ) -> Result<Self> {
                Ok(Self::from(table))
            }
        }
    };
}
