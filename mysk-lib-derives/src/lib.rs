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

#[proc_macro_derive(GetById, attributes(get_by_id))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = GetByIdOpts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let expanded = match opts.table {
        None => quote! {
            impl GetById for #ident {
                async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> std::result::Result<Self, sqlx::Error> {
                    sqlx::query_as::<_, #ident>(format!("{} WHERE id = $1", Self::base_query()).as_str())
                        .bind(id)
                        .fetch_one(pool)
                        .await
                }

                async fn get_by_ids(pool: &sqlx::PgPool, id: Vec<Uuid>) -> std::result::Result<Vec<Self>, sqlx::Error> {
                    sqlx::query_as::<_, #ident>(format!("{} WHERE id = ANY($1)", Self::base_query()).as_str())
                        .bind(id)
                        .fetch_all(pool)
                        .await
                }
            }
        },
        Some(table) => quote! {
            impl GetById for #ident {
                async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> std::result::Result<Self, sqlx::Error> {
                    sqlx::query_as::<_, #ident>(format!("{} WHERE {}.id = $1", Self::base_query(), #table).as_str())
                        .bind(id)
                        .fetch_one(pool)
                        .await
                }

                async fn get_by_ids(pool: &sqlx::PgPool, id: Vec<Uuid>) -> std::result::Result<Vec<Self>, sqlx::Error> {
                    sqlx::query_as::<_, #ident>(format!("{} WHERE {}.id = ANY($1)", Self::base_query(), #table).as_str())
                        .bind(id)
                        .fetch_all(pool)
                        .await
                }
            }
        },
    };

    expanded.into()
}

#[proc_macro_derive(BaseQuery, attributes(base_query, count_query))]
pub fn derive_base_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = BaseQueryOpts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let query = opts.query;
    let count_query = opts.count_query;

    let expanded = quote! {
        impl BaseQuery for #ident {
            fn base_query() -> &'static str {
                #query
            }

            fn count_query() -> &'static str {
                #count_query
            }
        }
    };

    expanded.into()
}
