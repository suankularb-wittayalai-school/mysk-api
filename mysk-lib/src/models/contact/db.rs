use crate::{
    common::requests::FilterConfig,
    models::{
        contact::request::{queryable::QueryableContact, sortable::SortableContact},
        enums::ContactType,
        traits::QueryRelation,
    },
    query::Queryable as _,
};
use chrono::{DateTime, Utc};
use mysk_lib_macros::GetById;
use serde::Deserialize;
use sqlx::{FromRow, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, FromRow, GetById)]
#[from_query(
    query = "SELECT id, created_at, name_th, name_en, type, value FROM contacts",
    count_query = "SELECT COUNT(*) FROM contacts"
)]
pub struct DbContact {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: Option<String>,
    pub name_en: Option<String>,
    pub r#type: ContactType,
    pub value: String,
}

impl QueryRelation for DbContact {
    type Q = QueryableContact;
    type S = SortableContact;

    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<QueryableContact>>,
    ) {
        if let Some(filter) = filter {
            if let Some(data) = filter.data {
                data.to_where_clause()
                    .append_into_query_builder(query_builder);
            }
        }
    }
}
