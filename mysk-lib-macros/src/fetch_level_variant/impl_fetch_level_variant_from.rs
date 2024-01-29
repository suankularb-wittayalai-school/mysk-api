#[macro_export]
macro_rules! impl_fetch_level_variant_from {
    ($fetch_variant_type: ty, $db_type: ty) => {
        impl FetchLevelVariant<$db_type> for $fetch_variant_type {
            async fn from_table(
                _pool: &PgPool,
                table: $db_type,
                _descendant_fetch_level: Option<&FetchLevel>,
            ) -> Result<Self, sqlx::Error> {
                Ok(Self::from(table))
            }
        }
    };
}

#[macro_export]
macro_rules! id_only_variant_boiler_plate {
    ($fetch_variant_type: ty, $db_type: ty) => {
        impl From<$db_type> for $fetch_variant_type {
            fn from(db_variant: $db_type) -> Self {
                Self { id: db_variant.id }
            }
        }

        impl_fetch_level_variant_from!($fetch_variant_type, $db_type);
    };
}
