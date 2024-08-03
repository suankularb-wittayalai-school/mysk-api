use crate::models::contact::db::DbContact;
use fetch_levels::{default::DefaultContact, id_only::IdOnlyContact};

use super::{top_level_variant::TopLevelVariant, traits::TopLevelQuery};

pub mod db;
pub mod fetch_levels;

pub type Contact =
    TopLevelVariant<DbContact, IdOnlyContact, IdOnlyContact, DefaultContact, DefaultContact>;

// impl TopLevelQuery<DbContact, QueryablePlaceholder, SortablePlaceholder> for Contact {}

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Contact {
//     pub id: Uuid,
//     pub created_at: Option<DateTime<Utc>>,
//     pub name: Option<FlexibleMultiLangString>,
//     pub r#type: ContactType,
//     pub value: String,
//     pub include_students: Option<bool>,
//     pub include_teachers: Option<bool>,
//     pub include_parents: Option<bool>,
// }

// #[async_trait]
// impl TopLevelFromTable<DbContact> for Contact {
//     async fn from_table(
//         pool: &PgPool,
//         table: DbContact,
//         _: Option<&FetchLevel>,
//         _: Option<&FetchLevel>,
//         authorizer: &Box<dyn Authorizer>,
//     ) -> Result<Self> {
//         authorizer
//             .authorize_contact(&table, pool, ActionType::ReadDefault)
//             .await?;

//         Ok(Self {
//             id: table.id,
//             created_at: table.created_at,
//             name: match (table.name_th, table.name_en) {
//                 (Some(name_th), Some(name_en)) => Some(FlexibleMultiLangString {
//                     th: Some(name_th),
//                     en: Some(name_en),
//                 }),
//                 (Some(name_th), None) => Some(FlexibleMultiLangString {
//                     th: Some(name_th),
//                     en: None,
//                 }),
//                 (None, Some(name_en)) => Some(FlexibleMultiLangString {
//                     th: None,
//                     en: Some(name_en),
//                 }),
//                 (None, None) => None,
//             },
//             r#type: table.r#type,
//             value: table.value,
//             include_students: table.include_students,
//             include_teachers: table.include_teachers,
//             include_parents: table.include_parents,
//         })
//     }
// }

// #[async_trait]
// impl TopLevelGetById for Contact {
//     type Id = Uuid;

//     async fn get_by_id(
//         pool: &PgPool,
//         id: Self::Id,
//         fetch_level: Option<&FetchLevel>,
//         descendant_fetch_level: Option<&FetchLevel>,
//         authorizer: &Box<dyn Authorizer>,
//     ) -> Result<Self> {
//         let contact = DbContact::get_by_id(pool, id).await?;

//         Self::from_table(
//             pool,
//             contact,
//             fetch_level,
//             descendant_fetch_level,
//             authorizer,
//         )
//         .await
//     }

//     async fn get_by_ids(
//         pool: &PgPool,
//         ids: Vec<Self::Id>,
//         fetch_level: Option<&FetchLevel>,
//         descendant_fetch_level: Option<&FetchLevel>,
//         authorizer: &Box<dyn Authorizer>,
//     ) -> Result<Vec<Self>> {
//         let contacts = DbContact::get_by_ids(pool, ids).await?;
//         let fetch_level = fetch_level.copied();
//         let descendant_fetch_level = descendant_fetch_level.copied();
//         let futures: Vec<_> = contacts
//             .into_iter()
//             .map(|contact| {
//                 let pool = pool.clone();
//                 let authorizer = dyn_clone::clone_box(&**authorizer);

//                 tokio::spawn(async move {
//                     Self::from_table(
//                         &pool,
//                         contact,
//                         fetch_level.as_ref(),
//                         descendant_fetch_level.as_ref(),
//                         &authorizer,
//                     )
//                     .await
//                 })
//             })
//             .collect();

//         let mut result = Vec::with_capacity(futures.len());
//         for future in futures {
//             result.push(future.await??);
//         }

//         Ok(result)
//     }
// }
