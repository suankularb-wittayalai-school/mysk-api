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
    relation_ident: Ident,
    fetch_level: Ident,
    fetch_variant: Type,
    relation_ty: Type,
}

impl Parse for ImplFetchLevelInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let relation_ident = input.call(Ident::parse_any)?;
        _ = input.parse::<Token![,]>()?;
        let fetch_level = input.call(Ident::parse_any)?;
        _ = input.parse::<Token![,]>()?;
        let fetch_variant = input.call(Type::parse)?;
        _ = input.parse::<Token![,]>()?;
        let relation_ty = input.call(Type::parse)?;
        parse_trailing_comma(input)?;

        Ok(Self {
            relation_ident,
            fetch_level,
            fetch_variant,
            relation_ty,
        })
    }
}

pub(crate) fn make_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ImplFetchLevelInput);
    let ImplFetchLevelInput {
        relation_ident,
        fetch_level,
        fetch_variant,
        relation_ty,
    } = input;
    let authorize_relation_ident = format_ident!("authorize_{}", relation_ident);
    let action_type = format_ident!("Read{}", fetch_level);

    let expanded = quote! {
        #[automatically_derived]
        impl crate::models::traits::FetchVariant for #fetch_variant {
            type Relation = #relation_ty;

            async fn from_relation(
                pool: &::sqlx::PgPool,
                relation: #relation_ty,
                descendant_fetch_level: crate::common::requests::FetchLevel,
                authorizer: &crate::permissions::Authorizer,
            ) -> crate::prelude::Result<Self> {
                <crate::permissions::Authorizer as crate::permissions::Authorizable>
                ::#authorize_relation_ident(
                    authorizer,
                    &relation,
                    &mut *(pool.acquire().await?),
                    crate::permissions::ActionType::#action_type,
                )
                .await?;

                Ok(Self::from(relation))
            }
        }
    };

    expanded.into()
}

struct ImplIdOnlyInput {
    relation_ident: Ident,
    fetch_variant: Type,
    relation_ty: Type,
}

impl Parse for ImplIdOnlyInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let relation_ident = input.call(Ident::parse_any)?;
        _ = input.parse::<Token![,]>()?;
        let fetch_variant = input.call(Type::parse)?;
        _ = input.parse::<Token![,]>()?;
        let relation_ty = input.call(Type::parse)?;
        parse_trailing_comma(input)?;

        Ok(Self {
            relation_ident,
            fetch_variant,
            relation_ty,
        })
    }
}

pub(crate) fn make_from_id_only(input: TokenStream) -> TokenStream {
    let ImplIdOnlyInput {
        relation_ident,
        fetch_variant,
        relation_ty,
    } = parse_macro_input!(input as ImplIdOnlyInput);

    let expanded = quote! {
        impl From<#relation_ty> for #fetch_variant {
            fn from(relation_ty: #relation_ty) -> Self {
                Self { id: relation_ty.id }
            }
        }

        ::mysk_lib_macros::impl_fetch_variant_from!(
            #relation_ident,
            IdOnly,
            #fetch_variant,
            #relation_ty,
        );
    };

    expanded.into()
}
