use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(from_query))]
struct GetByIdOpts {
    relation: Option<String>,
    query: String,
    count_query: String,
}

pub(crate) fn expand_from_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let GetByIdOpts {
        relation,
        query,
        count_query,
    } = match GetByIdOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(err) => {
            return err.write_errors().into();
        }
    };
    let DeriveInput { ident, .. } = input;

    let query_one = if let Some(ref relation) = relation {
        quote! { concat!(#query, " WHERE ", #relation, ".id = $1") }
    } else {
        quote! { concat!(#query, " WHERE id = $1") }
    };

    let query_many = if let Some(ref relation) = relation {
        quote! { concat!(#query, " WHERE ", #relation, ".id = ANY($1)") }
    } else {
        quote! { concat!(#query, " WHERE id = ANY($1)") }
    };

    let expanded = quote! {
        #[automatically_derived]
        impl crate::models::traits::GetById for #ident {
            const BASE_QUERY: &str = #query;

            const COUNT_QUERY: &str = #count_query;

            async fn get_by_id<T>(
                conn: &mut ::sqlx::PgConnection,
                id: T,
            ) -> ::std::result::Result<Self, sqlx::Error>
            where
                T: for<'q> ::sqlx::Encode<'q, ::sqlx::Postgres> + ::sqlx::Type<::sqlx::Postgres>,
            {
                ::sqlx::query_as::<_, #ident>(#query_one)
                    .bind(id)
                    .fetch_one(&mut *conn)
                    .await
            }

            async fn get_by_ids<T>(
                conn: &mut ::sqlx::PgConnection,
                id: &[T],
            ) -> ::std::result::Result<Vec<Self>, sqlx::Error>
            where
                T: for<'q> ::sqlx::Encode<'q, ::sqlx::Postgres>
                    + ::sqlx::postgres::PgHasArrayType
                    + ::sqlx::Type<::sqlx::Postgres>,
            {
                ::sqlx::query_as::<_, #ident>(#query_many)
                    .bind(id)
                    .fetch_all(&mut *conn)
                    .await
            }
        }
    };

    expanded.into()
}
