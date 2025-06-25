use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Ident, Result as SynResult, Token, Type,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

fn parse_trailing_comma(input: ParseStream) -> SynResult<()> {
    if input.lookahead1().peek(Token![,]) {
        _ = input.parse::<Token![,]>()?;
    }

    Ok(())
}

struct ImplFetchLevelInput {
    table: Ident,
    fetch_level: Ident,
    fetch_variant: Type,
    row: Type,
}

impl Parse for ImplFetchLevelInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let table = input.call(Ident::parse_any)?;
        _ = input.parse::<Token![,]>()?;
        let fetch_level = input.call(Ident::parse_any)?;
        _ = input.parse::<Token![,]>()?;
        let fetch_variant = input.call(Type::parse)?;
        _ = input.parse::<Token![,]>()?;
        let row = input.call(Type::parse)?;
        parse_trailing_comma(input)?;

        Ok(Self {
            table,
            fetch_level,
            fetch_variant,
            row,
        })
    }
}

pub(crate) fn make_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ImplFetchLevelInput);
    let ImplFetchLevelInput {
        table,
        fetch_level,
        fetch_variant,
        row,
    } = input;
    let authorize_table = format_ident!("authorize_{}", table);
    let action_type = format_ident!("Read{}", fetch_level);

    let expanded = quote! {
        use crate::permissions::Authorizable as _;

        #[automatically_derived]
        impl crate::models::traits::FetchLevelVariant<#row> for #fetch_variant {
            async fn from_table(
                pool: &::sqlx::PgPool,
                table: #row,
                _: Option<crate::common::requests::FetchLevel>,
                authorizer: &crate::permissions::Authorizer,
            ) -> crate::prelude::Result<Self> {
                authorizer.#authorize_table(
                    &table,
                    &mut *(pool.acquire().await?),
                    crate::permissions::ActionType::#action_type,
                )
                .await?;

                Ok(Self::from(table))
            }
        }
    };

    expanded.into()
}

struct ImplIdOnlyInput {
    table: Ident,
    fetch_variant: Type,
    row: Type,
}

impl Parse for ImplIdOnlyInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let table = input.call(Ident::parse_any)?;
        _ = input.parse::<Token![,]>()?;
        let fetch_variant = input.call(Type::parse)?;
        _ = input.parse::<Token![,]>()?;
        let row = input.call(Type::parse)?;
        parse_trailing_comma(input)?;

        Ok(Self {
            table,
            fetch_variant,
            row,
        })
    }
}

pub(crate) fn make_from_id_only(input: TokenStream) -> TokenStream {
    let ImplIdOnlyInput {
        table,
        fetch_variant,
        row,
    } = parse_macro_input!(input as ImplIdOnlyInput);

    let expanded = quote! {
        impl From<#row> for #fetch_variant {
            fn from(row: #row) -> Self {
                Self { id: row.id }
            }
        }

        ::mysk_lib_macros::impl_fetch_level_variant_from!(
            #table,
            IdOnly,
            #fetch_variant,
            #row,
        );
    };

    expanded.into()
}
