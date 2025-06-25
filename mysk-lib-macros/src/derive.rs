use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(base_query, count_query))]
struct BaseQueryOpts {
    query: String,
    count_query: String,
}

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(get_by_id))]
struct GetByIdOpts {
    table: Option<String>,
}

pub(crate) fn get_by_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = GetByIdOpts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;
    let base_query = quote! { <Self as crate::models::traits::BaseQuery>::base_query() };

    let query_one = if let Some(table) = &opts.table {
        quote! { "{} WHERE {}.id = $1", #base_query, #table }
    } else {
        quote! { "{} WHERE id = $1", #base_query }
    };

    let query_many = if let Some(table) = opts.table {
        quote! { "{} WHERE {}.id = ANY($1)", #base_query, #table }
    } else {
        quote! { "{} WHERE id = ANY($1)", #base_query }
    };

    let expanded = quote! {
        use crate::models::traits::BaseQuery as _;

        #[automatically_derived]
        impl crate::models::traits::GetById for #ident {
            async fn get_by_id<T>(
                conn: &mut ::sqlx::PgConnection,
                id: T,
            ) -> ::std::result::Result<Self, sqlx::Error>
            where
                T: for<'q> ::sqlx::Encode<'q, ::sqlx::Postgres> + ::sqlx::Type<::sqlx::Postgres>,
            {
                let query = format!(#query_one);
                ::sqlx::query_as::<_, #ident>(&query)
                    .bind(id)
                    .fetch_one(&mut *conn)
                    .await
            }

            async fn get_by_ids<T>(
                conn: &mut ::sqlx::PgConnection,
                id: Vec<T>,
            ) -> ::std::result::Result<Vec<Self>, sqlx::Error>
            where
                T: for<'q> ::sqlx::Encode<'q, ::sqlx::Postgres>
                    + ::sqlx::postgres::PgHasArrayType
                    + ::sqlx::Type<::sqlx::Postgres>,
            {
                let query = format!(#query_many);
                ::sqlx::query_as::<_, #ident>(&query)
                    .bind(id)
                    .fetch_all(&mut *conn)
                    .await
            }
        }
    };

    expanded.into()
}

pub(crate) fn base_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = BaseQueryOpts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let query = opts.query;
    let count_query = opts.count_query;

    let expanded = quote! {
        #[automatically_derived]
        impl crate::models::traits::BaseQuery for #ident {
            #[must_use]
            fn base_query() -> &'static str {
                #query
            }

            #[must_use]
            fn count_query() -> &'static str {
                #count_query
            }
        }
    };

    expanded.into()
}
