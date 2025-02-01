use crate::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};

#[derive(Clone, Debug, Deserialize)]
pub struct PaginationConfig {
    pub p: u32,
    pub size: Option<u32>,
}

impl Default for PaginationConfig {
    fn default() -> Self {
        Self {
            p: 1,
            size: Some(50),
        }
    }
}

impl PaginationConfig {
    pub fn new(p: u32, size: Option<u32>) -> Self {
        Self { p, size }
    }

    pub fn append_into_query_builder(&self, qb: &mut QueryBuilder<'_, Postgres>) -> Result<()> {
        if self.p == 0 {
            return Err(Error::InvalidRequest(
                "Page number must be greater than zero".to_string(),
                "QueryDb::query".to_string(),
            ));
        }

        qb.push(" LIMIT ")
            .push_bind(i64::from(self.size.unwrap_or(50)))
            .push(" OFFSET ")
            .push_bind(i64::from((self.p - 1) * self.size.unwrap_or(50)));

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct PaginationType {
    first_p: u32,
    last_p: u32,
    next_p: Option<u32>,
    prev_p: Option<u32>,
    size: u32,
    total: u32,
}

impl PaginationType {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn new(current_p: u32, size: u32, total: u32) -> Self {
        let page_count = (f64::from(total) / f64::from(size)).ceil() as u32;

        PaginationType {
            first_p: 1,
            last_p: page_count,
            next_p: if current_p < page_count {
                Some(current_p + 1)
            } else {
                None
            },
            prev_p: if current_p > 1 {
                Some(current_p - 1)
            } else {
                None
            },
            size,
            total,
        }
    }
}
