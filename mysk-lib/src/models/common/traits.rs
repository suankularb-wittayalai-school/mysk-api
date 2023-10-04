use async_trait::async_trait;
use sqlx::Error;

use super::requests::FetchLevel;

#[async_trait]
pub trait FetchLevelVariant<T> {
    async fn from_table(
        table: T,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}
