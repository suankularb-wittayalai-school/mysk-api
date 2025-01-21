use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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

    let expanded = match opts.table {
        None => quote! {
            use crate::models::traits::BaseQuery as _;

            #[automatically_derived]
            #[::async_trait::async_trait]
            impl crate::models::traits::GetById for #ident {
                async fn get_by_id<'c, A, T>(
                    conn: A,
                    id: T,
                ) -> ::std::result::Result<Self, sqlx::Error>
                where
                    A: sqlx::Acquire<'c, Database = sqlx::Postgres> + Send,
                    T: for<'q> ::sqlx::Encode<'q, ::sqlx::Postgres>
                        + ::sqlx::Type<::sqlx::Postgres>
                        + Send,
                {
                    let mut conn = conn.acquire().await?;

                    let query = format!("{} WHERE id = $1", <Self as crate::models::traits::BaseQuery>::base_query());
                    ::sqlx::query_as::<_, #ident>(&query)
                        .bind(id)
                        .fetch_one(&mut *conn)
                        .await
                }

                async fn get_by_ids<'c, A, T>(
                    conn: A,
                    id: Vec<T>,
                ) -> ::std::result::Result<Vec<Self>, sqlx::Error>
                where
                    A: ::sqlx::Acquire<'c, Database = ::sqlx::Postgres> + Send,
                    T: for<'q> ::sqlx::Encode<'q, ::sqlx::Postgres>
                        + ::sqlx::Type<::sqlx::Postgres>
                        + ::sqlx::postgres::PgHasArrayType
                        + Send,
                {
                    let mut conn = conn.acquire().await?;

                    let query = format!("{} WHERE id = ANY($1)", <Self as crate::models::traits::BaseQuery>::base_query());
                    ::sqlx::query_as::<_, #ident>(&query)
                        .bind(id)
                        .fetch_all(&mut *conn)
                        .await
                }
            }
        },
        Some(table) => quote! {
            use crate::models::traits::BaseQuery as _;

            #[automatically_derived]
            #[::async_trait::async_trait]
            impl crate::models::traits::GetById for #ident {
                async fn get_by_id<'c, A, T>(
                    conn: A,
                    id: T,
                ) -> ::std::result::Result<Self, sqlx::Error>
                where
                    A: ::sqlx::Acquire<'c, Database = ::sqlx::Postgres> + Send,
                    T: for<'q> ::sqlx::Encode<'q, ::sqlx::Postgres>
                        + ::sqlx::Type<::sqlx::Postgres>
                        + Send,
                {
                    let mut conn = conn.acquire().await?;

                    let query = format!("{} WHERE {}.id = $1", <Self as crate::models::traits::BaseQuery>::base_query(), #table);
                    ::sqlx::query_as::<_, #ident>(&query)
                        .bind(id)
                        .fetch_one(&mut *conn)
                        .await
                }

                async fn get_by_ids<'c, A, T>(
                    conn: A,
                    id: Vec<T>,
                ) -> ::std::result::Result<Vec<Self>, sqlx::Error>
                where
                    A: ::sqlx::Acquire<'c, Database = ::sqlx::Postgres> + Send,
                    T: for<'q> ::sqlx::Encode<'q, ::sqlx::Postgres>
                        + ::sqlx::Type<::sqlx::Postgres>
                        + ::sqlx::postgres::PgHasArrayType
                        + Send,
                {
                    let mut conn = conn.acquire().await?;

                    let query = format!("{} WHERE {}.id = ANY($1)", <Self as crate::models::traits::BaseQuery>::base_query(), #table);
                    ::sqlx::query_as::<_, #ident>(&query)
                        .bind(id)
                        .fetch_all(&mut *conn)
                        .await
                }
            }
        },
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
