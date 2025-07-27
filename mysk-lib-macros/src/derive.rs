use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::{ToTokens, quote};
use syn::{DeriveInput, TypePath, parse_macro_input};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(from_query))]
struct GetByIdOpts {
    id: Option<TypePath>,
    relation: Option<String>,
    query: String,
    count_query: String,
}

pub(crate) fn expand_from_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let GetByIdOpts {
        id,
        relation,
        query,
        count_query,
    } = match GetByIdOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(err) => {
            return err.write_errors().into();
        }
    };
    let id = if let Some(id) = id {
        id.to_token_stream()
    } else {
        quote! { ::uuid::Uuid }
    };
    let DeriveInput { ident, .. } = input;

    let query_one = if let Some(ref relation) = relation {
        quote! { concat!(#query, " WHERE ", #relation, ".id = $1 ORDER BY ", #relation, ".id") }
    } else {
        quote! { concat!(#query, " WHERE id = $1 ORDER BY id") }
    };

    let query_many = if let Some(ref relation) = relation {
        quote! {
            concat!(#query, " WHERE ", #relation, ".id = ANY($1) ORDER BY ", #relation, ".id")
        }
    } else {
        quote! { concat!(#query, " WHERE id = ANY($1) ORDER BY id") }
    };

    let expanded = quote! {
        #[automatically_derived]
        impl crate::models::traits::GetById for #ident {
            const BASE_QUERY: &str = #query;

            const COUNT_QUERY: &str = #count_query;

            type Id = #id;

            async fn get_by_id(
                conn: &mut ::sqlx::PgConnection,
                id: Self::Id,
            ) -> ::std::result::Result<Self, sqlx::Error> {
                ::sqlx::query_as::<_, #ident>(#query_one)
                    .bind(id)
                    .fetch_one(&mut *conn)
                    .await
            }

            async fn get_by_ids(
                conn: &mut ::sqlx::PgConnection,
                id: &[Self::Id],
            ) -> ::std::result::Result<Vec<Self>, sqlx::Error> {
                ::sqlx::query_as::<_, #ident>(#query_many)
                    .bind(id)
                    .fetch_all(&mut *conn)
                    .await
            }
        }
    };

    expanded.into()
}
