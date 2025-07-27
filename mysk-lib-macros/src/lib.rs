#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use proc_macro::TokenStream;

mod derive;
mod fetch_variant;

/// Implements a fetch variant for a base relation. Note that this macro is more of a "helper" macro
/// and fetch variants that require additional dependencies must be hand-written.
///
/// The signature of the macro being:
///
/// ```rust
/// struct RelationStruct {
///     id: Uuid,
///     created_at: DateTime<Utc>,
///     /* -- snip -- */
/// }
///
/// impl GetById for RelationStruct {
///     /* -- snip -- */
///
///     type Id = Uuid;
///
///     /* -- snip -- */
/// }
///
/// struct FetchVariantStruct {
///     id: Uuid,
///     /* -- snip -- */
/// }
///
/// impl_fetch_variant_from!(relation_ident, FetchLevel, FetchVariantStruct, RelationStruct);
/// ```
///
/// - `relation_ident`: The identifier (or name) of the relation as used throughout the codebase.
///   One example of usage is the `permissions` module.
/// - `FetchLevel`: One of `Compact`, `Default`, or `Detailed`.
/// - `FetchVariantStruct`: The target struct (fetch variant).
/// - `RelationStruct`: The struct (base relation) that the fetch variant is derived from which must
///   implement [`GetById`].
#[proc_macro]
pub fn impl_fetch_variant_from(input: TokenStream) -> TokenStream {
    fetch_variant::make_from(input)
}

/// Implements an `IdOnly` fetch variant for a base relation.
///
/// The signature of the macro being:
///
/// ```rust
/// struct RelationStruct {
///     id: Uuid,
///     created_at: DateTime<Utc>,
///     /* -- snip -- */
/// }
///
/// impl GetById for RelationStruct {
///     /* -- snip -- */
///
///     type Id = Uuid;
///
///     /* -- snip -- */
/// }
///
/// struct IdOnlyStruct {
///     id: Uuid,
/// }
///
/// impl_id_only_variant_from!(relation_ident, IdOnlyStruct, RelationStruct);
/// ```
///
/// - `relation_ident`: The identifier (or name) of the relation as used throughout the codebase.
///   One example of usage is the `permissions` module.
/// - `IdOnlyStruct`: The target struct (fetch variant) containing one field named `id`.
/// - `RelationStruct`: The struct (base relation) that the fetch variant is derived from which must
///   implement [`GetById`].
#[proc_macro]
pub fn impl_id_only_variant_from(input: TokenStream) -> TokenStream {
    fetch_variant::make_from_id_only(input)
}

#[proc_macro_derive(GetById, attributes(from_query))]
pub fn derive_from_query(input: TokenStream) -> TokenStream {
    derive::expand_from_query(input)
}
