#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

extern crate paste;

use proc_macro::TokenStream;

mod derive;
mod fetch_level_variant;

#[proc_macro]
pub fn impl_fetch_level_variant_from(input: TokenStream) -> TokenStream {
    fetch_level_variant::make_from(input)
}

#[proc_macro]
pub fn impl_id_only_variant_from(input: TokenStream) -> TokenStream {
    fetch_level_variant::make_from_id_only(input)
}

#[proc_macro_derive(GetById, attributes(get_by_id))]
pub fn derive_get_by_id(input: TokenStream) -> TokenStream {
    derive::get_by_id(input)
}

#[proc_macro_derive(BaseQuery, attributes(base_query, count_query))]
pub fn derive_base_query(input: TokenStream) -> TokenStream {
    derive::base_query(input)
}
