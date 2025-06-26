#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

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

#[proc_macro_derive(GetById, attributes(from_query))]
pub fn derive_from_query(input: TokenStream) -> TokenStream {
    derive::expand_from_query(input)
}
